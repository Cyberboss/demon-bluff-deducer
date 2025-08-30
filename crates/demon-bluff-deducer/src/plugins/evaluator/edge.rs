pub enum Edge {
	Hypothesis(usize, usize),
	DesireProducer(usize, Option<bool>),
	DesireConsumer(usize),
}
