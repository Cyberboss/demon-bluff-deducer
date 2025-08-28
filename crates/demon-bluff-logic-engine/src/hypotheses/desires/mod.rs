use std::fmt::Display;

pub use self::get_testimony::GetTestimonyDesire;
use crate::engine::Desire;

mod get_testimony;

#[derive(PartialEq, Eq, Debug, Clone)]
#[enum_delegate::implement(Desire)]
pub enum DesireType {
	GetTestimony(GetTestimonyDesire),
}

impl Display for DesireType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.describe(f)
	}
}
