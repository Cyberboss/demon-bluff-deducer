use core::slice;
use std::{
	alloc::{Layout, alloc, dealloc},
	mem::{ManuallyDrop, MaybeUninit, transmute},
	ptr::drop_in_place,
};

use demon_bluff_gameplay_engine::Expression;

pub struct OptimizedExpression<'a, T>
where
	T: Eq + Clone,
{
	raw_allocation: *mut u8,
	variables: &'a mut [T],
	clauses: &'a mut [ExpressionClause],
}

enum ExpressionClause {
	Variable(usize),
	Not,
	And(usize),
	Or(usize),
}

impl<'a, T> OptimizedExpression<'a, T>
where
	T: Eq + Clone,
{
	fn get_layout(num_variables: usize, num_clauses: usize) -> Layout {
		let alignment = align_of::<T>();
		let clause_alignment = align_of::<ExpressionClause>();
		assert_eq!(alignment, clause_alignment);

		let variables_size = num_variables * size_of::<T>();
		let size = variables_size + (num_clauses * size_of::<ExpressionClause>());

		Layout::from_size_align(size, alignment).expect("Could not create layout!")
	}

	pub fn new(expression: &Expression<T>) -> Self {
		let mut variables_builder = Vec::new();
		let clause_count =
			Self::count_clauses_and_gather_variables(expression, &mut variables_builder);

		// allocate a dual-contiguous pair of vectors for variables and clauses
		let alignment = align_of::<T>();
		let clause_alignment = align_of::<ExpressionClause>();
		assert_eq!(alignment, clause_alignment);

		let variables_size = variables_builder.len() * size_of::<T>();

		let layout = Self::get_layout(variables_builder.len(), clause_count);

		let mut optimized_expression;
		unsafe {
			let raw_allocation = alloc(layout);
			let variables_allocation = raw_allocation as *mut MaybeUninit<T>;

			let raw_expression_clause_allocation = raw_allocation.add(variables_size);
			let clauses_allocation =
				raw_expression_clause_allocation as *mut MaybeUninit<ExpressionClause>;

			let maybe_uninit_variables =
				slice::from_raw_parts_mut(variables_allocation, variables_builder.len());
			maybe_uninit_variables.write_iter(variables_builder.into_iter());
			let variables = maybe_uninit_variables.assume_init_mut();

			let maybe_uninit_clauses = slice::from_raw_parts_mut(clauses_allocation, clause_count);
			maybe_uninit_clauses
				.write_iter((0..clause_count).map(|_| ExpressionClause::Variable(0)));
			let clauses = maybe_uninit_clauses.assume_init_mut();

			// both variables and clauses are now valid
			optimized_expression = OptimizedExpression {
				raw_allocation,
				variables,
				clauses,
			};

			// allocation is now safely stored in optimized_expression and will be properly dropped
		}

		let rightmost_used_index = optimized_expression.build_expression(expression, 0);
		assert_eq!(optimized_expression.clauses.len() - 1, rightmost_used_index);

		optimized_expression
	}

	fn count_clauses_and_gather_variables(
		expression: &Expression<T>,
		mut variables: &mut Vec<T>,
	) -> usize {
		1 + match expression {
			Expression::Leaf(variable) => {
				if None
					== variables
						.iter()
						.position(|existing_variable| existing_variable == variable)
				{
					variables.push(variable.clone());
				}

				0
			}
			Expression::Not(expression) => {
				Self::count_clauses_and_gather_variables(expression, &mut variables)
			}
			Expression::And(lhs, rhs) | Expression::Or(lhs, rhs) => {
				Self::count_clauses_and_gather_variables(lhs, &mut variables)
					+ Self::count_clauses_and_gather_variables(rhs, &mut variables)
			}
		}
	}

	pub fn satisfies<F>(&self, get_assignment: F) -> bool
	where
		F: FnMut(usize) -> bool + Copy,
	{
		self.clause_satisied(0, get_assignment)
	}

	fn clause_satisied<F>(&self, clause_index: usize, mut get_assignment: F) -> bool
	where
		F: FnMut(usize) -> bool + Copy,
	{
		let next_clause_index = clause_index + 1;
		match &self.clauses[clause_index] {
			ExpressionClause::Variable(variable_index) => get_assignment(*variable_index),
			ExpressionClause::Not => !self.clause_satisied(next_clause_index, get_assignment),
			ExpressionClause::And(rhs) => {
				self.clause_satisied(next_clause_index, get_assignment)
					&& self.clause_satisied(*rhs, get_assignment)
			}
			ExpressionClause::Or(rhs) => {
				self.clause_satisied(next_clause_index, get_assignment)
					|| self.clause_satisied(*rhs, get_assignment)
			}
		}
	}

	fn build_expression(
		&mut self,
		expression: &Expression<T>,
		our_expression_index: usize,
	) -> usize {
		let (clause, rightmost_used_index) = match expression {
			Expression::Leaf(variable) => (
				ExpressionClause::Variable(
					self.variables
						.iter()
						.position(|existing_variable| variable == existing_variable)
						.expect("Variable was not registered!"),
				),
				our_expression_index,
			),
			Expression::Not(expression) => {
				let not_index = our_expression_index + 1;
				let rightmost_used_index = self.build_expression(expression, not_index);
				(ExpressionClause::Not, rightmost_used_index)
			}
			Expression::And(lhs, rhs) => {
				let left_index = our_expression_index + 1;
				let left_rightmost_used_index = self.build_expression(lhs, left_index);
				let right_index = left_rightmost_used_index + 1;
				let rightmost_used_index = self.build_expression(rhs, right_index);
				(ExpressionClause::And(right_index), rightmost_used_index)
			}
			Expression::Or(lhs, rhs) => {
				let left_index = our_expression_index + 1;
				let left_rightmost_used_index = self.build_expression(lhs, left_index);
				let right_index = left_rightmost_used_index + 1;
				let rightmost_used_index = self.build_expression(rhs, right_index);
				(ExpressionClause::Or(right_index), rightmost_used_index)
			}
		};

		self.clauses[our_expression_index] = clause;
		rightmost_used_index
	}

	pub fn variables(&self) -> &[T] {
		&self.variables
	}
}

unsafe impl<'a, T> Send for OptimizedExpression<'a, T> where T: Eq + Clone {}
unsafe impl<'a, T> Sync for OptimizedExpression<'a, T> where T: Eq + Clone {}

impl<'a, T> Drop for OptimizedExpression<'a, T>
where
	T: Eq + Clone,
{
	fn drop(&mut self) {
		let num_variables = self.variables.len();
		let num_clauses = self.clauses.len();
		unsafe {
			std::ptr::drop_in_place(std::ptr::slice_from_raw_parts_mut(
				self.clauses.as_mut_ptr(),
				num_clauses,
			));
			std::ptr::drop_in_place(std::ptr::slice_from_raw_parts_mut(
				self.variables.as_mut_ptr(),
				num_variables,
			));

			dealloc(
				self.raw_allocation,
				Self::get_layout(num_variables, num_clauses),
			);
		}
	}
}

#[test]
fn test_some_expression() {
	let term_a = "a".to_owned();
	let term_b = "b".to_owned();
	let term_c = "c".to_owned();

	let expression = Expression::And(
		Box::new(Expression::Or(
			Box::new(Expression::Not(Box::new(Expression::Leaf(term_a.clone())))),
			Box::new(Expression::Not(Box::new(Expression::Leaf(term_b)))),
		)),
		Box::new(Expression::Or(
			Box::new(Expression::Not(Box::new(Expression::Leaf(term_c)))),
			Box::new(Expression::Not(Box::new(Expression::Leaf(term_a)))),
		)),
	);

	let optimized_expression = OptimizedExpression::new(&expression);
	drop(optimized_expression);
}
