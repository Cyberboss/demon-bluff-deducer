use log::{Log, info};

use super::{Hypothesis, HypothesisResult};
use crate::{
	Breakpoint,
	engine::{
		HypothesisRepository,
		debugger::{hypothesis_nodes_mut, update_hypothesis_node},
		index_reference::IndexReference,
		iteration_data::VisitState,
		stack_data::StackData,
	},
	hypotheses::{DesireType, HypothesisType},
};

pub trait HypothesisInvocation {
	fn invoke(&mut self) -> HypothesisResult;
}

impl<'a, TLog, FDebugBreak> HypothesisInvocation
	for StackData<'a, TLog, HypothesisType, DesireType, FDebugBreak>
where
	TLog: Log,
	FDebugBreak: FnMut(Breakpoint) + Clone,
{
	fn invoke(&mut self) -> HypothesisResult {
		let reference = self.current_reference().clone();

		let mut hypothesis = self.hypotheses[reference.index()].borrow_mut();

		info!(logger: self.log, "{} Entering: {}", self.depth(), hypothesis);

		if let Some(debugger) = &mut self.debugger {
			debugger.breakpoint(Breakpoint::EnterHypothesis(reference.index()));
		}

		let hypo_return = hypothesis.evaluate(
			self.log,
			self.depth(),
			self.game_state,
			HypothesisRepository::new(self.share()),
		);

		let result = hypo_return.unpack();
		if let Some(debugger) = &mut self.debugger {
			let mut guard = debugger.context();
			update_hypothesis_node(
				&mut hypothesis_nodes_mut(&mut guard)[reference.index()],
				&result,
			);
			drop(guard);
			debugger.breakpoint(Breakpoint::ExitHypothesis(reference.index()));
		}

		info!(logger: self.log, "{} Result: {}", self.depth(), result);

		if let HypothesisResult::Conclusive(_) = &result {
			for producer_reference in &self.dependencies.desire_producers[reference.index()] {
				let desire_data = self.desire_data.borrow();
				if desire_data[producer_reference.index()]
					.pending
					.iter()
					.any(|pending_reference| *pending_reference == reference)
				{
					panic!(
						"{reference}: {hypothesis} was supposed to produce a result for {producer_reference} before concluding but didn't!"
					)
				}
			}
		}

		let mut current_data = self.current_data.borrow_mut();
		current_data.inner.results[self.current_reference().index()] =
			VisitState::Visited(result.clone());

		result
	}
}
