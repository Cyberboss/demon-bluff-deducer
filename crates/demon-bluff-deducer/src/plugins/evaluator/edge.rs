pub enum Edge {
	Hypothesis(usize),
	DesireProducer(usize, Option<bool>),
	DesireConsumer(usize),
}
