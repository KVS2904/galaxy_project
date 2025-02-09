use crate::{galaxy_core::*, stars_generators::*};
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use std::process;
pub trait UserInputHandler {
	fn handle_input(&self, galaxy: &mut Galaxy, stars_generator: &mut StarsGenerator, camera: &mut Camera2D);
}

struct KeyboardInputHandler {
	camera_move_speed: f32,
	camera_zoom_speed: f32,
}

impl UserInputHandler for KeyboardInputHandler {
	fn handle_input(&self, galaxy: &mut Galaxy, stars_generator: &mut StarsGenerator, camera: &mut Camera2D) {
		let dt = get_frame_time();

		if is_key_down(KeyCode::W) {
			camera.target.y -= self.camera_move_speed * dt / camera.zoom.length();
		}
		if is_key_down(KeyCode::S) {
			camera.target.y += self.camera_move_speed * dt / camera.zoom.length();
		}
		if is_key_down(KeyCode::A) {
			camera.target.x -= self.camera_move_speed * dt / camera.zoom.length();
		}
		if is_key_down(KeyCode::D) {
			camera.target.x += self.camera_move_speed * dt / camera.zoom.length();
		}
		if is_key_down(KeyCode::E) {
			camera.zoom *= 1.0 + self.camera_zoom_speed * dt;
		}
		if is_key_down(KeyCode::Q) {
			camera.zoom *= 1.0 - self.camera_zoom_speed * dt;
		}

		if root_ui().button(Vec2::new(50.0, 150.0), "Add 1000 stars") {
			stars_generator.generate(galaxy, 1000);
		}

		if root_ui().button(Vec2::new(50.0, 180.0), "Add 10000 stars") {
			stars_generator.generate(galaxy, 10000);
		}

		if root_ui().button(Vec2::new(50.0, 210.0), "Exit") {
			process::exit(0);
		}
	}
}

pub fn create_input_handler() -> impl UserInputHandler {
	KeyboardInputHandler { camera_move_speed: 1.0, camera_zoom_speed: 0.5 }
}

pub fn create_camera() -> Camera2D {
	Camera2D {
		target: vec2(0.0, 0.0),
		zoom: vec2(screen_height() / (screen_width() * screen_width()), 1.0 / screen_height()),
		offset: vec2(0.0, 0.0),
		..Default::default()
	}
}
