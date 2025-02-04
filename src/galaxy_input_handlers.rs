use macroquad::prelude::*;

use crate::{galaxy_core::*, stars_generators::*};

pub trait GalaxyInputHandler {
	fn handle_input(
		&self,
		galaxy: &mut Galaxy,
		stars_generator: &mut impl StarsGenerator,
		camera: &mut Camera2D,
	);
}

struct KeyboardInputHandler {
	camera_move_speed: f32,
	camera_zoom_speed: f32,
}

impl GalaxyInputHandler for KeyboardInputHandler {
	fn handle_input(
		&self,
		galaxy: &mut Galaxy,
		stars_generator: &mut impl StarsGenerator,
		camera: &mut Camera2D,
	) {
		if is_key_pressed(KeyCode::C) {
			stars_generator.generate(galaxy, 1000);
		}
		if is_key_down(KeyCode::W) {
			camera.target.y -= self.camera_move_speed;
		}
		if is_key_down(KeyCode::S) {
			camera.target.y += self.camera_move_speed;
		}
		if is_key_down(KeyCode::A) {
			camera.target.x -= self.camera_move_speed;
		}
		if is_key_down(KeyCode::D) {
			camera.target.x += self.camera_move_speed;
		}
		if is_key_down(KeyCode::E) {
			camera.zoom *= 1.0 + self.camera_zoom_speed;
		}
		if is_key_down(KeyCode::Q) {
			camera.zoom *= 1.0 - self.camera_zoom_speed;
		}
	}
}

pub fn create_input_handler() -> impl GalaxyInputHandler {
	KeyboardInputHandler {
		camera_move_speed: 2.0,
		camera_zoom_speed: 0.01,
	}
}

pub fn create_camera() -> Camera2D {
	Camera2D {
		target: vec2(0.0, 0.0),
		zoom: vec2(
			screen_height() / (screen_width() * screen_width()),
			1.0 / screen_height(),
		),
		offset: vec2(0.0, 0.0),
		..Default::default()
	}
}
