use demon_bluff_gameplay_engine::{
    game_state::{Action, DrawStats, RevealResult, new_game},
    villager::{Demon, GoodVillager, Minion, Outcast, VillagerArchetype, VillagerIndex},
};
use demon_bluff_logic_engine::{PlayerAction, predict};

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

    let prediction = predict(&state).expect("prediction failed??");

    assert_eq!(PlayerAction::TryReveal(VillagerIndex(0)), prediction);
}
