use std::fmt::Display;

use all_evils_accounted_for::{
	AllEvilsAccountedForHypothesis, AllEvilsAccountedForHypothesisBuilder,
};
use appears_evil::{AppearsEvilHypothesis, AppearsEvilHypothesisBuilder};
use testimony_condemns_expression::{
	TestimonyCondemnsExpressionHypothesis, TestimonyCondemnsExpressionHypothesisBuilder,
};
use testimony_exonerates::{TestimonyExoneratesHypothesis, TestimonyExoneratesHypothesisBuilder};
use testimony_exonerates_expression::{
	TestimonyExoneratesExpressionHypothesis, TestimonyExoneratesExpressionHypothesisBuilder,
};

use self::{
	ability_index::{AbilityIndexHypothesis, AbilityIndexHypothesisBuilder},
	is_truly_archetype::{IsTrulyArchetypeHypothesis, IsTrulyArchetypeHypothesisBuilder},
	testimony_condemns::{TestimonyCondemnsHypothesis, TestimonyCondemnsHypothesisBuilder},
	true_identity::{TrueIdentityHypothesis, TrueIdentityHypothesisBuilder},
};
pub use self::{desires::DesireType, master::MasterHypothesisBuilder};
use crate::{
	engine::{Hypothesis, HypothesisBuilder},
	hypotheses::{
		ability::{AbilityHypothesis, AbilityHypothesisBuilder},
		archetype_in_play::{ArchetypeInPlayHypothesis, ArchetypeInPlayHypothesisBuilder},
		corruption_in_play::{CorruptionInPlayHypothesis, CorruptionInPlayHypothesisBuilder},
		execute::{ExecuteHypothesis, ExecuteHypothesisBuilder},
		execute_index::{ExecuteIndexHypothesis, ExecuteIndexHypothesisBuilder},
		gather_information::{GatherInformationHypothesis, GatherInformationHypothesisBuilder},
		is_corrupt::{IsCorruptHypothesis, IsCorruptHypothesisBuilder},
		is_evil::{IsEvilHypothesis, IsEvilHypothesisBuilder},
		is_truthful::{IsTruthfulHypothesis, IsTruthfulHypothesisBuilder},
		master::MasterHypothesis,
		need_testimony::{NeedTestimonyHypothesis, NeedTestimonyHypothesisBuilder},
		negate::{NegateHypothesis, NegateHypothesisBuilder},
		reveal::{RevealHypothesis, RevealHypothesisBuilder},
		reveal_index::{RevealIndexHypothesis, RevealIndexHypothesisBuilder},
		revealing_is_safe::{RevealingIsSafeHypothesis, RevealingIsSafeHypothesisBuilder},
		testimony::{TestimonyHypothesis, TestimonyHypothesisBuilder},
		testimony_expression::{
			TestimonyExpressionHypothesis, TestimonyExpressionHypothesisBuilder,
		},
	},
};

mod ability;
mod ability_index;
mod all_evils_accounted_for;
mod appears_evil;
mod archetype_in_play;
mod corruption_in_play;
mod desires;
mod execute;
mod execute_index;
mod gather_information;
mod hypothesis_expression;
mod is_corrupt;
mod is_evil;
mod is_truly_archetype;
mod is_truthful;
mod master;
mod need_testimony;
mod negate;
mod reveal;
mod reveal_index;
mod revealing_is_safe;
mod testimony;
mod testimony_condemns;
mod testimony_condemns_expression;
mod testimony_exonerates;
mod testimony_exonerates_expression;
mod testimony_expression;
mod true_identity;

#[enum_delegate::implement(HypothesisBuilder)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum HypothesisBuilderType {
	Master(MasterHypothesisBuilder),
	Reveal(RevealHypothesisBuilder),
	RevealIndex(RevealIndexHypothesisBuilder),
	Execute(ExecuteHypothesisBuilder),
	RevealingIsSafe(RevealingIsSafeHypothesisBuilder),
	NeedTestimony(NeedTestimonyHypothesisBuilder),
	ArchetypeInPlay(ArchetypeInPlayHypothesisBuilder),
	ExecuteIndex(ExecuteIndexHypothesisBuilder),
	IsEvil(IsEvilHypothesisBuilder),
	IsTruthful(IsTruthfulHypothesisBuilder),
	TestimonyExpression(TestimonyExpressionHypothesisBuilder),
	Testimony(TestimonyHypothesisBuilder),
	GatherInformation(GatherInformationHypothesisBuilder),
	Ability(AbilityHypothesisBuilder),
	Negate(NegateHypothesisBuilder),
	IsCorrupt(IsCorruptHypothesisBuilder),
	CorruptionInPlay(CorruptionInPlayHypothesisBuilder),
	TrueIdentity(TrueIdentityHypothesisBuilder),
	AbilityIndex(AbilityIndexHypothesisBuilder),
	IsTrulyArchetype(IsTrulyArchetypeHypothesisBuilder),
	TestimonyCondemns(TestimonyCondemnsHypothesisBuilder),
	AllEvilsAccountedFor(AllEvilsAccountedForHypothesisBuilder),
	AppearsEvil(AppearsEvilHypothesisBuilder),
	TestimonyCondemnsExpression(TestimonyCondemnsExpressionHypothesisBuilder),
	TestimonyExoneratesExpression(TestimonyExoneratesExpressionHypothesisBuilder),
	TestimonyExonerates(TestimonyExoneratesHypothesisBuilder),
}

#[enum_delegate::implement(Hypothesis)]
#[derive(Debug)]
pub enum HypothesisType {
	Master(MasterHypothesis),
	Reveal(RevealHypothesis),
	RevealIndex(RevealIndexHypothesis),
	Execute(ExecuteHypothesis),
	RevealingIsSafe(RevealingIsSafeHypothesis),
	NeedTestimony(NeedTestimonyHypothesis),
	ArchetypeInPlay(ArchetypeInPlayHypothesis),
	ExecuteIndex(ExecuteIndexHypothesis),
	IsEvil(IsEvilHypothesis),
	IsTruthful(IsTruthfulHypothesis),
	TestimonyExpression(TestimonyExpressionHypothesis),
	Testimony(TestimonyHypothesis),
	GatherInformation(GatherInformationHypothesis),
	Ability(AbilityHypothesis),
	Negate(NegateHypothesis),
	IsCorrupt(IsCorruptHypothesis),
	CorruptionInPlay(CorruptionInPlayHypothesis),
	TrueIdentity(TrueIdentityHypothesis),
	AbilityIndex(AbilityIndexHypothesis),
	IsTrulyArchetype(IsTrulyArchetypeHypothesis),
	TestimonyCondemns(TestimonyCondemnsHypothesis),
	AllEvilsAccountedFor(AllEvilsAccountedForHypothesis),
	AppearsEvil(AppearsEvilHypothesis),
	TestimonyCondemnsExpression(TestimonyCondemnsExpressionHypothesis),
	TestimonyExoneratesExpression(TestimonyExoneratesExpressionHypothesis),
	TestimonyExonerates(TestimonyExoneratesHypothesis),
}

impl Display for HypothesisType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.describe(f)?;
		write!(f, "{}", if self.wip() { " (WIP)" } else { "" })
	}
}
