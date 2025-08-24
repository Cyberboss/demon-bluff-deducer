use std::fmt::Display;

use get_testimony::GetTestimonyDesire;

pub(crate) mod get_testimony;

#[derive(PartialEq, Eq, Debug, Display, Clone)]
pub enum DesireType {
    GetTestimony(GetTestimonyDesire),
}
