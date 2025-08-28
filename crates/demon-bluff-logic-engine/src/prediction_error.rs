use thiserror::Error;

#[derive(Error, Debug)]
pub enum PredictionError {
	#[error("Evaluation could not determine an action to perform!")]
	ConclusiveNoAction,
}
