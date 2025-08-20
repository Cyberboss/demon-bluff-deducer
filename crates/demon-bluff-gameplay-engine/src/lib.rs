pub mod affect;
pub mod game_state;
pub mod testimony;
pub mod villager;

#[derive(Clone, Debug)]
pub enum Expression<Type> {
    Unary(Type),
    Not(Box<Expression<Type>>),
    And(Box<Expression<Type>>, Box<Expression<Type>>),
    Or(Box<Expression<Type>>, Box<Expression<Type>>),
}
