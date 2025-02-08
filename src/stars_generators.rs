use crate::galaxy_core::*;
use ::rand::prelude::*;
use macroquad::math::clamp;
use rand_distr::Normal;

// --- Main generator ---
pub struct StarGenerationPreset {
	pub name_generator: Box<dyn StarNameGenerator>,
	pub position_generator: Box<dyn StarPositionGenerator>,
	pub class_generator: Box<dyn StarClassGenerator>,
}
pub struct StarsGenerator {
	pub generation_preset: StarGenerationPreset,
	pub galaxy_seed: u64,
	rng: StdRng,
}

impl StarsGenerator {
	pub fn new(galaxy_seed: u64, generation_preset: StarGenerationPreset) -> Self {
		Self { generation_preset, galaxy_seed, rng: StdRng::seed_from_u64(galaxy_seed) }
	}

	pub fn generate(&mut self, galaxy: &mut Galaxy, stars_num: u32) {
		unsafe {
			let generated_stars_num = clamp(stars_num, 0, MAX_STARS_QUANTITY - STARS_COUNTER);
			for _ in 0..generated_stars_num {
				let new_star: Star = self.create_new_star(galaxy);
				galaxy.add_star(new_star);
				STARS_COUNTER += 1;
			}
		}
	}

	fn create_new_star(&mut self, galaxy: &mut Galaxy) -> Star {
		let mut position: Position = self.get_position();
		while !galaxy.is_valid_position(&position) {
			position = self.get_position();
		}
		Star::new(&self.get_name(), &position, self.get_star_class())
	}

	fn get_name(&mut self) -> String {
		self.generation_preset.name_generator.generate(&mut self.rng)
	}

	fn get_position(&mut self) -> Position {
		self.generation_preset.position_generator.generate(&mut self.rng)
	}

	fn get_star_class(&mut self) -> StarClass {
		self.generation_preset.class_generator.generate(&mut self.rng)
	}
}

// --- Name generators ---
pub trait StarNameGenerator {
	fn generate(&mut self, rng: &mut dyn RngCore) -> String;
}

pub struct CodeNameGenerator {
	pub letters_num: u32,
	pub numbers_num: u32,
}

impl StarNameGenerator for CodeNameGenerator {
	fn generate(&mut self, rng: &mut dyn RngCore) -> String {
		let letters: String = (0..self.letters_num)
			.map(|_| (rng.random_range(b'A'..=b'Z') as char))
			.collect();
		let number: u32 = rng.random_range(10_u32.pow(self.numbers_num - 1)..10_u32.pow(self.numbers_num));
		format!("{}-{}", &letters, &number)
	}
}

// --- Position generators ---
pub trait StarPositionGenerator {
	fn generate(&mut self, rng: &mut dyn RngCore) -> Position;
}

pub struct RectanglePositionGenerator;

impl StarPositionGenerator for RectanglePositionGenerator {
	fn generate(&mut self, rng: &mut dyn RngCore) -> Position {
		let x: f32 = rng.random_range(-GALAXY_SIZE / 2.0..=GALAXY_SIZE / 2.0);
		let y: f32 = rng.random_range(-GALAXY_SIZE / 2.0..=GALAXY_SIZE / 2.0);
		Position::new(x, y)
	}
}

pub struct CloudPositionGenerator {
	normal_pos: Normal<f32>,
}

impl CloudPositionGenerator {
	fn new() -> Self {
		Self { normal_pos: Normal::new(0.0, GALAXY_SIZE / 4.0).unwrap() }
	}
}

impl StarPositionGenerator for CloudPositionGenerator {
	fn generate(&mut self, rng: &mut dyn RngCore) -> Position {
		let x: f32 = self.normal_pos.sample(rng);
		let y: f32 = self.normal_pos.sample(rng);
		Position::new(x, y)
	}
}

pub struct SpiralPositionGenerator {
	arm_count: u32,
	arm_curvature: f32,
	arm_angle_step: f32,
	normal_r: Normal<f32>,
	normal_angle: Normal<f32>,
}

impl SpiralPositionGenerator {
	pub fn new(arm_count: u32) -> Self {
		if arm_count == 0 {
			panic!("arm_count must be positive!");
		}
		Self {
			arm_count,
			arm_curvature: arm_count as f32 * 0.2,
			arm_angle_step: std::f32::consts::PI * 2.0 / arm_count as f32,
			normal_r: Normal::new(0.0, GALAXY_SIZE / 4.0).unwrap(),
			normal_angle: Normal::new(0.0, 0.8 / arm_count as f32).unwrap(),
		}
	}

	fn get_theta(&self, rng: &mut dyn RngCore, r: f32, arm_base_angle: f32) -> f32 {
		1.0 / self.arm_curvature * f32::ln(r) + arm_base_angle + self.normal_angle.sample(rng)
	}
}

impl StarPositionGenerator for SpiralPositionGenerator {
	fn generate(&mut self, rng: &mut dyn RngCore) -> Position {
		let arm_index = rng.random_range(0..self.arm_count);
		let arm_base_angle = arm_index as f32 * self.arm_angle_step;
		let r: f32 = self.normal_r.sample(rng);
		let theta: f32 = self.get_theta(rng, r, arm_base_angle);
		let x = r * theta.cos();
		let y = r * theta.sin();
		Position::new(x, y)
	}
}

// --- Class generators ---
pub trait StarClassGenerator {
	fn generate(&mut self, rng: &mut dyn RngCore) -> StarClass;
}

pub struct UniformClassGenerator;

impl StarClassGenerator for UniformClassGenerator {
	fn generate(&mut self, rng: &mut dyn RngCore) -> StarClass {
		STAR_CLASSES[rng.random_range(0..STAR_CLASSES.len())]
	}
}

pub struct ColorDependedClassGenerator {
	star_classes_filtered: Vec<StarClass>,
}

impl ColorDependedClassGenerator {
	pub fn new(star_classes_flags: [bool; 3]) -> Self {
		let mut star_classes_filtered: Vec<StarClass> = Vec::new();
		if star_classes_flags[0] {
			star_classes_filtered.push(StarClass::BrownDwarf);
			star_classes_filtered.push(StarClass::RedDwarf);
			star_classes_filtered.push(StarClass::RedGiant);
		}
		if star_classes_flags[1] {
			star_classes_filtered.push(StarClass::YellowDwarf);
			star_classes_filtered.push(StarClass::YellowGiant);
		}

		if star_classes_flags[2] {
			star_classes_filtered.push(StarClass::Neutron);
			star_classes_filtered.push(StarClass::WhiteGiant);
			star_classes_filtered.push(StarClass::BlueGiant);
		}

		Self { star_classes_filtered }
	}
}

impl StarClassGenerator for ColorDependedClassGenerator {
	fn generate(&mut self, rng: &mut dyn RngCore) -> StarClass {
		self.star_classes_filtered[rng.random_range(0..self.star_classes_filtered.len())]
	}
}
