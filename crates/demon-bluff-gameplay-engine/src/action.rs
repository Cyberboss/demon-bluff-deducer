pub enum Action<'a> {
    Reveal(VillagerIndex),
    Kill(VillagerIndex),
    Ability {
        source: VillagerIndex,
        targets: &'a [VillagerIndex],
    },
}
