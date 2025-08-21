use std::io::prelude::*;
use std::{env::temp_dir, path::PathBuf};

use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use thiserror::Error;

use anyhow::Result;

use rten::Model;

#[derive(Debug, Error)]
pub enum MainError {
    #[error("Could not get temp path for saving screenshots!")]
    CouldNotGetTempPath,
}

fn main() -> Result<()> {
    let mut rustautogui = rustautogui::RustAutoGui::new(false)?; // arg: debug
    println!("Hello, world!");

    let screenshot_path = temp_dir().as_path().join("demon-bluff-bot-screenshot.png");
    let screenshot_path_str = screenshot_path
        .to_str()
        .ok_or(MainError::CouldNotGetTempPath)?;

    let detection_model = Model::load_file(PathBuf::from(
        "S:/workspace/demon-bluff-bot/text-detection.rten",
    ))?;
    let recognition_model = Model::load_file(PathBuf::from(
        "S:/workspace/demon-bluff-bot/text-recognition.rten",
    ))?;

    let engine = OcrEngine::new(OcrEngineParams {
        detection_model: Some(detection_model),
        recognition_model: Some(recognition_model),
        ..Default::default()
    })?;

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    loop {
        rustautogui
            .save_screenshot(screenshot_path_str)
            .expect("Screenshot failed!");
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
        }

        write!(stdout, "Type \"next\" for next turn or \"exit\" to quit...")?;
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
