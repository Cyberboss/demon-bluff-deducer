use super::verticies::Verticies;

pub trait HasVerticies {
	fn verticies(&self) -> &Verticies;
}
