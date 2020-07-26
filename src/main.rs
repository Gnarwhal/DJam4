/*******************************************************************************
 *
 * Copyright (c) 2020 Gnarwhal
 *
 * -----------------------------------------------------------------------------
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files(the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 *******************************************************************************/

mod components;
mod states;
mod systems;

use amethyst::{
	core::TransformBundle,
	prelude::*,
	input::{InputBundle},
	renderer::{
		RenderingBundle,
		types::DefaultBackend,
		plugins::{RenderFlat2D, RenderToWindow},
	},
	utils::application_root_dir,
	window::{DisplayConfig, MonitorIdent},
	winit::EventsLoop,
};
use std::path::PathBuf;

fn main() -> amethyst::Result<()> {
	amethyst::start_logger(Default::default());

	let app_root = application_root_dir()?;

	let resources_dir = app_root.join("resources");
	let display_config_path = resources_dir.join("display_config.ron");
	let binding_path = resources_dir.join("bindings.ron");

	let events_loop = EventsLoop::new();
	let monitor = MonitorIdent::from_primary(&events_loop);

	let mut display_config = DisplayConfig::load(display_config_path)?;
	display_config.icon = Some(PathBuf::from("resources/icon.png"));
	display_config.dimensions = Some(monitor.monitor_id(&events_loop).get_dimensions().into());
	display_config.fullscreen = Some(monitor);

	let input_bundle = InputBundle::<systems::PlayerBindings>::new()
		.with_bindings_from_file(binding_path)?;

	let game_data = GameDataBuilder::default()
		.with_bundle(TransformBundle::new())?
		.with_bundle(input_bundle)?
		.with_bundle(
			RenderingBundle::<DefaultBackend>::new()
				.with_plugin(
					RenderToWindow::from_config(display_config)
						.with_clear([0.0, 0.0, 0.0, 1.0]),
				)
				.with_plugin(RenderFlat2D::default())
		)?
		.with(systems::ForceSystem, "force_system", &[])
		.with(systems::PlayerMovementSystem, "player_movement_system", &["force_system", "input_system"])
		.with(systems::CollisionSystem, "collision_system", &["player_movement_system"])
		.with(systems::CameraFollowSystem, "player_post_collision_system", &["collision_system"]);

	let mut game = Application::new(resources_dir, states::LevelState, game_data)?;
	game.run();

	Ok(())
}
