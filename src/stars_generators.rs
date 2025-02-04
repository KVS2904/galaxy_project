use crate::galaxy_core::*;
use ::rand::prelude::*;
use macroquad::math::clamp;

pub trait StarsGenerator {
	fn generate(&mut self, galaxy: &mut Galaxy, stars_num: u32);
}

pub struct RandomGenerator {
	pub rng: StdRng,
}

impl RandomGenerator {
	fn create_new_star(&mut self, galaxy: &mut Galaxy) -> Star {
		let new_name: String = self.get_rand_name();
		let new_position: Position = self.get_rand_position();
		let new_class: StarClass = self.get_rand_class();
		Star::new(&new_name, &new_position, new_class)
	}

	fn get_rand_name(&mut self) -> String {
		let letters: String = (0..2)
			.map(|_| (self.rng.random_range(b'A'..=b'Z') as char))
			.collect();
		let number: u16 = self.rng.random_range(1000..=9999);
		format!("{}-{}", &letters, &number)
	}

	fn get_rand_position(&mut self) -> Position {
		let x: f32 = self
			.rng
			.random_range(-GALAXY_SIZE_X / 2.0..=GALAXY_SIZE_X / 2.0);
		let y: f32 = self
			.rng
			.random_range(-GALAXY_SIZE_Y / 2.0..=GALAXY_SIZE_Y / 2.0);
		Position::new(x, y)
	}

	fn get_rand_class(&mut self) -> StarClass {
		let classes = [
			StarClass::BlueGiant,
			StarClass::WhiteGiant,
			StarClass::YellowGiant,
			StarClass::RedGiant,
			StarClass::YellowDwarf,
			StarClass::RedDwarf,
			StarClass::BrownDwarf,
			StarClass::Neutron,
		];
		let class = &classes[self.rng.random_range(0..classes.len())];
		class.clone()
	}
}

impl StarsGenerator for RandomGenerator {
	fn generate(&mut self, galaxy: &mut Galaxy, stars_num: u32) {
		unsafe {
			let generated_stars_num = clamp(stars_num, 0, MAX_STARS_QUANTITY - STARS_COUNTER);
			let stars: Vec<Star> = (0..generated_stars_num)
				.map(|_| self.create_new_star(galaxy))
				.collect();
			galaxy.add_stars(stars);
		}
	}
}

pub fn create_stars_generator(galaxy_seed: u64) -> impl StarsGenerator {
	let rng: StdRng = StdRng::seed_from_u64(galaxy_seed);
	RandomGenerator { rng }
}
