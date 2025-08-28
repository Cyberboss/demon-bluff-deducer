#[enum_delegate::register]
pub trait Desire {
	fn describe(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error>;
}
