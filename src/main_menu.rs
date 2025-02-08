use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};
use std::process;

pub struct GenerationParams {
	pub galaxy_name: String,
	pub galaxy_seed_string: String,
	pub arms_count_string: String,
	pub initial_stars_num_string: String,
	pub star_classes_flags: [bool; 3],
}

impl GenerationParams {
	pub fn new() -> Self {
		Self {
			galaxy_name: String::new(),
			galaxy_seed_string: String::new(),
			arms_count_string: String::new(),
			initial_stars_num_string: String::new(),
			star_classes_flags: [false; 3],
		}
	}
}

pub async fn main_menu_loop(quit_flag: &mut bool, generation_params: &mut GenerationParams) {
	clear_background(BLACK);
	root_ui().window(
		hash!(),
		vec2(screen_width() / 2.0, screen_height() / 2.0),
		vec2(750., 250.),
		|ui| {
			ui.input_text(hash!(), "Galaxy name", &mut generation_params.galaxy_name);
			ui.input_text(hash!(), "Seed", &mut generation_params.galaxy_seed_string);
			ui.input_text(hash!(), "Number of spiral arms", &mut generation_params.arms_count_string);
			ui.input_text(
				hash!(),
				"Initial number of stars",
				&mut generation_params.initial_stars_num_string,
			);
			ui.checkbox(hash!(), "Red stars", &mut generation_params.star_classes_flags[0]);
			ui.checkbox(hash!(), "Yellow stars", &mut generation_params.star_classes_flags[1]);
			ui.checkbox(hash!(), "White stars", &mut generation_params.star_classes_flags[2]);

			if ui.button(vec2(280.0, 180.0), "Exit") {
				process::exit(0);
			}

			if is_values_correct(generation_params) {
				if ui.button(vec2(80.0, 180.0), "Create galaxy") {
					*quit_flag = true;
				}
			} else {
				ui.label(vec2(80.0, 100.0), "Enter the correct values!");
			}
		},
	);
	next_frame().await;
}

fn is_values_correct(generation_params: &GenerationParams) -> bool {
	!generation_params.galaxy_name.is_empty()
		&& !generation_params.galaxy_seed_string.trim().parse::<u64>().is_err()
		&& !generation_params.arms_count_string.trim().parse::<u64>().is_err()
		&& !generation_params.initial_stars_num_string.trim().parse::<u64>().is_err()
		&& generation_params.star_classes_flags.iter().any(|flag| *flag)
}
