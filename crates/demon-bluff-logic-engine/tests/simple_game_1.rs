use demon_bluff_gameplay_engine::{
    Expression,
    game_state::{
        Action, DrawStats, GameStateMutationResult, KillAttempt, KillData, KillResult,
        RevealResult, new_game,
    },
    testimony::{ConfessorClaim, Direction, Testimony},
    villager::{GoodVillager, Minion, VillagerArchetype, VillagerIndex, VillagerInstance},
};
use demon_bluff_logic_engine::{player_action::PlayerAction, predict};

#[test]
pub fn simple_game_1() {
    let log = log::logger();

    let mut state = new_game(
        vec![
            VillagerArchetype::GoodVillager(GoodVillager::Confessor),
            VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
            VillagerArchetype::GoodVillager(GoodVillager::Enlightened),
            VillagerArchetype::GoodVillager(GoodVillager::Hunter),
            VillagerArchetype::Minion(Minion::Minion),
        ],
        DrawStats::new(4, 0, 1, 0),
        1,
        false,
    );

    // reveal confessor
    println!("Prediction 1:");
    let mut prediction = predict(&log, &state).expect("prediction failed??");

    assert_eq!(
        &PlayerAction::TryReveal(VillagerIndex(0)),
        prediction.iter().next().unwrap()
    );

    let mut mutation_result = state
        .mutate(Action::TryReveal(RevealResult::new(
            VillagerIndex(0),
            Some(VillagerInstance::new(
                VillagerArchetype::GoodVillager(GoodVillager::Confessor),
                Some(Expression::Unary(Testimony::Confess(ConfessorClaim::Good))),
            )),
        )))
        .expect("malformed game step??");
    assert_eq!(GameStateMutationResult::Continue, mutation_result);

    // reveal hunter (fake)
    println!("Prediction 2:");
    prediction = predict(&log, &state).expect("prediction failed??");

    assert_eq!(
        &PlayerAction::TryReveal(VillagerIndex(1)),
        prediction.iter().next().unwrap()
    );

    mutation_result = state
        .mutate(Action::TryReveal(RevealResult::new(
            VillagerIndex(1),
            Some(VillagerInstance::new(
                VillagerArchetype::GoodVillager(GoodVillager::Hunter),
                Some(Testimony::hunter(
                    &VillagerIndex(1),
                    state.total_villagers(),
                    state.total_villagers(),
                )),
            )),
        )))
        .expect("malformed game step??");
    assert_eq!(GameStateMutationResult::Continue, mutation_result);

    // reveal gemcrafter
    println!("Prediction 3:");
    prediction = predict(&log, &state).expect("prediction failed??");

    assert_eq!(
        &PlayerAction::TryReveal(VillagerIndex(2)),
        prediction.iter().next().unwrap()
    );

    mutation_result = state
        .mutate(Action::TryReveal(RevealResult::new(
            VillagerIndex(2),
            Some(VillagerInstance::new(
                VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
                Some(Expression::Unary(Testimony::Good(VillagerIndex(4)))),
            )),
        )))
        .expect("malformed game step??");
    assert_eq!(GameStateMutationResult::Continue, mutation_result);

    // reveal hunter2
    println!("Prediction 4:");
    prediction = predict(&log, &state).expect("prediction failed??");

    assert_eq!(
        &PlayerAction::TryReveal(VillagerIndex(3)),
        prediction.iter().next().unwrap()
    );

    mutation_result = state
        .mutate(Action::TryReveal(RevealResult::new(
            VillagerIndex(3),
            Some(VillagerInstance::new(
                VillagerArchetype::GoodVillager(GoodVillager::Hunter),
                Some(Testimony::hunter(
                    &VillagerIndex(3),
                    2,
                    state.total_villagers(),
                )),
            )),
        )))
        .expect("malformed game step??");
    assert_eq!(GameStateMutationResult::Continue, mutation_result);

    // reveal enlightened
    println!("Prediction 5:");
    prediction = predict(&log, &state).expect("prediction failed??");

    assert_eq!(
        &PlayerAction::TryReveal(VillagerIndex(4)),
        prediction.iter().next().unwrap()
    );

    mutation_result = state
        .mutate(Action::TryReveal(RevealResult::new(
            VillagerIndex(4),
            Some(VillagerInstance::new(
                VillagerArchetype::GoodVillager(GoodVillager::Enlightened),
                Some(Expression::Unary(Testimony::Enlightened(
                    Direction::Clockwise,
                ))),
            )),
        )))
        .expect("malformed game step??");
    assert_eq!(GameStateMutationResult::Continue, mutation_result);

    // kill hunter1
    println!("Prediction 6:");

    colog::init();
    let log = log::logger();

    prediction = predict(&log, &state).expect("prediction failed??");

    assert_eq!(
        &PlayerAction::TryExecute(VillagerIndex(1)),
        prediction.iter().next().unwrap()
    );

    mutation_result = state
        .mutate(Action::TryExecute(KillAttempt::new(
            VillagerIndex(1),
            Some(KillResult::Revealed(
                KillData::new(Some(VillagerArchetype::Minion(Minion::Minion)), false)
                    .expect("This is valid kill data"),
            )),
        )))
        .expect("malformed game step??");
    assert_eq!(GameStateMutationResult::Win, mutation_result);
}
