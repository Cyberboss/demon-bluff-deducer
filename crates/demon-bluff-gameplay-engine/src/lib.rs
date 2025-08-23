use std::fmt::{Display, write};

#[macro_use]
extern crate enum_display_derive;

pub mod affect;
pub mod game_state;
pub mod testimony;
pub mod villager;

pub const VILLAGERS_MIN: usize = 7;
pub const VILLAGERS_MAX: usize = 9;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression<Type>
where
    Type: Display,
{
    Unary(Type),
    Not(Box<Expression<Type>>),
    And(Box<Expression<Type>>, Box<Expression<Type>>),
    Or(Box<Expression<Type>>, Box<Expression<Type>>),
}

impl<Type> Display for Expression<Type>
where
    Type: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Unary(item) => write!(f, "{}", item),
            Expression::Not(expression) => write!(f, "!({})", expression),
            Expression::And(lhs, rhs) => write!(f, "({} && {})", lhs, rhs),
            Expression::Or(lhs, rhs) => write!(f, "({} || {})", lhs, rhs),
        }
    }
}
