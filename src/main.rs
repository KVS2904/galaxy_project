use macroquad::prelude::*;
mod galaxy_core;
use galaxy_core::*;
mod stars_generators;
use stars_generators::*;
mod galaxy_renderers;
use galaxy_renderers::*;
mod main_menu;
use main_menu::*;
mod user_input_handlers;
use user_input_handlers::*;

#[macroquad::main("Galaxy project")]
async fn main() {
	set_fullscreen(true);
	let mut game_camera: Camera2D = create_camera();

	// --- Main menu loop ---
	let mut generation_params: GenerationParams = GenerationParams::new();
	let mut quit_flag: bool = false;
	while !quit_flag {
		main_menu::main_menu_loop(&mut quit_flag, &mut generation_params).await;
	}
	let galaxy_seed: u32 = generation_params.galaxy_seed_string.trim().parse::<u32>().ok().unwrap();
	let arms_count: u32 = generation_params.arms_count_string.trim().parse::<u32>().ok().unwrap();
	let initial_stars_num: u32 = generation_params.initial_stars_num_string.trim().parse::<u32>().ok().unwrap();

	// --- Components creation ---
	let mut galaxy: Galaxy = Galaxy::new(&generation_params.galaxy_name);
	let mut stars_generator: StarsGenerator = StarsGenerator::new(
		galaxy_seed as u64,
		StarGenerationPreset {
			name_generator: Box::new(CodeNameGenerator { letters_num: 2, numbers_num: 5 }),
			position_generator: Box::new(SpiralPositionGenerator::new(arms_count)),
			class_generator: Box::new(ColorDependedClassGenerator::new(generation_params.star_classes_flags)),
		},
	);
	stars_generator.generate(&mut galaxy, initial_stars_num);

	let input_handler = create_input_handler();
	let galaxy_renderer = create_galaxy_renderer().await;

	// --- Simulation loop
	loop {
		input_handler.handle_input(&mut galaxy, &mut stars_generator, &mut game_camera);
		galaxy_renderer.render_frame(&galaxy, &stars_generator, &game_camera).await;
	}
}
