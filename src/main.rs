use macroquad::prelude::*;
mod galaxy_core;
use galaxy_core::*;
mod stars_generators;
use stars_generators::*;
mod galaxy_renderers;
use galaxy_renderers::*;
mod galaxy_input_handlers;
use galaxy_input_handlers::*;

#[macroquad::main("Galaxy project")]
async fn main() {
	let galaxy_seed: u64 = 239;
	let galaxy_type: GalaxyType = GalaxyType::Spiral;
	let mut stars_generator: StarsGenerator = create_stars_generator(galaxy_seed, galaxy_type);
	let mut galaxy: Galaxy = Galaxy::new("Milky way");

	let initial_stars_num: u32 = 55000;
	stars_generator.generate(&mut galaxy, initial_stars_num);

	let mut camera: Camera2D = create_camera();
	let input_handler = create_input_handler();

	let star_texture_path: &str = "planet_640.png";
	let star_texture: Texture2D = load_texture(star_texture_path).await.unwrap();
	star_texture.set_filter(FilterMode::Linear);
	let galaxy_renderer = create_galaxy_renderer(star_texture);

	loop {
		input_handler.handle_input(&mut galaxy, &mut stars_generator, &mut camera);
		set_camera(&camera);
		galaxy_renderer.render_frame(&galaxy).await;
	}
}
