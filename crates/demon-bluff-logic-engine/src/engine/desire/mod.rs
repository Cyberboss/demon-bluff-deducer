mod consumer_reference;
mod data;
mod definition;
mod producer_reference;
mod r#trait;

use std::fmt::Formatter;

pub use self::{
    consumer_reference::DesireConsumerReference, data::DesireData, definition::DesireDefinition,
    producer_reference::DesireProducerReference, r#trait::Desire,
};

fn fmt_desire(index: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "D-{:05}", index + 1)
}
