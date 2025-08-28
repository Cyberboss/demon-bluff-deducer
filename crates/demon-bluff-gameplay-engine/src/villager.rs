use std::fmt::Display;

use serde::Serialize;
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    Expression,
    affect::{Affect, NightEffect, VillagerAffect},
    testimony::{Direction, Testimony},
};

#[derive(Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord, Serialize)]
pub struct VillagerIndex(pub usize);

#[derive(Clone, Eq, PartialEq, Debug, Hash, Display, EnumIter, Serialize)]
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

#[derive(Clone, Eq, PartialEq, Debug, Hash, Display, EnumIter, Serialize)]
pub enum Outcast {
    Drunk,
    Wretch,
    Bombardier,
    Doppelganger,
    PlagueDoctor,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Display, EnumIter, Serialize)]
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

#[derive(Clone, Eq, PartialEq, Debug, Hash, Display, EnumIter, Serialize)]
pub enum Demon {
    Baa,
    Pooka,
    Lilis,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Display, Serialize)]
pub enum VillagerArchetype {
    GoodVillager(GoodVillager),
    Outcast(Outcast),
    Minion(Minion),
    Demon(Demon),
}

#[derive(Debug, Clone, Serialize)]
pub struct ActiveVillager {
    instance: VillagerInstance,
    cant_kill: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct HiddenVillager {
    dead: bool,
    cant_reveal: bool,
    cant_kill: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct VillagerInstance {
    archetype: VillagerArchetype,
    testimony: Option<Expression<Testimony>>,
    action_available: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConfirmedVillager {
    instance: VillagerInstance,
    true_identity: Option<VillagerArchetype>,
    corrupted: bool,
}

#[derive(Debug, Clone, Serialize)]
pub enum Villager {
    Active(ActiveVillager),
    Hidden(HiddenVillager),
    Confirmed(ConfirmedVillager),
}

pub enum ExecutionResult {
    EvilKilled,
    SelfDestructKilled,
    HealthDeduction(u8),
}

impl Display for VillagerIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.0 + 1)
    }
}

impl VillagerArchetype {
    pub fn iter() -> impl Iterator<Item = VillagerArchetype> {
        GoodVillager::iter()
            .map(|good| VillagerArchetype::GoodVillager(good))
            .chain(Outcast::iter().map(|outcast| VillagerArchetype::Outcast(outcast)))
            .chain(Minion::iter().map(|minion| VillagerArchetype::Minion(minion)))
            .chain(Demon::iter().map(|demon| VillagerArchetype::Demon(demon)))
    }

    pub fn is_evil(&self) -> bool {
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

    pub fn appears_evil(&self) -> bool {
        if VillagerArchetype::Outcast(Outcast::Wretch) == *self || self.is_evil() {
            true
        } else {
            false
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

    pub fn starts_corrupted(&self) -> bool {
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

    pub fn can_be_corrupted(&self) -> bool {
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
                | GoodVillager::Slayer => true,
            },
            Self::Outcast(outcast) => match outcast {
                Outcast::Drunk => true,
                Outcast::Wretch => false,
                Outcast::Bombardier => false,
                Outcast::Doppelganger => todo!("Can a Doppleganger be corrupted?"),
                Outcast::PlagueDoctor => todo!("Can a PlagueDoctor be corrupted?"),
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

    // this function only exists because of pupeteer so I'm taking the lazy route
    pub fn deck_prerequisite(&self) -> VillagerArchetype {
        match self {
            Self::GoodVillager(good_villager) => Self::GoodVillager(good_villager.clone()),
            Self::Outcast(outcast) => Self::Outcast(outcast.clone()),
            Self::Minion(minion) => match minion {
                Minion::Puppet => Self::Minion(Minion::Puppeteer),
                other => Self::Minion(other.clone()),
            },
            Self::Demon(demon) => Self::Demon(demon.clone()),
        }
    }

    /// index is only used to calculate positions of the [`VillagerAffect`] if any
    pub fn affect(&self, total_villagers: usize, index: Option<VillagerIndex>) -> Option<Affect> {
        let index = index.unwrap_or(VillagerIndex(0));
        match self {
            VillagerArchetype::GoodVillager(good_villager) => match good_villager {
                GoodVillager::Alchemist
                | GoodVillager::Architect
                | GoodVillager::Baker
                | GoodVillager::Bard
                | GoodVillager::Bishop
                | GoodVillager::Confessor
                | GoodVillager::Dreamer
                | GoodVillager::Druid
                | GoodVillager::Empress
                | GoodVillager::Enlightened
                | GoodVillager::FortuneTeller
                | GoodVillager::Gemcrafter
                | GoodVillager::Hunter
                | GoodVillager::Jester
                | GoodVillager::Judge
                | GoodVillager::Knight
                | GoodVillager::Knitter
                | GoodVillager::Lover
                | GoodVillager::Medium
                | GoodVillager::Oracle
                | GoodVillager::Poet
                | GoodVillager::Scout
                | GoodVillager::Slayer
                | GoodVillager::Witness => None,
            },
            VillagerArchetype::Outcast(outcast) => match outcast {
                Outcast::Drunk | Outcast::Wretch | Outcast::Bombardier | Outcast::Doppelganger => {
                    // A doppleganger is a disguise not a dupe
                    None
                }
                Outcast::PlagueDoctor => Expression::or_from_iterator(
                    other_indicies(&index, total_villagers).map(|other_index| {
                        VillagerAffect::from_index(&index, &other_index, total_villagers)
                    }),
                )
                .map(|expr| Affect::Corrupt(expr)),
            },
            VillagerArchetype::Minion(minion) => match minion {
                Minion::Counsellor => Some(Affect::Corrupt(Expression::Or(
                    Box::new(Expression::Unary(VillagerAffect::new(
                        Direction::Clockwise,
                        1,
                    ))),
                    Box::new(Expression::Unary(VillagerAffect::new(
                        Direction::CounterClockwise,
                        1,
                    ))),
                ))),
                Minion::Puppeteer => Some(Affect::Puppet(Expression::Or(
                    Box::new(Expression::Unary(VillagerAffect::new(
                        Direction::Clockwise,
                        1,
                    ))),
                    Box::new(Expression::Unary(VillagerAffect::new(
                        Direction::CounterClockwise,
                        1,
                    ))),
                ))),
                Minion::Shaman => Some(Affect::DupeVillager),
                Minion::Witch => Some(Affect::BlockLastNReveals(1)),
                Minion::Minion | Minion::Poisoner | Minion::Twinion | Minion::Puppet => None,
            },
            VillagerArchetype::Demon(demon) => Some(match demon {
                Demon::Baa => Affect::FakeOutcast,
                Demon::Pooka => Affect::Corrupt(Expression::And(
                    Box::new(Expression::Unary(VillagerAffect::new(
                        Direction::Clockwise,
                        1,
                    ))),
                    Box::new(Expression::Unary(VillagerAffect::new(
                        Direction::CounterClockwise,
                        1,
                    ))),
                )),
                Demon::Lilis => Affect::Night(NightEffect::KillUnrevealed),
            }),
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

    pub fn instance(&self) -> &VillagerInstance {
        &self.instance
    }

    pub fn instance_mut(&mut self) -> &mut VillagerInstance {
        &mut self.instance
    }

    pub fn cant_kill(&self) -> bool {
        self.cant_kill
    }

    pub fn set_cant_kill(&mut self) {
        self.cant_kill = true;
    }
}

impl HiddenVillager {
    pub fn new(dead: bool, cant_reveal: bool, cant_kill: bool) -> Self {
        Self {
            dead,
            cant_reveal,
            cant_kill,
        }
    }

    pub fn die(&mut self) {
        self.dead = true;
    }

    pub fn dead(&self) -> bool {
        self.dead
    }

    pub fn cant_reveal(&self) -> bool {
        self.cant_reveal || self.dead
    }

    pub fn set_cant_reveal(&mut self) {
        self.cant_reveal = true;
    }

    pub fn reset_cant_reveal(&mut self) {
        self.cant_reveal = false;
    }

    pub fn cant_kill(&self) -> bool {
        self.cant_kill
    }

    pub fn set_cant_kill(&mut self) {
        self.cant_kill = true;
    }
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

    pub fn archetype(&self) -> &VillagerArchetype {
        &self.archetype
    }

    pub fn action_available(&self) -> bool {
        self.action_available
    }

    pub fn testimony(&self) -> &Option<Expression<Testimony>> {
        &self.testimony
    }

    pub fn set_testimony(&mut self, testimony: Expression<Testimony>) {
        self.testimony = Some(testimony);
    }
}

impl ConfirmedVillager {
    pub fn new(
        instance: VillagerInstance,
        true_identity: Option<VillagerArchetype>,
        corrupted: bool,
    ) -> Self {
        Self {
            instance,
            true_identity,
            corrupted,
        }
    }

    pub fn true_identity(&self) -> &VillagerArchetype {
        self.true_identity
            .as_ref()
            .unwrap_or(&self.instance.archetype)
    }

    pub fn corrupted(&self) -> bool {
        self.corrupted
    }

    pub fn lies(&self) -> bool {
        self.true_identity().lies() || self.corrupted
    }

    pub fn instance(&self) -> &VillagerInstance {
        &self.instance
    }

    pub fn instance_mut(&mut self) -> &mut VillagerInstance {
        &mut self.instance
    }

    pub fn execution_result(&self) -> ExecutionResult {
        match self.true_identity() {
            VillagerArchetype::GoodVillager(good_villager) => match good_villager {
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
                | GoodVillager::Slayer => ExecutionResult::HealthDeduction(5),
            },
            VillagerArchetype::Outcast(outcast) => match outcast {
                Outcast::Drunk => ExecutionResult::HealthDeduction(2), // I don't know if it's a bug or what, but its what happens despite what their card says
                Outcast::Wretch | Outcast::Doppelganger | Outcast::PlagueDoctor => {
                    ExecutionResult::HealthDeduction(5)
                }
                Outcast::Bombardier => ExecutionResult::SelfDestructKilled,
            },
            VillagerArchetype::Minion(minion) => match minion {
                Minion::Counsellor
                | Minion::Witch
                | Minion::Minion
                | Minion::Poisoner
                | Minion::Twinion
                | Minion::Shaman
                | Minion::Puppeteer
                | Minion::Puppet => ExecutionResult::EvilKilled,
            },
            VillagerArchetype::Demon(demon) => match demon {
                Demon::Baa | Demon::Pooka | Demon::Lilis => ExecutionResult::EvilKilled,
            },
        }
    }
}

pub gen fn other_indicies(index: &VillagerIndex, total_villagers: usize) -> VillagerIndex {
    for other_index in 0..total_villagers {
        if index.0 != other_index {
            yield VillagerIndex(other_index)
        }
    }
}
