use std::collections::HashMap;

use crate::galaxy_core::*;
use crate::stars_generators::*;
use macroquad::prelude::*;

const X_CENTER: f32 = 0.0;
const Y_CENTER: f32 = 0.0;
const BACKGROUND_COLOR: Color = Color { r: 0.0, g: 0.02, b: 0.04, a: 1.0 };
const STAR_TEXTURE_PATH: &str = "planet_640.png";
struct StarRenderParams {
	radius: f32,
	color: Color,
}

pub trait GalaxyRenderer {
	async fn render_frame(&self, galaxy: &Galaxy, stars_generator: &StarsGenerator, camera: &Camera2D);
}

struct VisibleStarsBorderIndexes {
	left: i32,
	up: i32,
	right: i32,
	down: i32,
}

pub struct GraphicsRenderer {
	star_classes_render_params: HashMap<StarClass, StarRenderParams>,
	star_texture: Texture2D,
	galaxy_area: f32,
}

impl GraphicsRenderer {
	fn new(star_classes_render_params: HashMap<StarClass, StarRenderParams>, star_texture: Texture2D) -> Self {
		//set_fullscreen(true);
		Self { star_classes_render_params, star_texture, galaxy_area: screen_height() * 2.0 }
	}

	fn to_render_position(&self, position: &Position) -> Vec2 {
		let x: f32 = (position.x / GALAXY_SIZE) * self.galaxy_area + X_CENTER;
		let y: f32 = (position.y / GALAXY_SIZE) * self.galaxy_area + Y_CENTER;
		Position::new(x, y)
	}

	fn from_render_position(&self, position: &Vec2) -> Position {
		let x: f32 = (position.x - X_CENTER) * GALAXY_SIZE / self.galaxy_area;
		let y: f32 = (position.y - Y_CENTER) * GALAXY_SIZE / self.galaxy_area;
		Position::new(x, y)
	}

	fn get_visible_stars_border_indexes(&self, camera: &Camera2D) -> VisibleStarsBorderIndexes {
		let frame_coef: f32 = 0.1;
		let left_up_corner_render: Vec2 = camera.screen_to_world(vec2(-screen_width() * frame_coef, -screen_height() * frame_coef));
		let right_down_corner_render: Vec2 = camera.screen_to_world(vec2(
			(1.0 + frame_coef) * screen_width(),
			(1.0 + frame_coef) * screen_height(),
		));
		let left_up_corner_position: Vec2 = self.from_render_position(&left_up_corner_render);
		let right_down_corner_position: Vec2 = self.from_render_position(&right_down_corner_render);

		let left_up_corner_cell: (i32, i32) = Galaxy::get_cell(&left_up_corner_position);
		let right_down_corner_cell: (i32, i32) = Galaxy::get_cell(&right_down_corner_position);
		VisibleStarsBorderIndexes {
			left: left_up_corner_cell.0,
			right: right_down_corner_cell.0,
			up: left_up_corner_cell.1,
			down: right_down_corner_cell.1,
		}
	}

	fn is_star_visible(x_grid_index: &i32, y_grid_index: &i32, border_indexes: &VisibleStarsBorderIndexes) -> bool {
		*x_grid_index >= border_indexes.left
			&& *x_grid_index <= border_indexes.right
			&& *y_grid_index >= border_indexes.up
			&& *y_grid_index <= border_indexes.down
	}

	fn draw_star(&self, star: &Star, camera: &Camera2D) {
		let render_position: Vec2 = self.to_render_position(&star.position);
		let star_params: &StarRenderParams = &self.star_classes_render_params[&star.class];
		draw_texture_ex(
			&self.star_texture,
			render_position.x,
			render_position.y,
			star_params.color,
			DrawTextureParams { dest_size: Some(1.0 * Vec2::new(star_params.radius, star_params.radius)), ..Default::default() },
		);
		GraphicsRenderer::draw_star_name(star, render_position, camera);
	}

	fn draw_star_name(star: &Star, render_position: Vec2, camera: &Camera2D) {
		if camera.zoom.length() > 0.1 {
			set_default_camera();
			let screen_position = camera.world_to_screen(render_position);
			draw_text(&star.name, screen_position.x, screen_position.y, 20.0, WHITE);
			set_camera(camera);
		}
	}

	fn draw_overlay_info(&self, galaxy: &Galaxy, stars_generator: &StarsGenerator) {
		draw_text(&format!("FPS: {}", get_fps()), screen_width() - 150.0, 50.0, 30.0, WHITE);
		draw_text(
			&format!("Galaxy: {}, seed: {}", galaxy.name, stars_generator.galaxy_seed),
			50.0,
			50.0,
			30.0,
			WHITE,
		);
		unsafe {
			let stars_count: u32 = STARS_COUNTER;
			draw_text(&format!("Stars count: {}", stars_count), 50.0, 90.0, 30.0, WHITE);
		}

		draw_text("Use QE to zoom and WASD to move", 50.0, 130.0, 30.0, WHITE);
	}
}

impl GalaxyRenderer for GraphicsRenderer {
	async fn render_frame(&self, galaxy: &Galaxy, stars_generator: &StarsGenerator, camera: &Camera2D) {
		set_camera(camera);
		clear_background(BACKGROUND_COLOR);

		let border_indexes: VisibleStarsBorderIndexes = self.get_visible_stars_border_indexes(camera);
		galaxy.stars_grid.iter().for_each(|((x_grid_index, y_grid_index), star)| {
			if GraphicsRenderer::is_star_visible(x_grid_index, y_grid_index, &border_indexes) {
				self.draw_star(star, camera);
			}
		});

		set_default_camera();
		self.draw_overlay_info(galaxy, stars_generator);
		next_frame().await;
	}
}

pub async fn create_galaxy_renderer() -> impl GalaxyRenderer {
	let star_classes_render_params: HashMap<StarClass, StarRenderParams> = HashMap::from([
		(
			StarClass::BlueGiant,
			StarRenderParams { radius: 1.1, color: Color { r: 0.7, g: 0.8, b: 1.0, a: 1.0 } },
		),
		(
			StarClass::WhiteGiant,
			StarRenderParams { radius: 1.2, color: Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 } },
		),
		(
			StarClass::YellowGiant,
			StarRenderParams { radius: 0.9, color: Color { r: 0.95, g: 0.95, b: 0.7, a: 1.0 } },
		),
		(
			StarClass::RedGiant,
			StarRenderParams { radius: 1.0, color: Color { r: 1.0, g: 0.4, b: 0.4, a: 1.0 } },
		),
		(
			StarClass::YellowDwarf,
			StarRenderParams { radius: 0.5, color: Color { r: 0.95, g: 0.95, b: 0.8, a: 1.0 } },
		),
		(
			StarClass::RedDwarf,
			StarRenderParams { radius: 0.4, color: Color { r: 1.0, g: 0.6, b: 0.6, a: 1.0 } },
		),
		(
			StarClass::BrownDwarf,
			StarRenderParams { radius: 0.4, color: Color { r: 0.7, g: 0.6, b: 0.4, a: 1.0 } },
		),
		(
			StarClass::Neutron,
			StarRenderParams { radius: 0.5, color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 } },
		),
	]);
	let star_texture: Texture2D = load_texture(STAR_TEXTURE_PATH).await.unwrap();
	star_texture.set_filter(FilterMode::Nearest);
	GraphicsRenderer::new(star_classes_render_params, star_texture)
}
