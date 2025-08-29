#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Node {
	Hypothesis(usize),
	Desire(usize),
}
