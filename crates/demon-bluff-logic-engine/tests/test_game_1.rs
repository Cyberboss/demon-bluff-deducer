use demon_bluff_gameplay_engine::{
    Expression,
    game_state::{Action, DrawStats, GameStateMutationResult, RevealResult, new_game},
    testimony::{ALCHEMIST_CURE_RANGE, ArchitectClaim, RoleClaim, Testimony},
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

    let mut prediction = predict(&log, &state).expect("prediction failed??");

    assert_eq!(
        &PlayerAction::TryReveal(VillagerIndex(0)),
        prediction.iter().next().unwrap()
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

    prediction = predict(&log, &state).expect("prediction failed??");

    assert_eq!(
        &PlayerAction::TryReveal(VillagerIndex(1)),
        prediction.iter().next().unwrap()
    );

    mutation_result = state
        .mutate(Action::TryReveal(RevealResult::new(
            VillagerIndex(1),
            Some(VillagerInstance::new(
                VillagerArchetype::GoodVillager(GoodVillager::Medium),
                Some(Expression::Unary(Testimony::Role(RoleClaim::new(
                    VillagerIndex(3),
                    VillagerArchetype::GoodVillager(GoodVillager::Architect),
                )))),
            )),
        )))
        .expect("malformed game step??");
    assert_eq!(GameStateMutationResult::Continue, mutation_result);

    prediction = predict(&log, &state).expect("prediction failed??");

    assert_eq!(
        &PlayerAction::TryReveal(VillagerIndex(2)),
        prediction.iter().next().unwrap()
    );

    mutation_result = state
        .mutate(Action::TryReveal(RevealResult::new(
            VillagerIndex(3),
            Some(VillagerInstance::new(
                VillagerArchetype::GoodVillager(GoodVillager::Architect),
                Some(Expression::Unary(Testimony::Architect(
                    ArchitectClaim::Right,
                ))),
            )),
        )))
        .expect("malformed game step??");
    assert_eq!(GameStateMutationResult::Continue, mutation_result);

    colog::init();
    let log = log::logger();
    prediction = predict(&log, &state).expect("prediction failed??");

    assert_eq!(
        &PlayerAction::TryReveal(VillagerIndex(2)),
        prediction.iter().next().unwrap()
    );

    todo!("rest of the fucking owl")
}
