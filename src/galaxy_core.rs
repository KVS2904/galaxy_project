use macroquad::prelude::*;

pub static mut STARS_COUNTER: u32 = 0;
pub const MAX_STARS_QUANTITY: u32 = 150_000;
pub const GALAXY_SIZE: f32 = 50_000.0; // parsec
pub const MIN_DIST_BETWEEN_STARS: f32 = 85.0; // parsec

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
	pub id: u32,
	pub name: String,
	pub position: Position,
	pub class: StarClass,
}

impl Star {
	pub fn new(name: &str, position: &Position, class: StarClass) -> Self {
		unsafe {
			let id: u32 = STARS_COUNTER;
			STARS_COUNTER += 1;

			Self {
				id,
				name: name.to_string(),
				position: *position,
				class,
			}
		}
	}
}

pub struct Galaxy {
	pub name: String,
	pub stars: Vec<Star>,
}

impl Galaxy {
	pub fn new(new_name: &str) -> Self {
		Self {
			name: new_name.to_string(),
			stars: Vec::new(),
		}
	}

	pub fn add_stars(&mut self, additional_stars: Vec<Star>) {
		self.stars.extend(additional_stars);
	}

	pub fn add_star(&mut self, additional_star: Star) {
		self.stars.push(additional_star);
	}
}
