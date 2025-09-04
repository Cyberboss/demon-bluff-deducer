use std::{cell::RefCell, collections::HashSet, vec};

use debugger::Debugger;
use demon_bluff_gameplay_engine::game_state::GameState;
use iteration_data::{CurrentIterationData, VisitState};
use log::{Log, error, info};

use self::{
	cycle::Cycle,
	desire::DesireData,
	hypothesis::{HypothesisInvocation, HypothesisRegistrarImpl},
	index_reference::IndexReference,
	iteration_data::IterationData,
	stack_data::StackData,
};
pub use self::{
	debugger::{Breakpoint, DebuggerContext, DesireNode, HypothesisNode},
	depth::Depth,
	desire::{Desire, DesireConsumerReference, DesireProducerReference},
	fitness_and_action::{
		FITNESS_UNIMPLEMENTED, FITNESS_UNKNOWN, FitnessAndAction, and_fitness, and_result,
		decide_result, not_result, or_result, sum_result,
	},
	hypothesis::{
		Hypothesis, HypothesisBuilder, HypothesisEvaluation, HypothesisEvaluator,
		HypothesisFunctions, HypothesisReference, HypothesisRegistrar, HypothesisRepository,
		HypothesisResult,
	},
};
use crate::{
	PredictionError,
	hypotheses::{DesireType, HypothesisBuilderType, HypothesisType},
	player_action::PlayerAction,
};

mod cycle;
mod debugger;
mod dependencies;
mod depth;
mod desire;
mod fitness_and_action;
mod hypothesis;
mod index_reference;
mod iteration_data;
mod stack_data;

const ITERATIONS_BEFORE_GRAPH_ASSUMED_STABLE: u32 = 100;

pub fn evaluate<TBuilder, TLog, F>(
	game_state: &GameState,
	initial_hypothesis_builder: TBuilder,
	log: &TLog,
	breakpoint_handler: Option<F>,
) -> Result<HashSet<PlayerAction>, PredictionError>
where
	TBuilder: HypothesisBuilder,
	HypothesisBuilderType: From<TBuilder>,
	TLog: Log,
	F: FnMut(Breakpoint) + Clone,
{
	let mut debugger = breakpoint_handler.map(|breaker| Debugger::new(breaker));

	let registrar = HypothesisRegistrarImpl::<TLog, HypothesisBuilderType, DesireType>::new(log);

	info!(logger: log, target: "evaluate", "Evaluate dependencies");
	let graph = registrar.run(game_state, initial_hypothesis_builder, debugger.as_mut());

	info!(logger: log, target: "evaluate", "Registered {} hypotheses. Root: {}", graph.hypotheses.len(), graph.hypotheses[graph.root.index()]);

	let hypotheses: Vec<RefCell<HypothesisType>> =
		graph.hypotheses.into_iter().map(RefCell::new).collect();

	let mut previous_results = None;

	let mut break_at = None;

	let mut iteration = 0;
	let mut stability_iteration = 0;
	let mut desire_data_vec = graph
		.desires
		.iter()
		.map(|desire_definition| DesireData {
			pending: HashSet::with_capacity(desire_definition.count()),
			desired: HashSet::with_capacity(desire_definition.count()),
			undesired: HashSet::with_capacity(desire_definition.count()),
		})
		.collect::<Vec<DesireData>>();
	{
		for (index, producer_dependencies) in graph.dependencies.desire_producers.iter().enumerate()
		{
			for desire_reference in producer_dependencies {
				let reference = HypothesisReference::new(index);
				desire_data_vec[desire_reference.index()]
					.pending
					.insert(reference);
			}
		}
	}

	let desire_data = RefCell::new(desire_data_vec);

	loop {
		log.flush();
		iteration += 1;
		info!(logger: log, "Iteration: {iteration}");

		if let Some(debugger) = &mut debugger {
			debugger.breakpoint(Breakpoint::IterationStart(iteration));
		}

		let mut data = Vec::with_capacity(hypotheses.len());
		for _ in 0..hypotheses.len() {
			data.push(VisitState::Unvisited);
		}

		let data = RefCell::new(CurrentIterationData {
			inner: IterationData { results: data },
			full_cycles: vec![Vec::new(); hypotheses.len()],
		});
		let cycles = RefCell::new(HashSet::new());
		let mut stack_data = StackData::new(
			game_state,
			log,
			&hypotheses,
			&cycles,
			previous_results.as_ref(),
			&data,
			&break_at,
			&graph.root,
			debugger.clone(),
			&graph.desires,
			&desire_data,
			&graph.dependencies,
		);

		let result = stack_data.invoke();

		break_at = None;

		match result {
			HypothesisResult::Pending(fitness_and_action) => {
				info!(logger: log, "Pending result. Fitness: {fitness_and_action}");

				let data = data.borrow();
				if let Some(previous_results) = &previous_results {
					let mut graph_stable = *previous_results == data.inner;

					stability_iteration += 1;
					if !graph_stable {
						if stability_iteration >= ITERATIONS_BEFORE_GRAPH_ASSUMED_STABLE {
							info!(logger: log, "Graph not stable after {ITERATIONS_BEFORE_GRAPH_ASSUMED_STABLE} iterations, assuming stable enough for progression");
							graph_stable = true;
						}
					} else {
						info!(logger: log, "Graph stable")
					}

					if graph_stable {
						stability_iteration = 0;

						let cycles = cycles.borrow();
						if cycles.is_empty() {
							let mut borrow = desire_data.borrow_mut();
							info!(
								"I-{}: We have a stagnate graph due to one of the following desires not concluding. We will forcefully set the hypotheses that are not evaluating it to false {}",
								iteration,
								borrow
									.iter()
									.filter(|desire| !desire.pending.is_empty())
									.enumerate()
									.map(|(index, data)| {
										format!("{}: {}", DesireProducerReference::new(index), data)
									})
									.collect::<Vec<String>>()
									.join(", ")
							);

							let mut least_pending_option = None;
							for (index, desire_data) in borrow
								.iter_mut()
								.filter(|desire| !desire.pending.is_empty())
								.enumerate()
							{
								least_pending_option = Some(match least_pending_option {
									Some((previous_index, previous_pending)) => {
										if previous_pending > desire_data.pending.len() {
											(index, desire_data.pending.len())
										} else {
											(previous_index, previous_pending)
										}
									}
									None => (index, desire_data.pending.len()),
								})
							}

							let (least_pending_index, _) = least_pending_option
								.expect("At least one stagnate desire should have been found!");
							info!(logger: log, "Selected {} for unblocking", DesireProducerReference::new(least_pending_index));
							let unblocking_data = &mut borrow[least_pending_index];
							for pending_hypothesis in std::mem::take(&mut unblocking_data.pending) {
								info!(logger: log, "Force setting {pending_hypothesis}'s desire to false");
								unblocking_data.undesired.insert(pending_hypothesis);
							}

							if let Some(debugger) = &mut debugger {
								debugger
									.breakpoint(Breakpoint::CollapseDesire(least_pending_index));
							}
						} else {
							info!(logger: log, "I-{}: We must break a cycle, of which there are {}",
                                iteration,cycles.len());

							let mut best_break_candidate =
								None::<(&Cycle, &HypothesisReference, f64)>;

							for cycle in cycles.iter() {
								for reference in cycle.references() {
									let fitness = match &data.inner.results[reference.index()] {
										VisitState::Unvisited => panic!(
											"A hypothesis in a cycle should have SOME result"
										),
										VisitState::Visiting(hypothesis_result)
										| VisitState::Visited(hypothesis_result) => {
											hypothesis_result.fitness_and_action().fitness()
										}
									};

									best_break_candidate = Some(match best_break_candidate {
										Some((
											previous_cycle,
											previous_reference,
											previous_fitness,
										)) => {
											if previous_fitness > fitness {
												(
													previous_cycle,
													previous_reference,
													previous_fitness,
												)
											} else if fitness > previous_fitness {
												(cycle, reference, fitness)
											} else {
												// break shortest fittest candidate cycle first for simplicity
												if cycle.references().len()
													< previous_cycle.references().len()
												{
													(cycle, reference, fitness)
												} else {
													(
														previous_cycle,
														previous_reference,
														previous_fitness,
													)
												}
											}
										}
										None => (cycle, reference, fitness),
									});
								}
							}

							let (break_cycle, break_reference, break_fitness) =
								best_break_candidate
									.expect("At least one break candidate should exist");
							info!(logger: log, "Breaking cycle {break_cycle} at {break_reference} which has a pending fitness value of {break_fitness}");
							break_at = Some(break_reference.clone());
						}
					}
				}

				previous_results = Some(data.inner.clone());
			}
			HypothesisResult::Conclusive(fitness_and_action) => {
				if fitness_and_action.action().is_empty() {
					error!(logger: log, "Obtained conclusive result with no actions!");
					return Err(PredictionError::ConclusiveNoAction);
				}

				info!(logger: log, "Conclusive result obtained. Fitness: {fitness_and_action}");

				if fitness_and_action.action().len() == 1 {
					for action in fitness_and_action.action() {
						info!(logger: log, "Conclusive action: {action}");
					}
				}

				return Ok(fitness_and_action.action().clone());
			}
		}
	}
}
