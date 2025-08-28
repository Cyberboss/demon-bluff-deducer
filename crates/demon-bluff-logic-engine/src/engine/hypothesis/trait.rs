#![allow(clippy::all)]
#[enum_delegate::register]
pub trait Hypothesis {
    fn describe(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error>;

    fn evaluate<TLog>(
        &mut self,
        log: &TLog,
        depth: crate::engine::Depth,
        game_state: &::demon_bluff_gameplay_engine::game_state::GameState,
        repository: crate::engine::HypothesisRepository<TLog>,
    ) -> crate::engine::HypothesisEvaluation
    where
        TLog: ::log::Log;

    fn wip(&self) -> bool {
        false
    }
}
