use std::collections::HashMap;

use crate::galaxy_core::*;
use macroquad::prelude::*;

struct StarRenderParams {
	radius: f32,
	color: Color,
}

pub trait GalaxyRenderer {
	async fn render_frame(&self, galaxy: &Galaxy, camera: &Camera2D);
}

pub struct GraphicsRenderer {
	star_classes_render_params: HashMap<StarClass, StarRenderParams>,
	star_texture: Texture2D,
}

impl GalaxyRenderer for GraphicsRenderer {
	async fn render_frame(&self, galaxy: &Galaxy, camera: &Camera2D) {
		set_fullscreen(true);
		clear_background(Color {
			r: (0.0),
			g: (0.02),
			b: (0.04),
			a: (0.0),
		});

		let half_width = screen_width() * 0.5 * (1.0 / camera.zoom.x);
		let half_height = screen_height() * 0.5 * (1.0 / camera.zoom.y);

		let left = camera.target.x - half_width;
		let right = camera.target.x + half_width;
		let top = camera.target.y - half_height;
		let bottom = camera.target.y + half_height;

		let x_center: f32 = 0.0;
		let y_center: f32 = 0.0;
		let galaxy_area: f32 = screen_height() * 0.8;

		let mut visible_stars = 0;

		galaxy.grid.iter().for_each(|((_, _), star)| {
			let x_pos: f32 = (star.position.x / GALAXY_SIZE) * galaxy_area + x_center;
			let y_pos: f32 = (star.position.y / GALAXY_SIZE) * galaxy_area + y_center;
			//if x_pos >= left && x_pos <= right && y_pos >= top && y_pos <= bottom {
			let star_params: &StarRenderParams = &self.star_classes_render_params[&star.class];
			draw_texture_ex(
				&self.star_texture,
				x_pos,
				y_pos,
				star_params.color,
				DrawTextureParams {
					dest_size: Some(Vec2::new(
						25.0 * star_params.radius,
						25.0 * star_params.radius,
					)),
					..Default::default()
				},
			);
			visible_stars += 1;
			//}
		});

		set_default_camera();
		draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 30.0, WHITE); // Координаты в пикселях экрана

		unsafe {
			let stars_count: u32 = STARS_COUNTER;
			draw_text(
				&format!("Stars count: {}, visible {}", stars_count, visible_stars),
				10.0,
				40.0,
				30.0,
				WHITE,
			);
		}

		next_frame().await;
	}
}

pub fn create_galaxy_renderer(star_texture: Texture2D) -> impl GalaxyRenderer {
	let star_classes_render_params: HashMap<StarClass, StarRenderParams> = HashMap::from([
		(
			StarClass::BlueGiant,
			StarRenderParams {
				radius: 1.1,
				color: Color {
					r: 0.7,
					g: 0.8,
					b: 1.0,
					a: 1.0,
				},
			},
		),
		(
			StarClass::WhiteGiant,
			StarRenderParams {
				radius: 1.2,
				color: Color {
					r: 0.9,
					g: 0.9,
					b: 0.9,
					a: 1.0,
				},
			},
		),
		(
			StarClass::YellowGiant,
			StarRenderParams {
				radius: 0.9,
				color: Color {
					r: 0.95,
					g: 0.95,
					b: 0.7,
					a: 1.0,
				},
			},
		),
		(
			StarClass::RedGiant,
			StarRenderParams {
				radius: 1.0,
				color: Color {
					r: 1.0,
					g: 0.4,
					b: 0.4,
					a: 1.0,
				},
			},
		),
		(
			StarClass::YellowDwarf,
			StarRenderParams {
				radius: 0.5,
				color: Color {
					r: 0.95,
					g: 0.95,
					b: 0.8,
					a: 1.0,
				},
			},
		),
		(
			StarClass::RedDwarf,
			StarRenderParams {
				radius: 0.4,
				color: Color {
					r: 1.0,
					g: 0.6,
					b: 0.6,
					a: 1.0,
				},
			},
		),
		(
			StarClass::BrownDwarf,
			StarRenderParams {
				radius: 0.4,
				color: Color {
					r: 0.7,
					g: 0.6,
					b: 0.4,
					a: 1.0,
				},
			},
		),
		(
			StarClass::Neutron,
			StarRenderParams {
				radius: 0.5,
				color: Color {
					r: 1.0,
					g: 1.0,
					b: 1.0,
					a: 1.0,
				},
			},
		),
	]);

	GraphicsRenderer {
		star_classes_render_params,
		star_texture,
	}
}
