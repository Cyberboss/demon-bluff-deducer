use std::{
	collections::VecDeque,
	fmt::{Display, Formatter},
};

use super::{HypothesisReference, IndexReference};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Cycle {
	/// This is the order from the RE-VISITED reference not repeating it
	order_from_root: Vec<HypothesisReference>,
}

impl Cycle {
	fn new(order_from_root: Vec<HypothesisReference>) -> Self {
		Self { order_from_root }
	}

	pub fn references(&self) -> &Vec<HypothesisReference> {
		&self.order_from_root
	}
}

impl Clone for Cycle {
	fn clone(&self) -> Self {
		let mut order_from_root = Vec::with_capacity(self.order_from_root.len());
		for trace_reference in &self.order_from_root {
			order_from_root.push(trace_reference.clone());
		}
		Self { order_from_root }
	}
}

impl Display for Cycle {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let mut first = true;
		for reference in &self.order_from_root {
			if first {
				first = false;
			} else {
				write!(f, " -> ")?;
			}

			write!(f, "{reference}")?;
		}

		Ok(())
	}
}

pub fn derive_from_full_cycle(
	full_cycle: &Cycle,
	reference_stack: &Vec<HypothesisReference>,
	attempted_reference_visit: &HypothesisReference,
) -> Cycle {
	let mut order_from_root =
		VecDeque::with_capacity(reference_stack.len() + full_cycle.order_from_root.len() - 1);
	for visited_reference in reference_stack {
		if visited_reference == attempted_reference_visit {
			break;
		}

		order_from_root.push_back(visited_reference.clone());
	}

	let mut adding: bool = false;
	let mut found_end = false;
	for full_cycle_reference in &full_cycle.order_from_root {
		if adding {
			if order_from_root.contains(full_cycle_reference) {
				while order_from_root[0] != *full_cycle_reference {
					order_from_root.pop_front();
				}

				found_end = true;
				break;
			}

			order_from_root.push_back(full_cycle_reference.clone());
		} else if full_cycle_reference == attempted_reference_visit {
			order_from_root.push_back(full_cycle_reference.clone());
			adding = true;
		}
	}

	if !found_end {
		// reference stack enters the cycle past where it starts
		let after_start_before_end = order_from_root
			.iter()
			.last()
			.expect("Pretty sure we checked this already")
			.clone();

		let mut adding = false;
		for full_cycle_reference in &full_cycle.order_from_root {
			if *full_cycle_reference == after_start_before_end {
				adding = true;
			} else if adding {
				if order_from_root.contains(full_cycle_reference) {
					while order_from_root[0] != *full_cycle_reference {
						order_from_root.pop_front();
					}

					found_end = true;
					break;
				}

				order_from_root.push_back(full_cycle_reference.clone());
			}
		}

		if !found_end {
			panic!("No idea man");
		}
	}

	Cycle::new(order_from_root.into())
}

pub fn clone_cycle(cycle: &Cycle) -> Cycle {
	Cycle::new(
		cycle
			.order_from_root
			.iter()
			.map(|reference| reference.clone())
			.collect(),
	)
}

pub fn new_cycle(order_from_root: Vec<HypothesisReference>) -> Cycle {
	Cycle::new(order_from_root)
}

#[test]
fn test_derive_from_full_cycle_1() {
	let full_cycle = Cycle::new(vec![
		HypothesisReference::new(0),
		HypothesisReference::new(1),
		HypothesisReference::new(3),
		HypothesisReference::new(2),
		HypothesisReference::new(3),
	]);

	let reference_stack = vec![
		HypothesisReference::new(0),
		HypothesisReference::new(2),
		HypothesisReference::new(3),
	];

	let attempted_reference_visit = HypothesisReference::new(2);

	let result = derive_from_full_cycle(&full_cycle, &reference_stack, &attempted_reference_visit);

	assert_eq!(2, result.order_from_root.len());
	assert_eq!(HypothesisReference::new(2), result.order_from_root[0]);
	assert_eq!(HypothesisReference::new(3), result.order_from_root[1]);
}

#[test]
fn test_derive_from_full_cycle_2() {
	let full_cycle = Cycle::new(vec![
		HypothesisReference::new(0),
		HypothesisReference::new(1),
		HypothesisReference::new(3),
		HypothesisReference::new(4),
		HypothesisReference::new(3),
	]);

	let reference_stack = vec![HypothesisReference::new(0), HypothesisReference::new(2)];

	let attempted_reference_visit = HypothesisReference::new(3);

	let result = derive_from_full_cycle(&full_cycle, &reference_stack, &attempted_reference_visit);

	assert_eq!(2, result.order_from_root.len());
	assert_eq!(HypothesisReference::new(3), result.order_from_root[0]);
	assert_eq!(HypothesisReference::new(4), result.order_from_root[1]);
}
