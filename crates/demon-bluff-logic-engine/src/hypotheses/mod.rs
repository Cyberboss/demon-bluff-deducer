use std::fmt::Display;

use ability::AbilityHypothesis;
use execute::ExecuteHypothesis;
use master::MasterHypothesis;
use reveal::RevealHypothesis;
use reveal_index::RevealIndexHypothesis;

use crate::hypothesis::Hypothesis;

mod ability;
mod execute;
pub(crate) mod master;
mod reveal;
mod reveal_index;

#[enum_delegate::implement(Hypothesis)]
#[derive(Eq, PartialEq, Debug)]
pub enum HypothesisType {
    Master(MasterHypothesis),
    Reveal(RevealHypothesis),
    RevealIndex(RevealIndexHypothesis),
    Execute(ExecuteHypothesis),
    Ability(AbilityHypothesis),
}

impl Display for HypothesisType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.describe(f)
    }
}
