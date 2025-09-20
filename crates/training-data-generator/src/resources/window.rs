use anyhow::Result;
use bevy::ecs::resource::Resource;
use image::RgbaImage;
use thiserror::Error;
use xcap::{Window, XCapResult};

#[derive(Error, Debug)]
enum WindowResourceError {
	#[error("Could not find Demon Bluff window")]
	WindowNotFound,
}

#[derive(Resource)]
pub struct WindowResource {
	window: Window,
}

impl WindowResource {
	pub fn new() -> Result<Self> {
		let windows = Window::all()?;
		for tuple in windows.into_iter().map(|window| {
			let app_name = window.app_name()?;
			XCapResult::Ok((app_name, window))
		}) {
			let (app_name, window) = tuple?;

			if app_name.ends_with("Demon Bluff.exe") {
				return Ok(WindowResource { window });
			}
		}

		Err(WindowResourceError::WindowNotFound)?
	}

	pub fn take_screenshot(&self) -> XCapResult<RgbaImage> {
		self.window.capture_image()
	}
}
