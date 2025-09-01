use std::collections::{HashMap, HashSet};

use demon_bluff_gameplay_engine::{Expression, testimony::Testimony};

use crate::engine::{HypothesisResult, and_result, not_result, sum_result};

pub fn probability_expression_asserts_x_given_true(
	expression: &Expression<Testimony>,
	leaf_x_assertion_probabilities: &HashMap<Testimony, HypothesisResult>,
) -> HypothesisResult {
	// First, collect all the "satisfying assignments" (ways the expression can be true)
	let assignments = collect_satisfying_assignments(expression);

	if assignments.is_empty() {
		return HypothesisResult::impossible();
	}

	// Calculate the probability that each assignment asserts X
	let mut total_prob = HypothesisResult::impossible();
	let num_assignments = assignments.len() as f64;

	for assignment in assignments {
		let prob_asserts_x =
			assignment_asserts_x(expression, &assignment, leaf_x_assertion_probabilities);
		total_prob = sum_result(total_prob, prob_asserts_x);
	}

	// Return the average probability across all satisfying assignments
	total_prob.map(|fitness_and_action| {
		fitness_and_action.map_action(|fitness| fitness / num_assignments)
	})
}

fn collect_satisfying_assignments(
	expression: &Expression<Testimony>,
) -> Vec<HashMap<Testimony, bool>> {
	let variables = collect_variables(expression);
	let mut assignments = Vec::new();

	// Generate all possible assignments (2^n where n is number of variables)
	let num_vars = variables.len();
	for i in 0..(1 << num_vars) {
		let mut assignment = HashMap::new();

		for (j, var) in variables.iter().enumerate() {
			assignment.insert(var.clone(), (i & (1 << j)) != 0);
		}

		// Check if this assignment satisfies the expression
		if evaluate_with_assignment(expression, &assignment) {
			assignments.push(assignment);
		}
	}

	assignments
}

fn collect_variables(expression: &Expression<Testimony>) -> HashSet<Testimony> {
	let mut vars = HashSet::new();
	collect_variables_helper(expression, &mut vars);
	vars
}

fn collect_variables_helper(expression: &Expression<Testimony>, vars: &mut HashSet<Testimony>) {
	match expression {
		Expression::Leaf(testimony) => {
			vars.insert(testimony.clone());
		}
		Expression::And(left, right) | Expression::Or(left, right) => {
			collect_variables_helper(left, vars);
			collect_variables_helper(right, vars);
		}
	}
}

/// Evaluate the expression with a given variable assignment
fn evaluate_with_assignment(
	expression: &Expression<Testimony>,
	assignment: &HashMap<Testimony, bool>,
) -> bool {
	match expression {
		Expression::Leaf(testimony) => *assignment
			.get(testimony)
			.unwrap_or_else(|| panic!("Missing assignment for testiomony: {}", testimony)),
		Expression::And(lhs, rhs) => {
			evaluate_with_assignment(lhs, assignment) && evaluate_with_assignment(rhs, assignment)
		}
		Expression::Or(lhs, rhs) => {
			evaluate_with_assignment(lhs, assignment) || evaluate_with_assignment(rhs, assignment)
		}
	}
}

/// Calculate the probability that a given assignment asserts X
fn assignment_asserts_x(
	expression: &Expression<Testimony>,
	assignment: &HashMap<Testimony, bool>,
	leaf_probabilities: &HashMap<Testimony, HypothesisResult>,
) -> HypothesisResult {
	match expression {
		Expression::Leaf(testimony) => {
			let is_true = assignment
				.get(testimony)
				.copied()
				.unwrap_or_else(|| panic!("Missing assignment for testiomony: {}", testimony));
			if is_true {
				leaf_probabilities
					.get(testimony)
					.unwrap_or_else(|| panic!("Probability not found for leaf: {}", testimony))
					.clone()
			} else {
				HypothesisResult::impossible() // TODO: Is this right??? False statements don't assert X
			}
		}
		Expression::And(left, right) => {
			// If A AND B is true, it asserts X if at least one operand asserts X
			let left_result = assignment_asserts_x(left, assignment, leaf_probabilities);
			let right_result = assignment_asserts_x(right, assignment, leaf_probabilities);

			// P(at least one asserts X) = 1 - P(neither asserts X)
			and_result(left_result, right_result)
		}
		Expression::Or(left, right) => {
			// For OR, we need to be more careful. If both sides are true in this assignment,
			// we use the same logic as AND. If only one side is true, we use that side's probability.
			let left_true = evaluate_with_assignment(left, assignment);
			let right_true = evaluate_with_assignment(right, assignment);

			match (left_true, right_true) {
				(true, false) => assignment_asserts_x(left, assignment, leaf_probabilities),
				(false, true) => assignment_asserts_x(right, assignment, leaf_probabilities),
				(true, true) => {
					// Both are true, use "at least one asserts X" logic
					let left_prob = assignment_asserts_x(left, assignment, leaf_probabilities);
					let right_prob = assignment_asserts_x(right, assignment, leaf_probabilities);
					not_result(and_result(not_result(left_prob), not_result(right_prob)))
				}
				(false, false) => HypothesisResult::impossible(), // This shouldn't happen if assignment satisfies the expression
			}
		}
	}
}

#[cfg(test)]
mod test {
	use demon_bluff_gameplay_engine::villager::VillagerIndex;

	use super::*;
	use crate::engine::FitnessAndAction;

	#[test]
	fn test_expression_probability_1() {
		let mut results = HashMap::new();
		let t1 = Testimony::Evil(VillagerIndex(4));
		let t2 = Testimony::Evil(VillagerIndex(5));
		results.insert(
			t1.clone(),
			HypothesisResult::Conclusive(FitnessAndAction::certainty(None)),
		);
		results.insert(
			t2.clone(),
			HypothesisResult::Conclusive(FitnessAndAction::certainty(None)),
		);
		let result = probability_expression_asserts_x_given_true(
			&Expression::<Testimony>::Or(
				Box::new(Expression::Leaf(t1)),
				Box::new(Expression::Leaf(t2)),
			),
			&results,
		);

		assert_eq!(1.0, result.fitness_and_action().fitness());
	}

	#[test]
	fn test_expression_probability_2() {
		let mut results = HashMap::new();
		let t1 = Testimony::Evil(VillagerIndex(4));
		let t2 = Testimony::Evil(VillagerIndex(5));
		let t3 = Testimony::Evil(VillagerIndex(6));
		results.insert(
			t1.clone(),
			HypothesisResult::Conclusive(FitnessAndAction::certainty(None)),
		);
		results.insert(
			t2.clone(),
			HypothesisResult::Conclusive(FitnessAndAction::impossible()),
		);
		results.insert(
			t3.clone(),
			HypothesisResult::Conclusive(FitnessAndAction::impossible()),
		);
		let result = probability_expression_asserts_x_given_true(
			&Expression::Or(
				Box::new(Expression::Or(
					Box::new(Expression::Leaf(t1)),
					Box::new(Expression::Leaf(t2)),
				)),
				Box::new(Expression::Leaf(t3)),
			),
			&results,
		);

		// think truth table
		assert_eq!(4.0 / 7.0, result.fitness_and_action().fitness());
	}
}
