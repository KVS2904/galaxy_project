use macroquad::prelude::*;

pub static mut STARS_COUNTER: u32 = 0;
pub const MAX_STARS_QUANTITY: u32 = 100_000;
pub const GALAXY_SIZE_X: f32 = 2_000_000.0;
pub const GALAXY_SIZE_Y: f32 = 2_000_000.0;
pub const STAR_COLLISION_RADIUS: f32 = 1000.0;

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
	pub fn new(new_name: &str, new_position: &Position, new_class: StarClass) -> Self {
		unsafe {
			let new_id: u32 = STARS_COUNTER;
			STARS_COUNTER += 1;

			Self {
				id: new_id,
				name: new_name.to_string(),
				position: *new_position,
				class: new_class,
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
}
