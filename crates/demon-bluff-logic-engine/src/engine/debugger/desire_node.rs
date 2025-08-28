use crate::hypotheses::DesireType;

#[derive(Debug)]
pub struct DesireNode {
    desire_type: DesireType,
    pending: usize,
    desired: usize,
    undesired: usize,
}
