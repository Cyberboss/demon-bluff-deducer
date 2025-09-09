use core::slice;
use std::{
	alloc::{Layout, alloc, dealloc},
	mem::MaybeUninit,
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

		let rightmost_used_index = optimized_expression.build_expression(expression);
		assert_eq!(optimized_expression.clauses.len() - 1, rightmost_used_index);

		optimized_expression
	}

	fn count_clauses_and_gather_variables(root: &Expression<T>, variables: &mut Vec<T>) -> usize {
		let mut remaining_visit_stack = Vec::new();
		remaining_visit_stack.push(root);

		let mut count = 0;
		while let Some(expression) = remaining_visit_stack.pop() {
			count += 1;
			match expression {
				Expression::Leaf(variable) => {
					if None
						== variables
							.iter()
							.position(|existing_variable| existing_variable == variable)
					{
						variables.push(variable.clone());
					}
				}
				Expression::Not(expression) => {
					remaining_visit_stack.push(expression);
				}
				Expression::And(lhs, rhs) | Expression::Or(lhs, rhs) => {
					remaining_visit_stack.push(rhs);
					remaining_visit_stack.push(lhs);
				}
			}
		}

		count
	}

	pub fn satisfies<F>(&self, mut get_assignment: F) -> bool
	where
		F: FnMut(usize) -> bool + Copy,
	{
		#[derive(Clone, Copy)]
		enum Frame {
			Eval(usize),  // Need to evaluate clause at index
			NotOp,        // Apply NOT to last value
			AndOp(usize), // After evaluating lhs, evaluate rhs at index
			OrOp(usize),  // After evaluating lhs, evaluate rhs at index
		}

		let expected_max_len = std::cmp::max(4, self.clauses.len() / 2);
		let mut visit_stack = Vec::with_capacity(expected_max_len);
		let mut results_stack = Vec::with_capacity(expected_max_len);

		visit_stack.push(Frame::Eval(0));

		while let Some(frame) = visit_stack.pop() {
			match frame {
				Frame::Eval(clause_index) => match &self.clauses[clause_index] {
					ExpressionClause::Variable(variable_index) => {
						results_stack.push(get_assignment(*variable_index))
					}
					ExpressionClause::Not => {
						visit_stack.push(Frame::NotOp);
						visit_stack.push(Frame::Eval(clause_index + 1));
					}
					ExpressionClause::And(rhs_clause_index) => {
						visit_stack.push(Frame::AndOp(*rhs_clause_index));
						visit_stack.push(Frame::Eval(clause_index + 1));
					}
					ExpressionClause::Or(rhs_clause_index) => {
						visit_stack.push(Frame::OrOp(*rhs_clause_index));
						visit_stack.push(Frame::Eval(clause_index + 1));
					}
				},
				Frame::NotOp => {
					let result = !results_stack.pop().unwrap();
					results_stack.push(result)
				}
				Frame::AndOp(rhs_clause_index) => {
					let lhs_result = results_stack.pop().unwrap();
					if lhs_result {
						visit_stack.push(Frame::Eval(rhs_clause_index));
					} else {
						results_stack.push(false);
					}
				}
				Frame::OrOp(rhs_clause_index) => {
					let lhs_result = results_stack.pop().unwrap();
					if lhs_result {
						results_stack.push(true);
					} else {
						visit_stack.push(Frame::Eval(rhs_clause_index));
					}
				}
			}
		}

		debug_assert_eq!(results_stack.capacity(), expected_max_len);
		debug_assert_eq!(visit_stack.capacity(), expected_max_len);
		assert_eq!(results_stack.len(), 1);
		results_stack.pop().unwrap()
	}

	fn build_expression(&mut self, root: &Expression<T>) -> usize {
		let expected_max_len = std::cmp::max(4, self.clauses.len() / 2);
		let mut visit_stack = Vec::with_capacity(expected_max_len);

		let root_index = 0;
		visit_stack.push((root, root_index, false));

		let mut last_used_index = root_index;

		while let Some((expression, our_expression_index, visited)) = visit_stack.pop() {
			match (expression, visited) {
				(Expression::Leaf(variable), _) => {
					self.clauses[our_expression_index] = ExpressionClause::Variable(
						self.variables
							.iter()
							.position(|existing_variable| variable == existing_variable)
							.expect("Variable was not registered!"),
					);

					last_used_index = our_expression_index
				}
				(Expression::Not(inner), _) => {
					self.clauses[our_expression_index] = ExpressionClause::Not;
					visit_stack.push((inner, our_expression_index + 1, false));
				}
				(Expression::And(lhs, _) | Expression::Or(lhs, _), false) => {
					visit_stack.push((expression, our_expression_index, true));
					visit_stack.push((lhs, our_expression_index + 1, false));
				}
				(Expression::And(_, rhs), true) => {
					visit_stack.push((rhs, last_used_index + 1, false));
					self.clauses[our_expression_index] = ExpressionClause::And(last_used_index);
				}
				(Expression::Or(_, rhs), true) => {
					visit_stack.push((rhs, last_used_index + 1, false));
					self.clauses[our_expression_index] = ExpressionClause::Or(last_used_index);
				}
			}
		}

		debug_assert_eq!(visit_stack.capacity(), expected_max_len);

		last_used_index
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
