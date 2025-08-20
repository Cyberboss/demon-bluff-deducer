use demon_bluff_gameplay_engine::{
    Expression,
    game_state::{
        AbilityResult, Action, DrawStats, GameStateMutationError, GameStateMutationResult,
        KillAttempt, KillData, KillResult, RevealResult, SlayerKill, new_game,
    },
    testimony::{ALCHEMIST_CURE_RANGE, ArchitectClaim, BakerClaim, RoleClaim, Testimony},
    villager::{
        Demon, GoodVillager, Minion, Outcast, VillagerArchetype, VillagerIndex, VillagerInstance,
    },
};

#[test]
pub fn test_game_1() {
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
    mutation_result = state
        .mutate(Action::TryReveal(RevealResult::new(
            VillagerIndex(1),
            Some(VillagerInstance::new(
                VillagerArchetype::GoodVillager(GoodVillager::Medium),
                Some(Expression::Unary(Testimony::Role(vec![RoleClaim::new(
                    VillagerIndex(1),
                    VillagerArchetype::GoodVillager(GoodVillager::Architect),
                )]))),
            )),
        )))
        .expect("malformed game step??");
    assert_eq!(GameStateMutationResult::Continue, mutation_result);

    let mut result = state.mutate(Action::TryReveal(RevealResult::new(
        VillagerIndex(1),
        Some(VillagerInstance::new(
            VillagerArchetype::GoodVillager(GoodVillager::Medium),
            Some(Expression::Unary(Testimony::Role(vec![RoleClaim::new(
                VillagerIndex(2),
                VillagerArchetype::GoodVillager(GoodVillager::Architect),
            )]))),
        )),
    )));

    match result {
        Ok(_) => panic!("Expected an error result"),
        Err(error) => match error {
            GameStateMutationError::VillagerCannotBeRevealed => {}
            _ => panic!("Incorrect error result"),
        },
    }

    result = state.mutate(Action::TryReveal(RevealResult::new(
        VillagerIndex(2),
        Some(VillagerInstance::new(
            VillagerArchetype::GoodVillager(GoodVillager::Bishop),
            None,
        )),
    )));

    match result {
        Ok(_) => panic!("Expected an error result"),
        Err(error) => match error {
            GameStateMutationError::InvalidReveal => {}
            _ => panic!("Incorrect error result"),
        },
    }

    mutation_result = state
        .mutate(Action::TryReveal(RevealResult::new(
            VillagerIndex(2),
            Some(VillagerInstance::new(
                VillagerArchetype::GoodVillager(GoodVillager::Alchemist),
                Some(Testimony::cure(
                    VillagerIndex(2),
                    2,
                    state.total_villagers(),
                    ALCHEMIST_CURE_RANGE,
                )),
            )),
        )))
        .expect("malformed game step??");
    assert_eq!(GameStateMutationResult::Continue, mutation_result);

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

    result = state.mutate(Action::TryReveal(RevealResult::new(
        VillagerIndex(4),
        Some(VillagerInstance::new(
            VillagerArchetype::GoodVillager(GoodVillager::Druid),
            Some(Expression::Unary(Testimony::Baker(BakerClaim::Original))),
        )),
    )));

    match result {
        Ok(_) => panic!("Expected an error result"),
        Err(error) => match error {
            GameStateMutationError::RevealActionAndTestimony => {}
            _ => panic!("Incorrect error result"),
        },
    }

    mutation_result = state
        .mutate(Action::TryReveal(RevealResult::new(
            VillagerIndex(4),
            Some(VillagerInstance::new(
                VillagerArchetype::GoodVillager(GoodVillager::Druid),
                None,
            )),
        )))
        .expect("malformed game step??");
    assert_eq!(GameStateMutationResult::Continue, mutation_result);

    result = state.mutate(Action::TryReveal(RevealResult::new(
        VillagerIndex(5),
        Some(VillagerInstance::new(
            VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
            None,
        )),
    )));

    match result {
        Ok(_) => panic!("Expected an error result"),
        Err(error) => match error {
            GameStateMutationError::RevealNoActionNorTestimony => {}
            _ => panic!("Incorrect error result"),
        },
    }

    mutation_result = state
        .mutate(Action::TryReveal(RevealResult::new(
            VillagerIndex(5),
            Some(VillagerInstance::new(
                VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
                Some(Expression::Unary(Testimony::Good(vec![VillagerIndex(0)]))),
            )),
        )))
        .expect("malformed game step??");
    assert_eq!(GameStateMutationResult::Continue, mutation_result);

    mutation_result = state
        .mutate(Action::TryReveal(RevealResult::new(VillagerIndex(6), None)))
        .expect("malformed game step??");
    assert_eq!(GameStateMutationResult::Continue, mutation_result);

    result = state.mutate(Action::TryReveal(RevealResult::new(VillagerIndex(6), None)));

    match result {
        Ok(_) => panic!("Expected an error result"),
        Err(error) => match error {
            GameStateMutationError::VillagerCannotBeRevealed => {}
            _ => panic!("Incorrect error result"),
        },
    }

    mutation_result = state
        .mutate(Action::Ability(AbilityResult::new(
            VillagerIndex(0),
            Some(Expression::Unary(Testimony::Slayed(VillagerIndex(1)))),
            Some(SlayerKill::new(
                VillagerIndex(1),
                KillResult::Revealed(
                    KillData::new(Some(VillagerArchetype::Minion(Minion::Witch)), false)
                        .expect("malformed kill data??"),
                ),
            )),
        )))
        .expect("malformed game step??");
    assert_eq!(GameStateMutationResult::Continue, mutation_result);

    mutation_result = state
        .mutate(Action::TryReveal(RevealResult::new(
            VillagerIndex(6),
            Some(VillagerInstance::new(
                VillagerArchetype::Outcast(Outcast::Bombardier),
                Some(Expression::Unary(Testimony::SelfDestruct(vec![
                    VillagerIndex(6),
                ]))),
            )),
        )))
        .expect("malformed game step??");
    assert_eq!(GameStateMutationResult::Continue, mutation_result);

    mutation_result = state
        .mutate(Action::TryExecute(KillAttempt::new(
            VillagerIndex(3),
            Some(KillResult::Revealed(
                KillData::new(Some(VillagerArchetype::Demon(Demon::Pooka)), false)
                    .expect("malformed kill data??"),
            )),
        )))
        .unwrap();
    assert_eq!(GameStateMutationResult::Win, mutation_result);
}
