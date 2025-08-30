#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Node {
	Hypothesis(usize, bool),
	Desire(usize),
}
