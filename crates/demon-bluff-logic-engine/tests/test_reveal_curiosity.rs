
use demon_bluff_gameplay_engine::villager::VillagerIndex;
use demon_bluff_logic_engine::PlayerAction;
use test_helpers::test_game_state;

mod test_helpers;

#[test]
pub fn test_gemcrafter_reveal_follows_her_testimony() {
	test_game_state(
		"gemcrafter_1_says_5_good",
		PlayerAction::TryReveal(VillagerIndex(4)),
	);
}
