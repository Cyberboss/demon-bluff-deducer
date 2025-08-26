use crate::hypotheses::DesireType;

pub struct DesireNode {
    desire_type: DesireType,
    pending: usize,
    desired: usize,
    undesired: usize,
}
