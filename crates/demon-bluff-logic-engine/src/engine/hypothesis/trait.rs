#[enum_delegate::register]
pub trait Hypothesis {
    fn describe(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error>;

    fn evaluate<TRepository>(
        &mut self,
        log: &impl ::log::Log,
        depth: crate::engine::Depth,
        game_state: &::demon_bluff_gameplay_engine::game_state::GameState,
        repository: impl crate::engine::HypothesisRepository,
    ) -> crate::engine::HypothesisEvaluation;

    fn wip(&self) -> bool {
        false
    }
}
