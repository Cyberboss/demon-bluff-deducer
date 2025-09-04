use log::{Log, info};

use super::{HypothesisEvaluator, HypothesisResult};
use crate::{
	Breakpoint,
	engine::{
		fitness_and_action::FitnessAndAction, index_reference::IndexReference,
		iteration_data::VisitState, stack_data::StackData,
	},
	hypotheses::{DesireType, HypothesisType},
};

/// A repository of hypotheses available to a single `Hypothesis` during evaluation.
pub struct HypothesisRepository<'a, TLog, FDebugBreak>
where
	TLog: Log,
	FDebugBreak: FnMut(Breakpoint) + Clone,
{
	pub(super) stack_data: StackData<'a, TLog, HypothesisType, DesireType, FDebugBreak>,
}

impl<'a, TLog, FDebugBreak> HypothesisRepository<'a, TLog, FDebugBreak>
where
	TLog: Log,
	FDebugBreak: FnMut(Breakpoint) + Clone,
{
	pub(in crate::engine) fn new(
		stack_data: StackData<'a, TLog, HypothesisType, DesireType, FDebugBreak>,
	) -> Self {
		Self { stack_data }
	}

	/// If a hypothesis has dependencies
	pub fn require_sub_evaluation(
		self,
		initial_fitness: f64,
	) -> impl HypothesisEvaluator<'a, TLog, HypothesisType, DesireType, FDebugBreak> {
		let mut data = self.stack_data.current_data.borrow_mut();
		match &data.inner.results[self.stack_data.current_reference().index()] {
			VisitState::Visited(_) | VisitState::Visiting(_) => {
				panic!("We shouldn't be revisiting hypotheses!")
			}
			VisitState::Unvisited => {
				if let Some(previous) = self.stack_data.previous_data
					&& let VisitState::Visited(_) =
						&previous.results[self.stack_data.current_reference().index()]
				{
				} else {
					info!(logger: self.stack_data.log, "{} Set initial fitness: {}",self.stack_data.depth(), initial_fitness);
				}

				data.inner.results[self.stack_data.current_reference().index()] =
					VisitState::Visiting(HypothesisResult::Pending(FitnessAndAction::new(
						initial_fitness,
						None,
					)));
			}
		}

		self.stack_data
	}
}
