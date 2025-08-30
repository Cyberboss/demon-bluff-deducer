use crate::engine::HypothesisReference;

#[derive(Debug)]
pub enum HypothesisExpression {
	Unary(HypothesisReference),
	Not(HypothesisReference),
	And((HypothesisReference, HypothesisReference)),
	Or((HypothesisReference, HypothesisReference)),
}
