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
	assets::{AssetStorage, Loader, Handle},
	core::transform::Transform,
	prelude::*,
	renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture}
};
use crate::components::Dynamic;
use crate::components::Gravity;
use crate::components::Player;

pub const CAMERA_WIDTH:  f32 = 384.0;
pub const CAMERA_HEIGHT: f32 = 216.0;

pub const BLOCK_SIZE: f32 = 16.0;

fn initialize_camera(world: &mut World) {
	let mut transform = Transform::default();
	transform.set_translation_xyz(0.0, 0.0, 1.0);

	world
		.create_entity()
		.with(Camera::standard_2d(CAMERA_WIDTH, CAMERA_HEIGHT))
		.with(transform)
		.build();
}

fn initialize_player(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
	let sprite_render = SpriteRender{
		sprite_sheet: sprite_sheet_handle,
		sprite_number: 0,
	};

	let mut transform = Transform::default();
	transform.set_translation_xyz(0.0, 48.0, 0.0);

	world
		.create_entity()
		.with(sprite_render)
		.with(Player::default())
		.with(Transform::default())
		.with(Dynamic::default())
		.with(Gravity)
		.with(transform)
		.build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
	let texture_handle = {
		let loader = world.read_resource::<Loader>();
		let texture_storage = world.read_resource::<AssetStorage<Texture>>();
		loader.load(
			"sprites.png",
			ImageFormat::default(),
			(),
			&texture_storage,
		)
	};

	let loader = world.read_resource::<Loader>();
	let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
	loader.load(
		"sprites.ron",
		SpriteSheetFormat(texture_handle),
		(),
		&sprite_sheet_store,
	)
}

pub struct LevelState;

impl SimpleState for LevelState {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let world = data.world;

		let sprite_sheet_handle = load_sprite_sheet(world);

		initialize_camera(world);
		initialize_player(world, sprite_sheet_handle);
	}
}