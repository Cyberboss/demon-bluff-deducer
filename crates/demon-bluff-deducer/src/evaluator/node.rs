#[derive(Debug, PartialEq, Eq)]
pub enum Node {
	Hypothesis(usize),
	Desire(usize),
}
