mod consumer_reference;
mod data;
mod definition;
mod producer_reference;

use std::fmt::Formatter;

pub use self::{
    consumer_reference::DesireConsumerReference, definition::DesireDefinition,
    producer_reference::DesireProducerReference,
};

fn fmt_desire(index: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "D-{:05}", index + 1)
}
