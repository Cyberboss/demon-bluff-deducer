use demon_bluff_gameplay_engine::{
    game_state::{Action, DrawStats, GameStateMutationResult, RevealResult, new_game},
    villager::{
        Demon, GoodVillager, Minion, Outcast, VillagerArchetype, VillagerIndex, VillagerInstance,
    },
};
use demon_bluff_logic_engine::{player_action::PlayerAction, predict};

#[test]
pub fn test_game_1() {
    let log = log::logger();

    let mut state = new_game(
        vec![
            VillagerArchetype::GoodVillager(GoodVillager::Druid),
            VillagerArchetype::GoodVillager(GoodVillager::Architect),
            VillagerArchetype::GoodVillager(GoodVillager::Medium),
            VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
            VillagerArchetype::GoodVillager(GoodVillager::Slayer),
            VillagerArchetype::GoodVillager(GoodVillager::Alchemist),
            VillagerArchetype::Outcast(Outcast::Bombardier),
            VillagerArchetype::Minion(Minion::Witch),
            VillagerArchetype::Demon(Demon::Pooka),
        ],
        DrawStats::new(4, 1, 1, 1),
        2,
        false,
    );

    let prediction_1 = predict(&log, &state).expect("prediction failed??");

    assert_eq!(
        &PlayerAction::TryReveal(VillagerIndex(0)),
        prediction_1.iter().next().unwrap()
    );

    let mut mutation_result = state
        .mutate(Action::TryReveal(RevealResult::new(
            VillagerIndex(0),
            Some(VillagerInstance::new(
                VillagerArchetype::GoodVillager(GoodVillager::Slayer),
                None,
            )),
        )))
        .expect("malformed game step??");
    assert_eq!(GameStateMutationResult::Continue, mutation_result);

    colog::init();
    let log = log::logger();
    let prediction_2 = predict(&log, &state).expect("prediction failed??");

    assert_eq!(
        &PlayerAction::TryReveal(VillagerIndex(1)),
        prediction_2.iter().next().unwrap()
    );

    todo!("rest of the fucking owl")
}
