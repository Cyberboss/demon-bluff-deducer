use std::fmt::Display;

use ability::AbilityHypothesis;
use execute::ExecuteHypothesis;
use master::MasterHypothesis;
use reveal::RevealHypothesis;
use reveal_index::RevealIndexHypothesis;

use crate::{
    hypotheses::{
        archetype_in_play::ArchetypeInPlayHypothesis, execute_index::ExecuteIndexHypothesis,
        is_evil::IsEvilHypothesis, need_testimony::NeedTestimonyHypothesis,
        revealing_is_safe::RevealingIsSafeHypothesis,
    },
    hypothesis::Hypothesis,
};

mod ability;
mod archetype_in_play;
mod execute;
mod execute_index;
mod is_evil;
pub(crate) mod master;
mod need_testimony;
mod reveal;
mod reveal_index;
mod revealing_is_safe;

#[enum_delegate::implement(Hypothesis)]
#[derive(Eq, PartialEq, Debug)]
pub enum HypothesisType {
    Master(MasterHypothesis),
    Reveal(RevealHypothesis),
    RevealIndex(RevealIndexHypothesis),
    Execute(ExecuteHypothesis),
    Ability(AbilityHypothesis),
    RevealingIsSafe(RevealingIsSafeHypothesis),
    NeedTestimony(NeedTestimonyHypothesis),
    ArchetypeInPlay(ArchetypeInPlayHypothesis),
    ExecuteIndex(ExecuteIndexHypothesis),
    IsEvil(IsEvilHypothesis),
}

impl Display for HypothesisType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.describe(f)
    }
}
