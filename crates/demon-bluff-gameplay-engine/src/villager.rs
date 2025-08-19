use crate::{affect::Affect, testimony::Testimony};

pub struct VillagerIndex(pub usize);

pub enum GoodVillager {
    FortuneTeller,
    Bishop,
    Empress,
    Architect,
    Oracle,
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
    index: VillagerIndex,
}

pub struct HiddenVillager {
    dead: bool,
}

pub struct VillagerInstance {
    archetype: VillagerArchetype,
    testimony: Option<Expression<Testimony>>,
}

pub struct ConfirmedVillager {
    identity: VillagerInstance,
    disguise: VillagerInstance,
    corrupted: bool,
}

pub enum Expression<Type> {
    Unary(Type),
    Not(Box<Expression<Type>>),
    And(Box<Expression<Type>>, Box<Expression<Type>>),
    Or(Box<Expression<Type>>, Box<Expression<Type>>),
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

    pub fn affects(
        total_villagers: u8,
        index: VillagerIndex,
        hidden_villagers: &[VillagerIndex],
    ) -> Vec<Affect> {
        todo!()
    }
}

impl HiddenVillager {
    pub fn new(dead: bool) -> Self {
        Self { dead }
    }
}
