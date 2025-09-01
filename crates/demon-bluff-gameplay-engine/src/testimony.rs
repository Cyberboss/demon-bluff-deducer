use std::fmt::Display;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
	Expression,
	villager::{VillagerArchetype, VillagerIndex},
};
const ALCHEMIST_CURE_RANGE: usize = 2;

#[derive(Clone, Debug, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum ConfessorClaim {
	Good,
	Dizzy,
}

#[derive(Clone, Debug, PartialEq, Eq, Display, Hash, Serialize, Deserialize)]
pub enum Direction {
	Clockwise,
	CounterClockwise,
	Equidistant,
}

#[derive(Clone, Debug, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum ArchitectClaim {
	Left,
	Right,
	Equal,
}

#[derive(Clone, Debug, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum BakerClaim {
	Original,
	Was(VillagerArchetype),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlayResult {
	index: VillagerIndex,
	slayed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleClaim {
	villager: VillagerIndex,
	archetype: VillagerArchetype,
}

impl Display for RoleClaim {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} is {}", self.villager, self.archetype)
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoutClaim {
	evil_role: VillagerArchetype,
	distance: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvilPairsClaim(u8);

impl Display for EvilPairsClaim {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} pairs of adjacent evils", self.0)
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Testimony {
	Good(VillagerIndex),
	Evil(VillagerIndex),
	Corrupt(VillagerIndex),
	NotCorrupt(VillagerIndex),
	Lying(VillagerIndex),
	Cured(VillagerIndex),
	Architect(ArchitectClaim),
	Baker(BakerClaim),
	Role(RoleClaim),
	Invincible(VillagerIndex),
	Knitter(EvilPairsClaim),
	Affected(VillagerIndex),
	FakeEvil(VillagerIndex),
	SelfDestruct(VillagerIndex),
	Slayed(SlayResult),
	Confess(ConfessorClaim),
	Scout(ScoutClaim),
}

impl EvilPairsClaim {
	pub fn new(pair_count: u8) -> Self {
		Self(pair_count)
	}
}

impl RoleClaim {
	pub fn new(villager: VillagerIndex, archetype: VillagerArchetype) -> Self {
		Self {
			villager,
			archetype,
		}
	}

	pub fn index(&self) -> &VillagerIndex {
		&self.villager
	}

	pub fn role(&self) -> &VillagerArchetype {
		&self.archetype
	}
}

impl SlayResult {
	pub fn new(index: VillagerIndex, slayed: bool) -> Self {
		Self { index, slayed }
	}

	pub fn slayed(&self) -> bool {
		self.slayed
	}

	pub fn index(&self) -> &VillagerIndex {
		&self.index
	}
}

impl Testimony {
	pub fn alchemist(
		start_index: VillagerIndex,
		villagers_cured: usize,
		total_villagers: usize,
	) -> Expression<Testimony> {
		let start_index = start_index.0;
		let cure_range = ALCHEMIST_CURE_RANGE;
		let candidate_count = cure_range * 2;
		let mut potential_indicies = Vec::with_capacity(candidate_count);
		let mut current_index = if start_index >= cure_range {
			start_index - cure_range
		} else {
			total_villagers - (cure_range - start_index)
		};

		for _ in 0..cure_range {
			potential_indicies.push(current_index);
			current_index = if current_index + 1 >= total_villagers {
				0
			} else {
				current_index + 1
			}
		}

		current_index = if current_index + 1 >= total_villagers {
			0
		} else {
			current_index + 1
		};

		for _ in 0..cure_range {
			potential_indicies.push(current_index);
			current_index = if current_index + 1 >= total_villagers {
				0
			} else {
				current_index + 1
			}
		}

		let mut expr;
		if villagers_cured == 0 {
			expr = Expression::and_from_iterator(
				potential_indicies
					.iter()
					.map(|index| Testimony::NotCorrupt(VillagerIndex(*index))),
			);
		} else {
			expr = None;
			for combo in potential_indicies.iter().combinations(villagers_cured) {
				let new_expr = Expression::and_from_iterator(
					combo
						.iter()
						.map(|index| Testimony::Cured(VillagerIndex(**index))),
				);

				let new_expr =
					new_expr.expect("There should have been at least one villager that was cured");
				expr = Some(match expr {
					Some(old_expr) => Expression::Or(Box::new(old_expr), Box::new(new_expr)),
					None => new_expr,
				});
			}
		}

		expr.expect("logic error in cure expression builder")
	}

	pub fn hunter(
		start_index: &VillagerIndex,
		distance: usize,
		total_villagers: usize,
	) -> Expression<Testimony> {
		// hunter = (+N is evil || -N is evil) && (+(<N) good && -(<N) good)

		let clockwise_evil_unary = Expression::Leaf(Testimony::Evil(index_offset(
			start_index,
			total_villagers,
			distance,
			true,
		)));
		let counter_clockwise_evil_unary = Expression::Leaf(Testimony::Evil(index_offset(
			start_index,
			total_villagers,
			distance,
			false,
		)));

		let evil_or = Expression::Or(
			Box::new(clockwise_evil_unary),
			Box::new(counter_clockwise_evil_unary),
		);

		let mut good_expression = None;
		for i in 1..distance {
			let clockwise_good_unary = Expression::Leaf(Testimony::Good(index_offset(
				start_index,
				total_villagers,
				i,
				true,
			)));
			let counter_clockwise_good_unary = Expression::Leaf(Testimony::Good(index_offset(
				start_index,
				total_villagers,
				i,
				false,
			)));

			let good_and = Expression::And(
				Box::new(clockwise_good_unary),
				Box::new(counter_clockwise_good_unary),
			);

			good_expression = Some(match good_expression {
				Some(other_goods) => Expression::And(Box::new(other_goods), Box::new(good_and)),
				None => good_and,
			});
		}

		match good_expression {
			Some(good_expression) => Expression::And(Box::new(good_expression), Box::new(evil_or)),
			None => evil_or,
		}
	}

	pub fn lover(
		start_index: &VillagerIndex,
		amount: usize,
		total_villagers: usize,
	) -> Expression<Testimony> {
		if amount > 2 {
			panic!("Invalid amount of lover evils");
		}

		if amount == 2 {
			Expression::And(
				Box::new(Expression::Leaf(Testimony::Evil(index_offset(
					start_index,
					total_villagers,
					1,
					true,
				)))),
				Box::new(Expression::Leaf(Testimony::Evil(index_offset(
					start_index,
					total_villagers,
					1,
					false,
				)))),
			)
		} else if amount == 0 {
			Expression::And(
				Box::new(Expression::Leaf(Testimony::Good(index_offset(
					start_index,
					total_villagers,
					1,
					true,
				)))),
				Box::new(Expression::Leaf(Testimony::Good(index_offset(
					start_index,
					total_villagers,
					1,
					false,
				)))),
			)
		} else {
			Expression::Or(
				Box::new(Expression::And(
					Box::new(Expression::Leaf(Testimony::Good(index_offset(
						start_index,
						total_villagers,
						1,
						true,
					)))),
					Box::new(Expression::Leaf(Testimony::Evil(index_offset(
						start_index,
						total_villagers,
						1,
						false,
					)))),
				)),
				Box::new(Expression::And(
					Box::new(Expression::Leaf(Testimony::Evil(index_offset(
						start_index,
						total_villagers,
						1,
						true,
					)))),
					Box::new(Expression::Leaf(Testimony::Good(index_offset(
						start_index,
						total_villagers,
						1,
						false,
					)))),
				)),
			)
		}
	}

	pub fn englightened(
		start_index: &VillagerIndex,
		direction: Direction,
		total_villagers: usize,
	) -> Expression<Testimony> {
		if total_villagers < 3 {
			panic!("No thank you");
		}

		let odd_villagers = total_villagers % 2 != 0;
		let half_villagers = total_villagers / 2;
		let non_opposite_villagers_per_side = if odd_villagers {
			half_villagers
		} else {
			half_villagers - 1
		};

		let eqidistant_1: [VillagerIndex; 1];
		let eqidistant_2: [VillagerIndex; 2];

		let equidistant_villagers: &[VillagerIndex] = if odd_villagers {
			eqidistant_2 = [
				index_offset(start_index, total_villagers, half_villagers, false),
				index_offset(start_index, total_villagers, half_villagers, true),
			];
			eqidistant_2.as_slice()
		} else {
			eqidistant_1 = [index_offset(
				start_index,
				total_villagers,
				half_villagers,
				false,
			)];
			eqidistant_1.as_slice()
		};

		let mut expression = None;
		if direction == Direction::Equidistant {
			let check_range = if odd_villagers {
				non_opposite_villagers_per_side - 1
			} else {
				non_opposite_villagers_per_side
			};

			for check_distance in 1..=check_range {
				let additional_expression = Expression::And(
					Box::new(Expression::Leaf(Testimony::Good(index_offset(
						start_index,
						total_villagers,
						check_distance,
						true,
					)))),
					Box::new(Expression::Leaf(Testimony::Good(index_offset(
						start_index,
						total_villagers,
						check_distance,
						false,
					)))),
				);
				expression = Some(match expression {
					Some(existing_expression) => Expression::And(
						Box::new(existing_expression),
						Box::new(additional_expression),
					),
					None => additional_expression,
				});
			}

			let evil_expression = if odd_villagers {
				Expression::And(
					Box::new(Expression::Leaf(Testimony::Evil(index_offset(
						start_index,
						total_villagers,
						non_opposite_villagers_per_side,
						true,
					)))),
					Box::new(Expression::Leaf(Testimony::Evil(index_offset(
						start_index,
						total_villagers,
						non_opposite_villagers_per_side,
						false,
					)))),
				)
			} else {
				Expression::Leaf(Testimony::Evil(index_offset(
					start_index,
					total_villagers,
					half_villagers,
					true,
				)))
			};

			match expression {
				Some(good_expression) => {
					Expression::And(Box::new(good_expression), Box::new(evil_expression))
				}
				None => evil_expression,
			}
		} else {
			let mut good_indicies = Vec::<VillagerIndex>::with_capacity(total_villagers - 1);

			for check_distance in 1..=non_opposite_villagers_per_side {
				let mut directional_expression = |evil_clockwise| {
					let evil_index =
						index_offset(start_index, total_villagers, check_distance, evil_clockwise);
					let good_index = index_offset(
						start_index,
						total_villagers,
						check_distance,
						!evil_clockwise,
					);

					let mut expression = Expression::And(
						Box::new(Expression::Leaf(Testimony::Evil(evil_index.clone()))),
						Box::new(Expression::Leaf(Testimony::Good(good_index.clone()))),
					);

					for additional_good_index in &good_indicies {
						expression = Expression::And(
							Box::new(expression),
							Box::new(Expression::Leaf(Testimony::Good(
								additional_good_index.clone(),
							))),
						)
					}

					good_indicies.push(good_index);
					good_indicies.push(evil_index); // if its a further case, the evil index is now good
					expression
				};

				let new_expression = match direction {
					Direction::Clockwise => directional_expression(true),
					Direction::CounterClockwise => directional_expression(false),
					Direction::Equidistant => {
						unreachable!("We should not be checking equidistance!")
					}
				};

				expression = Some(match expression {
					Some(existing_expression) => {
						Expression::Or(Box::new(existing_expression), Box::new(new_expression))
					}
					None => new_expression,
				})
			}

			let mut non_optional_expression = expression
				.expect("Logic error in enlightened testimony builder directional route!");

			if !odd_villagers {
				// The equidistant villagers is also good
				for equidistant_villager in equidistant_villagers {
					non_optional_expression = Expression::And(
						Box::new(non_optional_expression),
						Box::new(Expression::Leaf(Testimony::Good(
							equidistant_villager.clone(),
						))),
					);
				}
			}

			non_optional_expression
		}
	}
}

impl Display for Testimony {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Good(villager_index) => write!(f, "{villager_index} is good"),
			Self::Evil(villager_index) => write!(f, "{villager_index} is evil"),
			Self::Corrupt(villager_index) => write!(f, "{villager_index} is corrupt"),
			Self::NotCorrupt(villager_index) => write!(f, "{villager_index} is not corrupt"),
			Self::Lying(villager_index) => write!(f, "{villager_index} is lying"),
			Self::Cured(villager_index) => write!(f, "{villager_index} was cured of corruption"),
			Self::Architect(architect_claim) => write!(f, "{architect_claim} side(s) more evil"),
			Self::Baker(baker_claim) => match baker_claim {
				BakerClaim::Original => write!(f, "I was the OG Baker"),
				BakerClaim::Was(villager_archetype) => write!(f, "I was a {villager_archetype}"),
			},
			Self::Role(role_claim) => {
				write!(f, "{} is a {}", role_claim.villager, role_claim.archetype)
			}
			Self::Invincible(villager_index) => write!(f, "{villager_index} is invincible"),
			Self::Knitter(evil_pairs_claim) => write!(f, "{evil_pairs_claim} evil pairs present"),
			Self::Affected(villager_index) => write!(f, "{villager_index} was affected"),
			Self::FakeEvil(villager_index) => write!(f, "{villager_index} looks evil but isn't"),
			Self::SelfDestruct(villager_index) => {
				write!(f, "{villager_index} will self destruct")
			}
			Self::Slayed(slay_result) => {
				if slay_result.slayed {
					write!(f, "I killed {}", slay_result.index)
				} else {
					write!(f, "I couldn't kill {}", slay_result.index)
				}
			}
			Self::Confess(confessor_claim) => write!(f, "I confess to being {confessor_claim}"),
			Self::Scout(scout_claim) => write!(
				f,
				"{} is {} away from the nearest evil",
				scout_claim.evil_role, scout_claim.distance
			),
		}
	}
}

fn index_offset(
	start_index: &VillagerIndex,
	total_villagers: usize,
	offset: usize,
	clockwise: bool,
) -> VillagerIndex {
	let mut current_index = start_index.0;
	for _ in 0..offset {
		if clockwise {
			current_index += 1;
			if current_index == total_villagers {
				current_index = 0;
			}
		} else if current_index == 0 {
			current_index = total_villagers - 1;
		} else {
			current_index -= 1;
		}
	}

	VillagerIndex(current_index)
}

#[test]
fn test_hunter() {
	assert_eq!(
		Expression::And(
			Box::new(Expression::And(
				Box::new(Expression::Leaf(Testimony::Good(VillagerIndex(2)))), // #3
				Box::new(Expression::Leaf(Testimony::Good(VillagerIndex(0))))  // #1
			)),
			Box::new(Expression::Or(
				Box::new(Expression::Leaf(Testimony::Evil(VillagerIndex(3)))), // #4
				Box::new(Expression::Leaf(Testimony::Evil(VillagerIndex(4))))  // #5
			))
		),
		Testimony::hunter(&VillagerIndex(1), 2, 5)
	);
}
