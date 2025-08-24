use std::fmt::Display;

use reveal_villager::RevealVillagerDesire;
use use_ability::UseAbilityDesire;

pub(crate) mod reveal_villager;
pub(crate) mod use_ability;

#[derive(PartialEq, Eq, Debug, Display, Clone)]
pub enum DesireType {
    RevealVillager(RevealVillagerDesire),
    UseAbility(UseAbilityDesire),
}
