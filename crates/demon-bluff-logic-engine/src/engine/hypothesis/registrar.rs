use std::sync::{Arc, Mutex};

use demon_bluff_gameplay_engine::game_state::GameState;
use log::{Log, info};

use super::{HypothesisReference, graph_data::HypothesisGraphData};
use crate::engine::HypothesisBuilder;
use crate::{
    Breakpoint, DebuggerContext, Node,
    engine::{
        DesireConsumerReference, DesireProducerReference,
        debugger::{create_hypothesis_node, nodes_mut},
        dependencies::DependencyData,
        desire::{Desire, DesireDefinition},
        index_reference::IndexReference,
    },
    hypotheses::{DesireType, HypothesisBuilderType, HypothesisType},
};

pub trait HypothesisRegistrar<THypothesisBuilder, TDesire>
where
    THypothesisBuilder: HypothesisBuilder,
{
    fn register<THypothesisBuilderImpl>(
        &mut self,
        builder: THypothesisBuilderImpl,
    ) -> HypothesisReference
    where
        THypothesisBuilder: From<THypothesisBuilderImpl>;
    fn register_builder_type(&mut self, builder: THypothesisBuilder) -> HypothesisReference;
    fn register_desire_consumer(&mut self, desire: TDesire) -> DesireConsumerReference;
    fn register_desire_producer(&mut self, desire: TDesire) -> DesireProducerReference;
}

pub struct HypothesisRegistrarImpl<'a, TLog, THypothesisBuilder, TDesire>
where
    TLog: Log,
    THypothesisBuilder: HypothesisBuilder + Clone,
    TDesire: Desire,
{
    log: &'a TLog,
    builders: Vec<THypothesisBuilder>,
    desires: Vec<TDesire>,
    dependencies: Option<DependencyData>,
}

impl<'a, TLog> HypothesisRegistrarImpl<'a, TLog, HypothesisBuilderType, DesireType>
where
    TLog: Log,
{
    pub fn new(log: &'a TLog) -> Self {
        Self {
            log,
            builders: Vec::new(),
            dependencies: Some(DependencyData::default()),
            desires: Vec::new(),
        }
    }

    fn register_desire_core(&mut self, desire: DesireType) -> usize {
        for (index, existing_desire) in self.desires.iter().enumerate() {
            if desire == *existing_desire {
                return index;
            }
        }

        let reference = self.desires.len();
        self.desires.push(desire);
        reference
    }

    pub(in crate::engine) fn run<THypothesisBuilderImpl, FDebugBreak>(
        mut self,
        game_state: &GameState,
        builder: THypothesisBuilderImpl,
        debugger: Option<&mut (Arc<Mutex<DebuggerContext>>, FDebugBreak)>,
    ) -> HypothesisGraphData<HypothesisType, DesireType>
    where
        HypothesisBuilderType: From<THypothesisBuilderImpl>,
        FDebugBreak: FnMut(Breakpoint),
    {
        let mut current_reference = self.builders.len();
        let root_reference = HypothesisReference::new(current_reference);
        self.builders.push(builder.into());

        info!(logger: self.log, "Registering hypotheses builders");
        loop {
            let current_builder = self.builders[current_reference].clone();

            let dependency_data = self
                .dependencies
                .as_mut()
                .expect("Dependencies should exist");

            dependency_data.hypotheses.push(Vec::new());
            dependency_data.desire_consumers.push(Vec::new());
            dependency_data.desire_producers.push(Vec::new());

            // intentionally dropping the initially built hypotheis
            _ = current_builder.build(game_state, &mut self);

            current_reference += 1;
            if current_reference == self.builders.len() {
                break;
            }
        }

        let dependencies = self
            .dependencies
            .take()
            .expect("Dependencies should still be here at this point");

        info!(logger: self.log, "Building hypotheses (Dependencies follow)");
        let mut hypotheses = Vec::with_capacity(current_reference);

        for (index, builder) in self.builders.clone().into_iter().enumerate() {
            let hypothesis = builder.build(game_state, &mut self);

            info!(logger: self.log, "{}: {}", HypothesisReference::new(index), hypothesis);
            if let Some((debugger, breaker)) = debugger {
                let mut debugger = debugger.lock().expect("Debugger lock was poisoned!");
                let node = create_hypothesis_node(
                    format!("{hypothesis}"),
                    dependencies.hypotheses[index]
                        .iter()
                        .map(|reference| reference.index())
                        .collect(),
                    dependencies.desire_producers[index]
                        .iter()
                        .map(|reference| reference.index())
                        .collect(),
                    dependencies.desire_consumers[index]
                        .iter()
                        .map(|reference| reference.index())
                        .collect(),
                );

                nodes_mut(&mut debugger).push(Node::Hypothesis(node));
                breaker(Breakpoint::RegisterNode(index))
            }

            for dependency in &dependencies.hypotheses[index] {
                info!(logger: self.log, "- {dependency}");
            }

            hypotheses.push(hypothesis);
        }

        info!(logger: self.log, "Hypotheses built");

        info!(logger: self.log, "{} Desires:", self.desires.len());
        let mut desire_definitions = Vec::with_capacity(self.desires.len());
        for (index, desire) in self.desires.into_iter().enumerate() {
            let definition = DesireDefinition::new(
                desire,
                dependencies
                    .desire_producers
                    .iter()
                    .filter(|producer_references| {
                        producer_references
                            .iter()
                            .any(|reference| reference.index() == index)
                    })
                    .count(),
                dependencies
                    .desire_consumers
                    .iter()
                    .any(|consumer_references| {
                        consumer_references
                            .iter()
                            .any(|reference| reference.index() == index)
                    }),
            );

            info!(logger: self.log, "- {}: {}", DesireProducerReference::new(index), definition);
            desire_definitions.push(definition);
        }

        HypothesisGraphData::new(root_reference, hypotheses, dependencies, desire_definitions)
    }
}

impl<'a, TLog> HypothesisRegistrar<HypothesisBuilderType, DesireType>
    for HypothesisRegistrarImpl<'a, TLog, HypothesisBuilderType, DesireType>
where
    TLog: Log,
{
    /// Register a dependency of the currently building [`Hypothesis`]' [`HypothesisBuilder`] and get its [`HypothesisReference`].
    fn register<THypothesisBuilderImpl>(
        &mut self,
        builder: THypothesisBuilderImpl,
    ) -> HypothesisReference
    where
        HypothesisBuilderType: From<THypothesisBuilderImpl>,
    {
        self.register_builder_type(builder.into())
    }

    fn register_builder_type(&mut self, builder: HypothesisBuilderType) -> HypothesisReference {
        let mut reference_option = None;
        for (index, existing_builder) in self.builders.iter().enumerate() {
            if builder == *existing_builder {
                reference_option = Some(HypothesisReference::new(index));
                break;
            }
        }

        let reference = match reference_option {
            Some(reference) => reference,
            None => {
                let reference = HypothesisReference::new(self.builders.len());
                self.builders.push(builder);
                reference
            }
        };

        if let Some(dependencies) = &mut self.dependencies {
            let dependencies_index = dependencies.hypotheses.len() - 1;
            dependencies.hypotheses[dependencies_index].push(reference.clone());
        }

        reference
    }

    fn register_desire_consumer(&mut self, desire: DesireType) -> DesireConsumerReference {
        let index = self.register_desire_core(desire.clone());
        let reference = DesireConsumerReference::new(index);

        if let Some(dependencies) = self.dependencies.as_mut() {
            let consumers = dependencies
                .desire_consumers
                .last_mut()
                .expect("Consumer entry should exist!");
            for existing_reference in consumers.iter() {
                if reference == *existing_reference {
                    return reference;
                }
            }

            consumers.push(reference.clone());
        }

        reference
    }

    fn register_desire_producer(&mut self, desire: DesireType) -> DesireProducerReference {
        let index = self.register_desire_core(desire.clone());
        let reference = DesireProducerReference::new(index);

        if let Some(dependencies) = self.dependencies.as_mut() {
            let producers = dependencies
                .desire_producers
                .last_mut()
                .expect("Producer entry should exist!");
            for existing_reference in producers.iter() {
                if reference == *existing_reference {
                    return reference;
                }
            }

            producers.push(reference.clone());
        }

        reference
    }
}
