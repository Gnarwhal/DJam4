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

use amethyst::{
	prelude::*,
	input::{InputBundle, StringBindings},
	renderer::{
		RenderingBundle,
		types::DefaultBackend,
		plugins::{RenderFlat2D, RenderToWindow},
	},
	utils::application_root_dir,
};

pub struct DJam;

impl SimpleState for DJam {}

fn main() -> amethyst::Result<()> {
	amethyst::start_logger(Default::default());

	let app_root = application_root_dir()?;

	let assets_dir = app_root.join("assets");
	let display_config_path = app_root.join("config").join("display.ron");
	let binding_path = app_root.join("config").join("bindings.ron");

	let input_bundle = InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

	let game_data = GameDataBuilder::default()
		.with_bundle(input_bundle)?
		.with_bundle(
			RenderingBundle::<DefaultBackend>::new()
				.with_plugin(
					RenderToWindow::from_config_path(display_config_path)?
						.with_clear([0.0, 0.0, 0.0, 1.0]),
				)
				.with_plugin(RenderFlat2D::default())
		)?;

	let mut game = Application::new(assets_dir,  DJam,game_data)?;
	game.run();

	Ok(())
}
