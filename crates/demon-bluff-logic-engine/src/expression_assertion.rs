use std::{
	arch::breakpoint,
	collections::{HashMap, HashSet},
	fmt::Display,
	hash::Hash,
};

use demon_bluff_gameplay_engine::Expression;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::optimized_expression::OptimizedExpression;

pub fn collect_satisfying_assignments<T>(expression: &OptimizedExpression<T>) -> Vec<Vec<bool>>
where
	T: Display + Hash + Eq + Clone + Send + Sync,
{
	// Generate all possible assignments (2^n where n is number of variables)
	let num_vars = expression.variables().len();
	let total_potential_assignments: u64 = 1 << num_vars;

	if total_potential_assignments > u32::MAX as u64 {
		panic!(
			"We should never have more than 2G potential assignments: {}",
			total_potential_assignments
		);
	}

	let assignments = (0..total_potential_assignments)
		.into_par_iter()
		.filter_map(|i| {
			let mut assignment = Vec::new();

			for (j, _) in expression.variables().iter().enumerate() {
				assignment.push((i & (1 << j)) != 0);
			}

			// Check if this assignment satisfies the expression
			if expression.satisfies(|variable_index| assignment[variable_index]) {
				Some(assignment)
			} else {
				None
			}
		})
		.collect();

	assignments
}

fn collect_variables<T>(expression: &Expression<T>) -> HashSet<T>
where
	T: Display + Hash + Eq + Clone + Send + Sync,
{
	let mut vars = HashSet::new();
	collect_variables_helper(expression, &mut vars);
	vars
}

fn collect_variables_helper<T>(expression: &Expression<T>, vars: &mut HashSet<T>)
where
	T: Display + Hash + Eq + Clone,
{
	match expression {
		Expression::Leaf(testimony) => {
			vars.insert(testimony.clone());
		}
		Expression::Not(inner) => {
			collect_variables_helper(inner, vars);
		}
		Expression::And(left, right) | Expression::Or(left, right) => {
			collect_variables_helper(left, vars);
			collect_variables_helper(right, vars);
		}
		Expression::MajorOr(expressions) => {
			for expression in expressions {
				collect_variables_helper(expression, vars);
			}
		}
	}
}

/// Evaluate the expression with a given variable assignment
pub fn evaluate_with_assignment<T>(
	expression: &Expression<T>,
	assignment: &HashMap<T, bool>,
) -> bool
where
	T: Display + Hash + Eq + Clone,
{
	match expression {
		Expression::Leaf(testimony) => *assignment
			.get(testimony)
			.unwrap_or_else(|| panic!("Missing assignment for testimony: {}", testimony)),
		Expression::Not(inner) => !evaluate_with_assignment(&inner, assignment),
		Expression::And(lhs, rhs) => {
			evaluate_with_assignment(lhs, assignment) && evaluate_with_assignment(rhs, assignment)
		}
		Expression::Or(lhs, rhs) => {
			evaluate_with_assignment(lhs, assignment) || evaluate_with_assignment(rhs, assignment)
		}
		Expression::MajorOr(expressions) => {
			for expression in expressions {
				if evaluate_with_assignment(expression, assignment) {
					return true;
				}
			}

			false
		}
	}
}
