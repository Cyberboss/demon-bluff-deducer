use thiserror::Error;

#[derive(Error, Debug)]
pub enum PredictionError {
	#[error("Evaluation could not determine an action to perform!")]
	ConclusiveNoAction,
	#[error(
		"The SAT solver could not find a solution to the game based on available permutations!"
	)]
	GameUnsolvable,
}
