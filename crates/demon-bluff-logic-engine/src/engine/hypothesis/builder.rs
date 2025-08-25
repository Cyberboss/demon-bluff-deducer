#[enum_delegate::register]
pub trait HypothesisBuilder {
    fn build<TLog>(
        self,
        game_state: &::demon_bluff_gameplay_engine::game_state::GameState,
        registrar: &mut crate::engine::HypothesisRegistrar<TLog>,
    ) -> crate::hypotheses::HypothesisType
    where
        TLog: ::log::Log;
}
