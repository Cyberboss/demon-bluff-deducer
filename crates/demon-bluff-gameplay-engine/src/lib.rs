#[macro_use]
extern crate enum_display_derive;

pub mod affect;
pub mod game_state;
pub mod testimony;
pub mod villager;

pub const VILLAGERS_MIN: usize = 7;
pub const VILLAGERS_MAX: usize = 9;

#[derive(Clone, Debug)]
pub enum Expression<Type> {
    Unary(Type),
    Not(Box<Expression<Type>>),
    And(Box<Expression<Type>>, Box<Expression<Type>>),
    Or(Box<Expression<Type>>, Box<Expression<Type>>),
}

/*
impl<Type> PartialEq for Expression<Type>
where
    Type: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Unary(l0), Self::Unary(r0)) => l0 == r0,
            _ => self.tree_equals(other),
        }
    }
}

impl<Type> Expression<Type>
where
    Type: Eq,
{
    fn collect_unique_types<'a>(&self, mut unique_types: Vec<&'a Type>) -> Vec<&'a Type> {
        match self {
            Self::Unary(inner) => {
                for unique_type in unique_types.iter() {
                    if inner.eq(*unique_type) {
                        return unique_types;
                    }
                }

                unique_types.push(inner);
            }
            Self::Not(expression) => todo!(),
            Self::And(expression, expression1) => todo!(),
            Self::Or(expression, expression1) => todo!(),
        }

        unique_types
    }
}
*/
