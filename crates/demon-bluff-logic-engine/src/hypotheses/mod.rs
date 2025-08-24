use std::fmt::Display;

use execute::ExecuteHypothesis;
use master::MasterHypothesis;
use reveal::RevealHypothesis;
use reveal_index::RevealIndexHypothesis;

use crate::{
    hypotheses::{
        ability::{AbilityHypothesis, AbilityHypothesisBuilder},
        archetype_in_play::{ArchetypeInPlayHypothesis, ArchetypeInPlayHypothesisBuilder},
        corruption_in_play::{CorruptionInPlayHypothesis, CorruptionInPlayHypothesisBuilder},
        execute::ExecuteHypothesisBuilder,
        execute_index::{ExecuteIndexHypothesis, ExecuteIndexHypothesisBuilder},
        gather_information::GatherInformationHypothesis,
        is_corrupt::IsCorruptHypothesis,
        is_evil::IsEvilHypothesis,
        is_truthful::IsTruthfulHypothesis,
        master::MasterHypothesisBuilder,
        need_testimony::NeedTestimonyHypothesis,
        negate::NegateHypothesis,
        revealing_is_safe::RevealingIsSafeHypothesis,
        testimony::TestimonyHypothesis,
        testimony_expression::TestimonyExpressionHypothesis,
    },
    hypothesis::{Hypothesis, HypothesisBuilder},
};

mod ability;
mod archetype_in_play;
mod corruption_in_play;
mod execute;
mod execute_index;
mod gather_information;
mod is_corrupt;
mod is_evil;
mod is_truthful;
pub(crate) mod master;
mod need_testimony;
mod negate;
mod reveal;
mod reveal_index;
mod revealing_is_safe;
mod testimony;
mod testimony_expression;

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
}

impl Display for HypothesisType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.describe(f)
    }
}
