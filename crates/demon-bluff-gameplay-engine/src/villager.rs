use crate::{
    Expression,
    affect::Affect,
    testimony::{self, Testimony},
};

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

pub struct ActiveVillager {
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
    Active(ActiveVillager),
    Hidden(HiddenVillager),
    Confirmed(ConfirmedVillager),
}

impl VillagerInstance {
    pub fn new(archetype: VillagerArchetype, testimony: Option<Expression<Testimony>>) -> Self {
        let action_available = archetype.has_action();
        Self {
            archetype,
            testimony,
            action_available,
        }
    }
}

impl ActiveVillager {
    pub fn new(instance: VillagerInstance) -> Self {
        Self {
            instance,
            cant_kill: false,
        }
    }
}

impl HiddenVillager {
    pub fn cant_reveal(&self) -> bool {
        self.cant_reveal
    }
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
            Self::GoodVillager(good_villager) => match good_villager {
                GoodVillager::Alchemist
                | GoodVillager::Architect
                | GoodVillager::Baker
                | GoodVillager::Bishop
                | GoodVillager::Confessor
                | GoodVillager::Empress
                | GoodVillager::Enlightened
                | GoodVillager::Gemcrafter
                | GoodVillager::Hunter
                | GoodVillager::Knight
                | GoodVillager::Knitter
                | GoodVillager::Lover
                | GoodVillager::Medium
                | GoodVillager::Oracle
                | GoodVillager::Poet
                | GoodVillager::Scout
                | GoodVillager::Witness
                | GoodVillager::Bard
                | GoodVillager::Dreamer
                | GoodVillager::Druid
                | GoodVillager::FortuneTeller
                | GoodVillager::Jester
                | GoodVillager::Judge
                | GoodVillager::Slayer => false,
            },
            Self::Outcast(outcast) => match outcast {
                Outcast::Drunk => true,
                Outcast::Wretch
                | Outcast::Bombardier
                | Outcast::Doppelganger
                | Outcast::PlagueDoctor => false,
            },
            Self::Minion(minion) => match minion {
                Minion::Puppet => false,
                Minion::Counsellor
                | Minion::Witch
                | Minion::Minion
                | Minion::Poisoner
                | Minion::Twinion
                | Minion::Shaman
                | Minion::Puppeteer => true,
            },
            Self::Demon(_) => true,
        }
    }

    pub fn disguises(&self) -> bool {
        match self {
            Self::GoodVillager(good_villager) => match good_villager {
                GoodVillager::Alchemist
                | GoodVillager::Architect
                | GoodVillager::Baker
                | GoodVillager::Bishop
                | GoodVillager::Confessor
                | GoodVillager::Empress
                | GoodVillager::Enlightened
                | GoodVillager::Gemcrafter
                | GoodVillager::Hunter
                | GoodVillager::Knight
                | GoodVillager::Knitter
                | GoodVillager::Lover
                | GoodVillager::Medium
                | GoodVillager::Oracle
                | GoodVillager::Poet
                | GoodVillager::Scout
                | GoodVillager::Witness
                | GoodVillager::Bard
                | GoodVillager::Dreamer
                | GoodVillager::Druid
                | GoodVillager::FortuneTeller
                | GoodVillager::Jester
                | GoodVillager::Judge
                | GoodVillager::Slayer => false,
            },
            Self::Outcast(outcast) => match outcast {
                Outcast::Drunk => true,
                Outcast::Wretch
                | Outcast::Bombardier
                | Outcast::Doppelganger
                | Outcast::PlagueDoctor => false,
            },
            Self::Minion(minion) => match minion {
                Minion::Counsellor
                | Minion::Witch
                | Minion::Minion
                | Minion::Poisoner
                | Minion::Twinion
                | Minion::Shaman
                | Minion::Puppeteer
                | Minion::Puppet => true,
            },
            Self::Demon(demon) => match demon {
                Demon::Baa | Demon::Pooka | Demon::Lilis => true,
            },
        }
    }

    pub fn corrupted(&self) -> bool {
        match self {
            Self::GoodVillager(good_villager) => match good_villager {
                GoodVillager::Alchemist
                | GoodVillager::Architect
                | GoodVillager::Baker
                | GoodVillager::Bishop
                | GoodVillager::Confessor
                | GoodVillager::Empress
                | GoodVillager::Enlightened
                | GoodVillager::Gemcrafter
                | GoodVillager::Hunter
                | GoodVillager::Knight
                | GoodVillager::Knitter
                | GoodVillager::Lover
                | GoodVillager::Medium
                | GoodVillager::Oracle
                | GoodVillager::Poet
                | GoodVillager::Scout
                | GoodVillager::Witness
                | GoodVillager::Bard
                | GoodVillager::Dreamer
                | GoodVillager::Druid
                | GoodVillager::FortuneTeller
                | GoodVillager::Jester
                | GoodVillager::Judge
                | GoodVillager::Slayer => false,
            },
            Self::Outcast(outcast) => match outcast {
                Outcast::Drunk => true,
                Outcast::Wretch
                | Outcast::Bombardier
                | Outcast::Doppelganger
                | Outcast::PlagueDoctor => false,
            },
            Self::Minion(minion) => match minion {
                Minion::Counsellor
                | Minion::Witch
                | Minion::Minion
                | Minion::Poisoner
                | Minion::Twinion
                | Minion::Shaman
                | Minion::Puppeteer
                | Minion::Puppet => false,
            },
            Self::Demon(demon) => match demon {
                Demon::Baa | Demon::Pooka | Demon::Lilis => false,
            },
        }
    }

    pub fn has_night_action(&self) -> bool {
        match self {
            Self::GoodVillager(good_villager) => match good_villager {
                GoodVillager::Alchemist
                | GoodVillager::Architect
                | GoodVillager::Baker
                | GoodVillager::Bishop
                | GoodVillager::Confessor
                | GoodVillager::Empress
                | GoodVillager::Enlightened
                | GoodVillager::Gemcrafter
                | GoodVillager::Hunter
                | GoodVillager::Knight
                | GoodVillager::Knitter
                | GoodVillager::Lover
                | GoodVillager::Medium
                | GoodVillager::Oracle
                | GoodVillager::Poet
                | GoodVillager::Scout
                | GoodVillager::Witness
                | GoodVillager::Bard
                | GoodVillager::Dreamer
                | GoodVillager::Druid
                | GoodVillager::FortuneTeller
                | GoodVillager::Jester
                | GoodVillager::Judge
                | GoodVillager::Slayer => false,
            },
            Self::Outcast(outcast) => match outcast {
                Outcast::Drunk
                | Outcast::Wretch
                | Outcast::Bombardier
                | Outcast::Doppelganger
                | Outcast::PlagueDoctor => false,
            },
            Self::Demon(demon) => match demon {
                Demon::Lilis => true,
                Demon::Baa | Demon::Pooka => false,
            },
            Self::Minion(minion) => match minion {
                Minion::Counsellor
                | Minion::Witch
                | Minion::Minion
                | Minion::Poisoner
                | Minion::Twinion
                | Minion::Shaman
                | Minion::Puppeteer
                | Minion::Puppet => false,
            },
        }
    }

    pub fn has_action(&self) -> bool {
        match self {
            Self::GoodVillager(good_villager) => match good_villager {
                GoodVillager::Alchemist
                | GoodVillager::Architect
                | GoodVillager::Baker
                | GoodVillager::Bishop
                | GoodVillager::Confessor
                | GoodVillager::Empress
                | GoodVillager::Enlightened
                | GoodVillager::Gemcrafter
                | GoodVillager::Hunter
                | GoodVillager::Knight
                | GoodVillager::Knitter
                | GoodVillager::Lover
                | GoodVillager::Medium
                | GoodVillager::Oracle
                | GoodVillager::Poet
                | GoodVillager::Scout
                | GoodVillager::Witness => false,
                GoodVillager::Bard
                | GoodVillager::Dreamer
                | GoodVillager::Druid
                | GoodVillager::FortuneTeller
                | GoodVillager::Jester
                | GoodVillager::Judge
                | GoodVillager::Slayer => true,
            },
            Self::Outcast(outcast) => match outcast {
                Outcast::Drunk | Outcast::Wretch | Outcast::Bombardier | Outcast::Doppelganger => {
                    false
                }
                Outcast::PlagueDoctor => true,
            },
            Self::Demon(demon) => match demon {
                Demon::Lilis | Demon::Baa | Demon::Pooka => false,
            },
            Self::Minion(minion) => match minion {
                Minion::Counsellor
                | Minion::Witch
                | Minion::Minion
                | Minion::Poisoner
                | Minion::Twinion
                | Minion::Shaman
                | Minion::Puppeteer
                | Minion::Puppet => false,
            },
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
