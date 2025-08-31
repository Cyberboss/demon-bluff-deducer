use log::{Log, info};

use super::{HypothesisFunctions, reference::HypothesisReference, result::HypothesisResult};
use crate::{
	Breakpoint,
	engine::{
		cycle::{clone_cycle, derive_from_full_cycle},
		hypothesis::invocation::HypothesisInvocation,
		index_reference::IndexReference,
		iteration_data::VisitState,
		stack_data::StackData,
	},
	hypotheses::{DesireType, HypothesisType},
};

/// Used to evaluate sub-hypotheses via their `HypothesisReference`s.
pub trait HypothesisEvaluator<'a, TLog, THypothesis, TDesire, FDebugBreak>:
	HypothesisFunctions
{
	fn sub_evaluate(&mut self, hypothesis_reference: &HypothesisReference) -> HypothesisResult;
}

impl<'a, TLog, FDebugBreak> HypothesisEvaluator<'a, TLog, HypothesisType, DesireType, FDebugBreak>
	for StackData<'a, TLog, HypothesisType, DesireType, FDebugBreak>
where
	TLog: Log,
	FDebugBreak: FnMut(Breakpoint) + Clone,
{
	fn sub_evaluate(&mut self, hypothesis_reference: &HypothesisReference) -> HypothesisResult {
		let current_reference = self.current_reference();

		let mut current_data = self.current_data.borrow_mut();
		let mut force_conclusive = false;
		if let Some(break_at) = self.break_at
			&& break_at == current_reference
		{
			info!(
				logger: self.log,
				"{} Want to evaluate {} but we are breaking the cycle",
				self.depth(),
				hypothesis_reference
			);

			let cycle: crate::engine::cycle::Cycle =
				self.create_cycle(hypothesis_reference, &mut current_data, false);
			let current_reference = current_reference.clone();
			if let Some(debugger) = &mut self.debugger {
				debugger.breakpoint(Breakpoint::BreakCycle(cycle, current_reference.index()));
			}

			force_conclusive = true;
		} else if let Some(previous_data) = self.previous_data
			&& let VisitState::Visited(HypothesisResult::Conclusive(previously_conclusive_result)) =
				&previous_data.results[hypothesis_reference.index()]
		{
			info!(logger: self.log, "{} Skipping previously concluded hypothesis: {}", self.depth(), hypothesis_reference);
			current_data.inner.results[hypothesis_reference.index()] = VisitState::Visited(
				HypothesisResult::Conclusive(previously_conclusive_result.clone()),
			);
		} else if let VisitState::Visited(_) =
			&current_data.inner.results[hypothesis_reference.index()]
		{
			info!(logger: self.log, "{} Skipping previously already evaluated this iteration: {}", self.depth(), hypothesis_reference);

			for full_cycle in &current_data.full_cycles[hypothesis_reference.index()] {
				let cycle = derive_from_full_cycle(
					full_cycle,
					self.reference_stack(),
					hypothesis_reference,
				);

				info!(
					logger: self.log,
					"{} Cycle detected when retracing paths under reference {}: {}",
					self.depth(),
					hypothesis_reference,
					cycle
				);

				if let Some(debugger) = &mut self.debugger {
					debugger.breakpoint(Breakpoint::DetectCycle(clone_cycle(&cycle)));
				}

				let mut cycles = self.cycles.borrow_mut();
				cycles.insert(cycle);
			}
		} else {
			match self.hypotheses[hypothesis_reference.index()].try_borrow_mut() {
				Ok(next_reference) => {
					// Important or entering the invocation will BorrowError
					drop(current_data);
					drop(next_reference);

					let mut invocation = self.push(hypothesis_reference.clone());

					return invocation.invoke();
				}
				Err(_) => {
					let cycle = self.create_cycle(hypothesis_reference, &mut current_data, true);
					info!(
						logger: self.log,
						"{} Cycle detected when trying to evaluate reference {}: {}",
						self.depth(),
						hypothesis_reference,
						cycle
					);

					if let Some(debugger) = &mut self.debugger {
						debugger.breakpoint(Breakpoint::DetectCycle(clone_cycle(&cycle)));
					}

					let mut cycles = self.cycles.borrow_mut();
					cycles.insert(cycle);
				}
			}
		}

		let current_results = &current_data.inner.results[hypothesis_reference.index()];
		let relevant_iteration_data = if let VisitState::Visited(rid) = current_results {
			rid
		} else if let VisitState::Visiting(rid) = current_results {
			rid
		} else if let VisitState::Visited(rid) = &self
			.previous_data
			.expect("We shouldn't be using cached fitness data if none exists")
			.results[hypothesis_reference.index()]
		{
			rid
		} else {
			panic!("Fitness for cycle break didn't previously exist")
		};

		let mut last_evaluate = relevant_iteration_data.clone();

		if force_conclusive {
			last_evaluate = HypothesisResult::Conclusive(match last_evaluate {
				HypothesisResult::Pending(fitness_and_action)
				| HypothesisResult::Conclusive(fitness_and_action) => fitness_and_action,
			})
		}

		info!(
			logger: self.log,
			"{} Using existing {} result: {}",
			self.depth(),
			hypothesis_reference,
			last_evaluate
		);

		last_evaluate
	}
}
