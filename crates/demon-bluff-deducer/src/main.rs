use std::{
	collections::HashMap,
	env::temp_dir,
	io::prelude::*,
	path::{Path, PathBuf},
};

use anyhow::Result;
use demon_bluff_gameplay_engine::villager::{GoodVillager, VillagerArchetype};
use image::ColorType;
use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;
use rustautogui::{MatchMode, RustAutoGui, errors::AutoGuiError};
use thiserror::Error;
use xcap::{Monitor, Window};

const IMAGE_MATCH_PRECISION: f32 = 0.5;

// TODO: Resolution specific regions for reading text

#[derive(Debug, Error)]
pub enum MainError {
	#[error("Could not get temp path for saving screenshots!")]
	CouldNotGetTempPath,
}

fn load_image(
	rustautogui: &mut RustAutoGui,
	lookup: &mut HashMap<VillagerArchetype, String>,
	archetype: VillagerArchetype,
	file_name: &str,
) -> Result<(), AutoGuiError> {
	lookup.insert(archetype, file_name.to_string());
	rustautogui.store_template_from_file(
		format!(
			"S:/workspace/demon-bluff-bot/crates/demon-bluff-bot/assets/{}.png",
			file_name
		)
		.as_str(),
		None,
		MatchMode::Segmented,
		file_name,
	)
}

fn main() -> Result<()> {
	let window = Window::all()
		.unwrap()
		.into_iter()
		.filter(|window| {
			let window_name = window
				.app_name()
				.expect("Unable to read app name of a window");

			window_name.ends_with("Demon Bluff.exe")
		})
		.next()
		.expect("Did not find Demon Bluff Window");

	let image = window
		.capture_image()
		.expect("Failed to capture demon bluff sceenshot");

	let mut image = image::imageops::crop_imm(&image, 410, 365, 112, 99).to_image();

	image::imageops::colorops::invert(&mut image);

	image
		.save("S:/workspace/demon-bluff-deducer/screen.png")
		.unwrap();

	let mut rustautogui = rustautogui::RustAutoGui::new(false)?; // arg: debug
	println!("Hello, world!");

	// TODO: Train custom model for recognizing game text
	let detection_model = Model::load_file(PathBuf::from(
		"S:/workspace/demon-bluff-deducer/text-detection.rten",
	))?;
	let recognition_model = Model::load_file(PathBuf::from(
		"S:/workspace/demon-bluff-deducer/text-recognition.rten",
	))?;

	let engine = OcrEngine::new(OcrEngineParams {
		detection_model: Some(detection_model),
		recognition_model: Some(recognition_model),
		..Default::default()
	})?;

	let img_source = ImageSource::from_bytes(image.as_raw(), image.dimensions())?;
	println!("Image loaded");
	let ocr_input = engine.prepare_input(img_source)?;
	println!("Input prepped");
	let word_rects = engine.detect_words(&ocr_input)?;
	println!("Words detected");
	let line_rects = engine.find_text_lines(&ocr_input, &word_rects);

	println!("lines detected");

	// Recognize the characters in each line.
	let line_texts = engine.recognize_text(&ocr_input, &line_rects)?;

	println!("Text recognized: ");

	for line in line_texts
		.iter()
		.flatten()
		// Filter likely spurious detections. With future model improvements
		// this should become unnecessary.
		.filter(|l| l.to_string().len() > 1)
	{
		println!("{}", line);
	}

	return Ok(());

	let mut lookup = HashMap::new();

	// load cards
	{
		load_image(
			&mut rustautogui,
			&mut lookup,
			VillagerArchetype::GoodVillager(GoodVillager::Alchemist),
			"alchemist",
		)?;
		load_image(
			&mut rustautogui,
			&mut lookup,
			VillagerArchetype::GoodVillager(GoodVillager::Architect),
			"architect",
		)?;
	}

	let stdin = std::io::stdin();
	let mut stdout = std::io::stdout();

	loop {
		let find = rustautogui.find_stored_image_on_screen(
			0.2,
			lookup
				.get(&VillagerArchetype::GoodVillager(GoodVillager::Alchemist))
				.expect("Missing alchemist!"),
		)?;

		if find.is_some() {
			writeln!(stdout, "Found alchemist")?;
		} else {
			writeln!(stdout, "Did not find alchemist")?;
		}

		let find2 = rustautogui.find_stored_image_on_screen(
			0.5,
			lookup
				.get(&VillagerArchetype::GoodVillager(GoodVillager::Architect))
				.expect("Missing architect!"),
		)?;

		if find2.is_some() {
			writeln!(stdout, "Found architect")?;
		} else {
			writeln!(stdout, "Did not find architect")?;
		}
		/*
		let img = image::open(&screenshot_path).map(|image| image.into_rgb8())?;
		let img_source = ImageSource::from_bytes(img.as_raw(), img.dimensions())?;
		let ocr_input = engine.prepare_input(img_source)?;
		let word_rects = engine.detect_words(&ocr_input)?;
		let line_rects = engine.find_text_lines(&ocr_input, &word_rects);

		// Recognize the characters in each line.
		let line_texts = engine.recognize_text(&ocr_input, &line_rects)?;

		for line in line_texts
			.iter()
			.flatten()
			// Filter likely spurious detections. With future model improvements
			// this should become unnecessary.
			.filter(|l| l.to_string().len() > 1)
		{
			println!("{}", line);
		}*/

		writeln!(stdout, "Type \"next\" for next turn or \"exit\" to quit...")?;
		stdout.flush()?;

		loop {
			let mut string = String::new();
			stdin.read_line(&mut string)?;

			if string.starts_with("next") {
				break;
			}
			if string.starts_with("exit") {
				return Ok(());
			}

			write!(stdout, "Unknown input")?;
			stdout.flush()?;
		}
	}
}
