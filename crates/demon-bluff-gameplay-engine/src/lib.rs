#![feature(gen_blocks)]

use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[macro_use]
extern crate enum_display_derive;

pub mod affect;
pub mod game_state;
pub mod testimony;
pub mod villager;

pub const VILLAGERS_MIN: usize = 7;
pub const VILLAGERS_MAX: usize = 9;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Expression<Type>
where
	Type: Display,
{
	Leaf(Type),
	And(Box<Expression<Type>>, Box<Expression<Type>>),
	Or(Box<Expression<Type>>, Box<Expression<Type>>),
}

impl<Type> Display for Expression<Type>
where
	Type: Display,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Expression::Leaf(item) => write!(f, "{item}"),
			Expression::And(lhs, rhs) => write!(f, "({lhs}) && ({rhs})"),
			Expression::Or(lhs, rhs) => write!(f, "({lhs}) || ({rhs})"),
		}
	}
}

impl<Type> Expression<Type>
where
	Type: Display,
{
	pub fn or_from_iterator(iterator: impl Iterator<Item = Type>) -> Option<Self> {
		let mut expr = None;
		for item in iterator {
			let unary_expression = Expression::Leaf(item);
			expr = Some(match expr {
				Some(expr) => Expression::Or(Box::new(expr), Box::new(unary_expression)),
				None => unary_expression,
			});
		}

		expr
	}

	pub fn and_from_iterator(iterator: impl Iterator<Item = Type>) -> Option<Self> {
		let mut expr = None;
		for item in iterator {
			let unary_expression = Expression::Leaf(item);
			expr = Some(match expr {
				Some(expr) => Expression::And(Box::new(expr), Box::new(unary_expression)),
				None => unary_expression,
			});
		}

		expr
	}
}
