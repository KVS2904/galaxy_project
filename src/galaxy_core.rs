use std::collections::HashMap;

use macroquad::prelude::*;

pub static mut STARS_COUNTER: u32 = 0;
pub const MAX_STARS_QUANTITY: u32 = 100_000_000;
pub const GALAXY_SIZE: f32 = 50_000.0; // parsec
pub const MIN_DIST_BETWEEN_STARS: f32 = 100.0; // parsec

pub type Position = Vec2;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum StarClass {
	BlueGiant,
	WhiteGiant,
	YellowGiant,
	RedGiant,
	YellowDwarf,
	RedDwarf,
	BrownDwarf,
	Neutron,
}

pub struct Star {
	pub name: String,
	pub position: Position,
	pub class: StarClass,
}

impl Star {
	pub fn new(name: &str, position: &Position, class: StarClass) -> Self {
		Self {
			name: name.to_string(),
			position: *position,
			class,
		}
	}
}

pub struct Galaxy {
	pub name: String,
	//pub stars: Vec<Star>,
	pub grid: HashMap<(i32, i32), Star>,
}

impl Galaxy {
	pub fn new(new_name: &str) -> Self {
		Self {
			name: new_name.to_string(),
			//stars: Vec::new(),
			grid: HashMap::new(),
		}
	}

	pub fn get_cell(position: &Position) -> (i32, i32) {
		let cell_size = MIN_DIST_BETWEEN_STARS;
		(
			(position.x / cell_size) as i32,
			(position.y / cell_size) as i32,
		)
	}

	pub fn add_star(&mut self, star: Star) {
		let position: &Position = &star.position;
		let cell: (i32, i32) = Self::get_cell(position);
		self.grid.entry(cell).or_insert(star);
	}

	pub fn is_valid_position(&self, position: &Position) -> bool {
		let cell = Self::get_cell(position);
		let neighbors = [
			(0, 0),
			(1, 0),
			(-1, 0),
			(0, 1),
			(0, -1),
			(1, 1),
			(-1, -1),
			(1, -1),
			(-1, 1),
		];
		for &(dx, dy) in &neighbors {
			if let Some(star) = self.grid.get(&(cell.0 + dx, cell.1 + dy)) {
				if star.position.distance_squared(*position) < MIN_DIST_BETWEEN_STARS * MIN_DIST_BETWEEN_STARS {
					return false;
				}
			}
		}
		true
	}
	// pub fn add_star(&mut self, additional_star: Star) {
	// 	self.stars.push(additional_star);
	// }
}
