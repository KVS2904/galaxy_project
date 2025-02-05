use crate::galaxy_core::*;
use ::rand::prelude::*;
use macroquad::math::clamp;
use rand_distr::Normal;

// --- Configuration preset ---
struct StarGenerationPreset {
	name_generator: Box<dyn StarNameGenerator>,
	position_generator: Box<dyn StarPositionGenerator>,
	class_generator: Box<dyn StarClassGenerator>,
}

// --- Name generators ---
trait StarNameGenerator {
	fn generate(&mut self, rng: &mut dyn RngCore) -> String;
}

struct CodeNameGenerator {
	letters_num: u16,
	numbers_num: u16,
}

impl StarNameGenerator for CodeNameGenerator {
	fn generate(&mut self, rng: &mut dyn RngCore) -> String {
		let letters: String = (0..self.letters_num)
			.map(|_| (rng.random_range(b'A'..=b'Z') as char))
			.collect();
		let number: u16 = rng.random_range(10 ^ self.numbers_num..10 ^ (self.numbers_num + 1));
		format!("{}-{}", &letters, &number)
	}
}

// --- Position generators ---
trait StarPositionGenerator {
	fn generate(&mut self, rng: &mut dyn RngCore) -> Position;
}

struct RectanglePositionGenerator;

impl StarPositionGenerator for RectanglePositionGenerator {
	fn generate(&mut self, rng: &mut dyn RngCore) -> Position {
		let x: f32 = rng.random_range(-GALAXY_SIZE / 2.0..=GALAXY_SIZE / 2.0);
		let y: f32 = rng.random_range(-GALAXY_SIZE / 2.0..=GALAXY_SIZE / 2.0);
		Position::new(x, y)
	}
}

struct CloudPositionGenerator;

impl StarPositionGenerator for CloudPositionGenerator {
	fn generate(&mut self, rng: &mut dyn RngCore) -> Position {
		let normal_pos = Normal::new(0.0, GALAXY_SIZE / 4.0).unwrap();
		let x = normal_pos.sample(rng);
		let y = normal_pos.sample(rng);
		Position::new(x, y)
	}
}

struct SpiralPositionGenerator;

impl StarPositionGenerator for SpiralPositionGenerator {
	fn generate(&mut self, rng: &mut dyn RngCore) -> Position {
		let arm_count = 3;
		let arm_angle_step = std::f32::consts::PI * 2.0 / arm_count as f32;

		let arm_index = rng.random_range(0..arm_count);
		let arm_base_angle = arm_index as f32 * arm_angle_step;

		let b = 0.3;

		let normal_r = Normal::new(0.0, GALAXY_SIZE / 4.0).unwrap();
		let r = normal_r.sample(rng);
		let normal_angle = Normal::new(0.0, 0.4).unwrap();
		let theta = 1.0 / b * f32::ln(r) + arm_base_angle + normal_angle.sample(rng);

		let x = r * theta.cos();
		let y = r * theta.sin();

		Position::new(x, y)
	}
}

// --- Class generators ---
const STAR_CLASSELS: [StarClass; 8] = [
	StarClass::BlueGiant,
	StarClass::WhiteGiant,
	StarClass::YellowGiant,
	StarClass::RedGiant,
	StarClass::YellowDwarf,
	StarClass::RedDwarf,
	StarClass::BrownDwarf,
	StarClass::Neutron,
];
trait StarClassGenerator {
	fn generate(&mut self, rng: &mut dyn RngCore) -> StarClass;
}

struct UniformClassGenerator;

impl StarClassGenerator for UniformClassGenerator {
	fn generate(&mut self, rng: &mut dyn RngCore) -> StarClass {
		let class: StarClass = STAR_CLASSELS[rng.random_range(0..STAR_CLASSELS.len())];
		class.clone()
	}
}

// --- Main generator ---
pub struct StarsGenerator {
	generation_preset: StarGenerationPreset,
	rng: StdRng,
}

impl StarsGenerator {
	fn new(galaxy_seed: u64, generation_preset: StarGenerationPreset) -> Self {
		Self {
			generation_preset,
			rng: StdRng::seed_from_u64(galaxy_seed),
		}
	}

	pub fn generate(&mut self, galaxy: &mut Galaxy, stars_num: u32) {
		unsafe {
			let generated_stars_num = clamp(stars_num, 0, MAX_STARS_QUANTITY - STARS_COUNTER);
			for _ in 0..generated_stars_num {
				let new_star: Star = self.create_new_star(galaxy);
				galaxy.add_star(new_star);
			}
		}
	}
	fn create_new_star(&mut self, galaxy: &mut Galaxy) -> Star {
		let name: String = self
			.generation_preset
			.name_generator
			.generate(&mut self.rng);

		let mut position: Position = self
			.generation_preset
			.position_generator
			.generate(&mut self.rng);

		while !StarsGenerator::is_valid_position(&position, galaxy) {
			position = self
				.generation_preset
				.position_generator
				.generate(&mut self.rng);
		}

		let class: StarClass = self
			.generation_preset
			.class_generator
			.generate(&mut self.rng);
		Star::new(&name, &position, class)
	}

	fn is_valid_position(position: &Position, galaxy: &mut Galaxy) -> bool {
		if galaxy.stars.is_empty() {
			return true;
		}
		galaxy.stars.iter().all(|star: &Star| -> bool {
			star.position.distance_squared(*position) > MIN_DIST_BETWEEN_STARS * MIN_DIST_BETWEEN_STARS
		})
	}
}

// --- Functions for creating generators ---
pub enum GalaxyType {
	Rectangle,
	Cloud,
	Spiral,
}

pub fn create_stars_generator(galaxy_seed: u64, galaxy_type: GalaxyType) -> StarsGenerator {
	match galaxy_type {
		GalaxyType::Rectangle => create_rectangle_generator(galaxy_seed),
		GalaxyType::Cloud => create_cloud_generator(galaxy_seed),
		GalaxyType::Spiral => create_spiral_generator(galaxy_seed),
	}
}

fn create_rectangle_generator(galaxy_seed: u64) -> StarsGenerator {
	StarsGenerator::new(
		galaxy_seed,
		StarGenerationPreset {
			name_generator: Box::new(CodeNameGenerator {
				letters_num: 2,
				numbers_num: 4,
			}),
			position_generator: Box::new(RectanglePositionGenerator {}),
			class_generator: Box::new(UniformClassGenerator {}),
		},
	)
}

fn create_spiral_generator(galaxy_seed: u64) -> StarsGenerator {
	StarsGenerator::new(
		galaxy_seed,
		StarGenerationPreset {
			name_generator: Box::new(CodeNameGenerator {
				letters_num: 2,
				numbers_num: 4,
			}),
			position_generator: Box::new(SpiralPositionGenerator {}),
			class_generator: Box::new(UniformClassGenerator {}),
		},
	)
}

fn create_cloud_generator(galaxy_seed: u64) -> StarsGenerator {
	StarsGenerator::new(
		galaxy_seed,
		StarGenerationPreset {
			name_generator: Box::new(CodeNameGenerator {
				letters_num: 2,
				numbers_num: 4,
			}),
			position_generator: Box::new(CloudPositionGenerator {}),
			class_generator: Box::new(UniformClassGenerator {}),
		},
	)
}
