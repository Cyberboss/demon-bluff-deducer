use crate::{Expression, affect::Affect, testimony::Testimony};

pub struct VillagerIndex(pub usize);

pub enum GoodVillager {
    Alchemist,
    Architect,
    Baker,
    Bard,
    Bishop,
    Confessor,
    Dreamer,
    Druid,
    Empress,
    Enlightened,
    FortuneTeller,
    Gemcrafter,
    Hunter,
    Jester,
    Judge,
    Knight,
    Knitter,
    Lover,
    Medium,
    Oracle,
    Poet,
    Scout,
    Slayer,
    Witness,
}

pub enum Outcast {
    Drunk,
    Wretch,
    Bombardier,
    Doppelganger,
    PlagueDoctor,
}

pub enum Minion {
    Counsellor,
    Witch,
    Minion,
    Poisoner,
    Twinion,
    Shaman,
    Puppeteer,
    Puppet,
}

pub enum Demon {
    Baa,
    Pooka,
    Lilis,
}

pub enum VillagerArchetype {
    GoodVillager(GoodVillager),
    Outcast(Outcast),
    Minion(Minion),
    Demon(Demon),
}

pub struct RevealedVillager {
    instance: VillagerInstance,
    cant_kill: bool,
}

pub struct HiddenVillager {
    dead: bool,
    cant_reveal: bool,
}

pub struct VillagerInstance {
    archetype: VillagerArchetype,
    testimony: Option<Expression<Testimony>>,
    action_available: bool,
}

pub struct ConfirmedVillager {
    instance: VillagerInstance,
    true_identity: Option<VillagerArchetype>,
    corrupted: bool,
}

pub enum Villager {
    Revealed(RevealedVillager),
    Hidden(HiddenVillager),
    Confirmed(ConfirmedVillager),
}

impl VillagerArchetype {
    pub fn is_evil(&self) -> bool {
        match self {
            Self::GoodVillager(_) => false,
            Self::Outcast(outcast) => match outcast {
                Outcast::Drunk
                | Outcast::Wretch
                | Outcast::Bombardier
                | Outcast::Doppelganger
                | Outcast::PlagueDoctor => false,
            },
            Self::Minion(_) | Self::Demon(_) => true,
        }
    }

    pub fn lies(&self) -> bool {
        match self {
            Self::GoodVillager(_) => false,
            Self::Outcast(outcast) => match outcast {
                Outcast::Drunk => true,
                _ => false,
            },
            Self::Minion(minion) => match minion {
                Minion::Puppet => false,
                _ => true,
            },
            Self::Demon(_) => true,
        }
    }

    pub fn disguises(&self) -> bool {
        match self {
            Self::GoodVillager(_) => false,
            Self::Outcast(outcast) => match outcast {
                Outcast::Drunk => true,
                _ => false,
            },
            Self::Minion(_) | Self::Demon(_) => true,
        }
    }

    pub fn corrupted(&self) -> bool {
        match self {
            Self::GoodVillager(_) => false,
            Self::Outcast(outcast) => match outcast {
                Outcast::Drunk => true,
                _ => false,
            },
            Self::Minion(_) | Self::Demon(_) => false,
        }
    }

    pub fn affects(
        total_villagers: u8,
        index: VillagerIndex,
        hidden_villagers: &[VillagerIndex],
    ) -> Vec<Affect> {
        todo!()
    }
}

impl HiddenVillager {
    pub fn new(dead: bool, cant_reveal: bool) -> Self {
        Self { dead, cant_reveal }
    }
}
