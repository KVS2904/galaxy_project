use std::collections::HashMap;

use crate::galaxy_core::*;
use macroquad::prelude::*;

struct StarRenderParams {
	radius: f32,
	color: Color,
}

pub trait GalaxyRenderer {
	async fn render_frame(&self, galaxy: &Galaxy);
}

pub struct GraphicsRenderer {
	star_classes_render_params: HashMap<StarClass, StarRenderParams>,
}

impl GalaxyRenderer for GraphicsRenderer {
	async fn render_frame(&self, galaxy: &Galaxy) {
		set_fullscreen(true);
		clear_background(Color {
			r: (0.0),
			g: (0.02),
			b: (0.04),
			a: (0.0),
		});

		let x_center: f32 = 0.0;
		let y_center: f32 = 0.0;
		let galaxy_area: f32 = screen_height() * 0.8;

		galaxy.stars.iter().for_each(|star: &Star| {
			let x_pos: f32 = (star.position.x / GALAXY_SIZE_X) * galaxy_area + x_center;
			let y_pos: f32 = (star.position.y / GALAXY_SIZE_Y) * galaxy_area + y_center;
			let star_params: &StarRenderParams = &self.star_classes_render_params[&star.class];
			draw_circle(x_pos, y_pos, star_params.radius, star_params.color);
			//draw_circle(x_pos, y_pos, 2.0, YELLOW);
			//draw_text(&star.name, x_pos - 10.0, y_pos + 20.0, 15.0, WHITE);
		});

		let fps = get_fps();
		draw_text(
			&format!("FPS: {}", fps),
			-0.6 * screen_width(),
			0.0,
			40.0,
			WHITE,
		);

		unsafe {
			let stars_count: u32 = STARS_COUNTER;
			draw_text(
				&format!("Stars count: {}", stars_count),
				-0.6 * screen_width(),
				120.0,
				40.0,
				WHITE,
			);
		}
		next_frame().await;
	}
}

pub fn create_galaxy_renderer() -> impl GalaxyRenderer {
	let star_classes_render_params: HashMap<StarClass, StarRenderParams> = HashMap::from([
		(
			StarClass::BlueGiant,
			StarRenderParams {
				radius: 1.1,
				color: Color {
					r: 0.5,
					g: 0.8,
					b: 1.0,
					a: 1.0,
				},
			},
		),
		(
			StarClass::WhiteGiant,
			StarRenderParams {
				radius: 1.0,
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
					g: 0.5,
					b: 0.5,
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
					g: 0.8,
					b: 0.8,
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
					g: 0.7,
					b: 0.5,
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
	}
}
