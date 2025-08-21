use execute::ExecuteHypothesis;
use master::MasterHypothesis;
use reveal::RevealHypothesis;
use reveal_index::RevealIndexHypothesis;

use crate::hypothesis::Hypothesis;

mod execute;
mod master;
mod reveal;
mod reveal_index;

#[enum_delegate::implement(Hypothesis)]
#[derive(Eq, PartialEq, Debug)]
pub enum HypothesisType {
    Master(MasterHypothesis),
    Reveal(RevealHypothesis),
    RevealIndex(RevealIndexHypothesis),
    Execute(ExecuteHypothesis),
}
