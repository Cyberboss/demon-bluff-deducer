use std::{fmt::Display, num::NonZeroUsize};

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use strum::Display;

use crate::{
	Expression,
	villager::{GoodVillager, Outcast, VillagerArchetype, VillagerIndex},
};
const ALCHEMIST_CURE_RANGE: usize = 2;

#[derive(Clone, Debug, PartialEq, Eq, Display, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub enum ConfessorClaim {
	Good,
	Dizzy,
}

#[derive(Clone, Debug, PartialEq, Eq, Display, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub enum Direction {
	Clockwise,
	CounterClockwise,
	Equidistant,
}

#[derive(Clone, Debug, PartialEq, Eq, Display, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub enum ArchitectClaim {
	Left,
	Right,
	Equal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub struct BakerClaim {
	was: Option<GoodVillager>,
}

impl BakerClaim {
	pub fn new(was: Option<GoodVillager>) -> Self {
		Self { was }
	}

	pub fn was(&self) -> &Option<GoodVillager> {
		&self.was
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub struct SlayResult {
	index: VillagerIndex,
	slayed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub struct RoleClaim {
	villager: VillagerIndex,
	archetype: VillagerArchetype,
}

impl Display for RoleClaim {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} is {}", self.villager, self.archetype)
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub struct DreamerClaim {
	villager: VillagerIndex,
	archetype: Option<VillagerArchetype>,
}

impl DreamerClaim {
	pub fn new(villager: VillagerIndex, archetype: Option<VillagerArchetype>) -> Self {
		Self {
			villager,
			archetype,
		}
	}

	pub fn target(&self) -> &VillagerIndex {
		&self.villager
	}

	pub fn role(&self) -> &Option<VillagerArchetype> {
		&self.archetype
	}
}

impl Display for DreamerClaim {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} is a ", self.villager)?;
		match &self.archetype {
			Some(role) => write!(f, "{}", role),
			None => write!(f, "cabbage"),
		}
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub struct ScoutClaim {
	evil_role: VillagerArchetype,
	distance: usize,
}

impl ScoutClaim {
	pub fn new(evil_role: VillagerArchetype, distance: usize) -> Self {
		Self {
			evil_role,
			distance,
		}
	}

	pub fn evil_role(&self) -> &VillagerArchetype {
		&self.evil_role
	}

	pub fn distance(&self) -> usize {
		self.distance
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub struct EvilPairsClaim(usize);

impl Display for EvilPairsClaim {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} pairs of adjacent evils", self.0)
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord, Display)]
pub enum AffectType {
	Puppeted,
	CorruptedByEvil,
	Outcasted,
	Cloned,
	Killed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub struct AffectedClaim {
	index: VillagerIndex,
	affect_type: AffectType,
}

impl AffectedClaim {
	pub fn new(index: VillagerIndex, affect_type: AffectType) -> Self {
		Self { index, affect_type }
	}

	pub fn index(&self) -> &VillagerIndex {
		&self.index
	}

	pub fn affect_type(&self) -> &AffectType {
		&self.affect_type
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub struct FortuneTellerClaim {
	targets: [VillagerIndex; 2],
	evil: bool,
}

impl FortuneTellerClaim {
	pub fn new(targets: &[VillagerIndex; 2], evil: bool) -> Self {
		Self {
			targets: targets.clone(),
			evil,
		}
	}

	pub fn targets(&self) -> &[VillagerIndex; 2] {
		&self.targets
	}

	pub fn evil(&self) -> bool {
		self.evil
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub struct DruidClaim {
	targets: [VillagerIndex; 3],
	outcast: Option<Outcast>,
}

impl DruidClaim {
	pub fn new(targets: &[VillagerIndex; 3], outcast: Option<Outcast>) -> Self {
		Self {
			targets: targets.clone(),
			outcast,
		}
	}

	pub fn targets(&self) -> &[VillagerIndex; 3] {
		&self.targets
	}

	pub fn outcast(&self) -> &Option<Outcast> {
		&self.outcast
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub enum BishopEvil {
	Minion,
	Demon,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub struct BishopClaim {
	targets: [VillagerIndex; 3],
	good_villager: bool,
	outcast: bool,
	evil: Option<BishopEvil>,
}

impl BishopClaim {
	pub fn new(
		targets: &[VillagerIndex; 3],
		good_villager: bool,
		outcast: bool,
		evil: Option<BishopEvil>,
	) -> Self {
		Self {
			targets: targets.clone(),
			good_villager,
			outcast,
			evil,
		}
	}

	pub fn targets(&self) -> &[VillagerIndex; 3] {
		&self.targets
	}

	pub fn good_villager(&self) -> bool {
		self.good_villager
	}

	pub fn outcast(&self) -> bool {
		self.outcast
	}

	pub fn evil(&self) -> &Option<BishopEvil> {
		&self.evil
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub enum Testimony {
	Good(VillagerIndex),
	// both are needed because of wretch (looks evil) != !(looks good)
	Evil(VillagerIndex),
	Corrupt(VillagerIndex),
	Lying(VillagerIndex),
	Cured(usize),
	Baker(BakerClaim),
	Role(RoleClaim),
	Invincible(VillagerIndex),
	// TODO: What are they saying?
	Affected(Option<AffectedClaim>),
	FakeEvil(VillagerIndex),
	SelfDestruct(VillagerIndex),
	SlayAttempt(SlayResult),
	Confess(ConfessorClaim),
	Scout(ScoutClaim),
	// as sane as it'd sound to express these in terms of good/evil it blows up the problem space way too much due to associating testimonies with indexes in the boolean expression tree
	// handle it at validation time
	Enlightened(Direction),
	Knitter(EvilPairsClaim),
	Bard(Option<NonZeroUsize>),
	FortuneTeller(FortuneTellerClaim),
	Druid(DruidClaim),
	Architect(ArchitectClaim),
	Bishop(BishopClaim),
	Dreamer(DreamerClaim),
}

impl EvilPairsClaim {
	pub fn new(pair_count: usize) -> Self {
		Self(pair_count)
	}

	pub fn pairs(&self) -> usize {
		self.0
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
		start_index: &VillagerIndex,
		villagers_cured: usize,
		total_villagers: usize,
	) -> Expression<Testimony> {
		Expression::Leaf(Testimony::Cured(villagers_cured))
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

	pub fn empress(suspects: &[VillagerIndex; 3]) -> Expression<Testimony> {
		let statement_1 = Expression::And(
			Box::new(Expression::Leaf(Testimony::Evil(suspects[0].clone()))),
			Box::new(Expression::And(
				Box::new(Expression::Leaf(Testimony::Good(suspects[1].clone()))),
				Box::new(Expression::Leaf(Testimony::Good(suspects[2].clone()))),
			)),
		);
		let statement_2 = Expression::And(
			Box::new(Expression::Leaf(Testimony::Evil(suspects[1].clone()))),
			Box::new(Expression::And(
				Box::new(Expression::Leaf(Testimony::Good(suspects[0].clone()))),
				Box::new(Expression::Leaf(Testimony::Good(suspects[2].clone()))),
			)),
		);
		let statement_3 = Expression::And(
			Box::new(Expression::Leaf(Testimony::Evil(suspects[2].clone()))),
			Box::new(Expression::And(
				Box::new(Expression::Leaf(Testimony::Good(suspects[1].clone()))),
				Box::new(Expression::Leaf(Testimony::Good(suspects[0].clone()))),
			)),
		);

		let expression = Expression::Or(
			Box::new(statement_3),
			Box::new(Expression::Or(Box::new(statement_1), Box::new(statement_2))),
		);

		expression
	}

	pub fn fortune_teller(targets: &[VillagerIndex; 2], evil: bool) -> Expression<Testimony> {
		Expression::Leaf(Testimony::FortuneTeller(FortuneTellerClaim::new(
			targets, evil,
		)))
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
		return Expression::Leaf(Testimony::Enlightened(direction));
	}

	pub fn oracle(
		targets: &[VillagerIndex; 2],
		archetype: VillagerArchetype,
	) -> Expression<Testimony> {
		Expression::Or(
			Box::new(Expression::Leaf(Testimony::Role(RoleClaim::new(
				targets[0].clone(),
				archetype.clone(),
			)))),
			Box::new(Expression::Leaf(Testimony::Role(RoleClaim::new(
				targets[1].clone(),
				archetype,
			)))),
		)
	}

	pub fn jester(targets: &[VillagerIndex; 3], evil_count: usize) -> Expression<Testimony> {
		match evil_count {
			0 => Expression::And(
				Box::new(Expression::Not(Box::new(Expression::Leaf(
					Testimony::Evil(targets[0].clone()),
				)))),
				Box::new(Expression::And(
					Box::new(Expression::Not(Box::new(Expression::Leaf(
						Testimony::Evil(targets[1].clone()),
					)))),
					Box::new(Expression::Not(Box::new(Expression::Leaf(
						Testimony::Evil(targets[2].clone()),
					)))),
				)),
			),
			1 => Expression::Or(
				Box::new(Expression::And(
					Box::new(Expression::Leaf(Testimony::Evil(targets[0].clone()))),
					Box::new(Expression::And(
						Box::new(Expression::Not(Box::new(Expression::Leaf(
							Testimony::Evil(targets[1].clone()),
						)))),
						Box::new(Expression::Not(Box::new(Expression::Leaf(
							Testimony::Evil(targets[2].clone()),
						)))),
					)),
				)),
				Box::new(Expression::Or(
					Box::new(Expression::And(
						Box::new(Expression::Not(Box::new(Expression::Leaf(
							Testimony::Evil(targets[0].clone()),
						)))),
						Box::new(Expression::And(
							Box::new(Expression::Leaf(Testimony::Evil(targets[1].clone()))),
							Box::new(Expression::Not(Box::new(Expression::Leaf(
								Testimony::Evil(targets[2].clone()),
							)))),
						)),
					)),
					Box::new(Expression::And(
						Box::new(Expression::Not(Box::new(Expression::Leaf(
							Testimony::Evil(targets[0].clone()),
						)))),
						Box::new(Expression::And(
							Box::new(Expression::Not(Box::new(Expression::Leaf(
								Testimony::Evil(targets[1].clone()),
							)))),
							Box::new(Expression::Leaf(Testimony::Evil(targets[2].clone()))),
						)),
					)),
				)),
			),
			2 => Expression::Or(
				Box::new(Expression::And(
					Box::new(Expression::Not(Box::new(Expression::Leaf(
						Testimony::Evil(targets[0].clone()),
					)))),
					Box::new(Expression::And(
						Box::new(Expression::Leaf(Testimony::Evil(targets[1].clone()))),
						Box::new(Expression::Leaf(Testimony::Evil(targets[2].clone()))),
					)),
				)),
				Box::new(Expression::Or(
					Box::new(Expression::And(
						Box::new(Expression::Leaf(Testimony::Evil(targets[0].clone()))),
						Box::new(Expression::And(
							Box::new(Expression::Not(Box::new(Expression::Leaf(
								Testimony::Evil(targets[1].clone()),
							)))),
							Box::new(Expression::Leaf(Testimony::Evil(targets[2].clone()))),
						)),
					)),
					Box::new(Expression::And(
						Box::new(Expression::Leaf(Testimony::Evil(targets[0].clone()))),
						Box::new(Expression::And(
							Box::new(Expression::Leaf(Testimony::Evil(targets[1].clone()))),
							Box::new(Expression::Not(Box::new(Expression::Leaf(
								Testimony::Evil(targets[2].clone()),
							)))),
						)),
					)),
				)),
			),
			3 => Expression::And(
				Box::new(Expression::Leaf(Testimony::Evil(targets[0].clone()))),
				Box::new(Expression::And(
					Box::new(Expression::Leaf(Testimony::Evil(targets[1].clone()))),
					Box::new(Expression::Leaf(Testimony::Evil(targets[2].clone()))),
				)),
			),
			_ => panic!("A jester can only have up to {} targets", targets.len()),
		}
	}
}

impl Display for Testimony {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Good(villager_index) => write!(f, "{villager_index} is good"),
			Self::Evil(villager_index) => write!(f, "{villager_index} is evil"),
			Self::Corrupt(villager_index) => write!(f, "{villager_index} is corrupt"),
			Self::Lying(villager_index) => write!(f, "{villager_index} is lying"),
			Self::Cured(villager_index) => write!(f, "{villager_index} was cured of corruption"),
			Self::Baker(baker_claim) => match &baker_claim.was {
				None => write!(f, "I was the OG Baker"),
				Some(villager_archetype) => write!(f, "I was a {villager_archetype}"),
			},
			Self::Role(role_claim) => {
				write!(f, "{} is a {}", role_claim.villager, role_claim.archetype)
			}
			Self::Invincible(villager_index) => write!(f, "{villager_index} is invincible"),
			Self::Affected(claim) => match claim {
				Some(claim) => write!(f, "{} was {}", claim.index(), claim.affect_type()),
				None => write!(f, "No one was affected"),
			},
			Self::FakeEvil(villager_index) => write!(f, "{villager_index} looks evil but isn't"),
			Self::SelfDestruct(villager_index) => {
				write!(f, "{villager_index} will self destruct")
			}
			Self::SlayAttempt(slay_result) => {
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
			Self::Enlightened(direction) => write!(f, "Closest evil is {}", direction),
			Self::Knitter(evil_pairs_claim) => {
				write!(f, "There are {} pair(s) of evils", evil_pairs_claim)
			}
			Self::Bard(distance_option) => match distance_option {
				Some(distance) => write!(
					f,
					"I am {} away from the closest corrupted character",
					distance
				),
				None => write!(f, "There are no corrupted characters"),
			},
			Self::FortuneTeller(fortune_teller_claim) => write!(
				f,
				"Is {} or {} evil? {}",
				fortune_teller_claim.targets[0],
				fortune_teller_claim.targets[1],
				if fortune_teller_claim.evil {
					"TRUE"
				} else {
					"FALSE"
				}
			),
			Self::Druid(druid_claim) => {
				write!(f, "Among ")?;
				let mut first = true;
				for target in druid_claim.targets() {
					if first {
						first = false;
					} else {
						write!(f, ", ")?;
					}

					write!(f, "{}", target)?;
				}

				write!(f, " there ",)?;

				match druid_claim.outcast() {
					Some(minion) => write!(f, "is a {}", minion),
					None => write!(f, "are NO outcasts"),
				}
			}
			Self::Architect(architect_claim) => write!(
				f,
				"{}",
				match architect_claim {
					ArchitectClaim::Left => "Left side is more evil",
					ArchitectClaim::Right => "Right side is more evil",
					ArchitectClaim::Equal => "Both sides are equally evil",
				}
			),
			Self::Bishop(bishop_claim) => {
				write!(f, "Among ")?;
				let mut first = true;
				for target in bishop_claim.targets() {
					if first {
						first = false;
					} else {
						write!(f, ", ")?;
					}

					write!(f, "{}", target)?;
				}

				write!(f, " there are",)?;

				if bishop_claim.good_villager {
					write!(f, " GoodVillager")?;
				}

				if bishop_claim.outcast {
					write!(f, " Outcast")?;
				}

				match bishop_claim.evil() {
					Some(bishop_evil) => write!(
						f,
						" {}",
						match bishop_evil {
							BishopEvil::Minion => "Minion",
							BishopEvil::Demon => "Demon",
						}
					),
					None => Ok(()),
				}
			}
			Self::Dreamer(dreamer_claim) => match &dreamer_claim.archetype {
				Some(role) => {
					write!(f, "{} is a {}", dreamer_claim.villager, role)
				}
				None => write!(f, "{} is a cabbage", dreamer_claim.villager),
			},
		}
	}
}

pub fn index_offset(
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
