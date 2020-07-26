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

use image::Rgba;
use amethyst::{
	assets::{AssetStorage, Loader, Handle},
	core::transform::Transform,
	prelude::*,
	ecs::Entity,
	renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture}
};
use crate::components::Dynamic;
use crate::components::Gravity;
use crate::components::Player;
use crate::components::Tile;

pub const CAMERA_WIDTH:  f32 = 384.0;
pub const CAMERA_HEIGHT: f32 = 216.0;

pub const BLOCK_SIZE: f32 = 16.0;

const BACKGROUND: Rgba<u8> = Rgba::<u8>([255, 255, 255, 255]);
const GROUND:     Rgba<u8> = Rgba::<u8>([0,   0,   0,   255]);
const START:      Rgba<u8> = Rgba::<u8>([0,   148, 255, 255]);
const END:        Rgba<u8> = Rgba::<u8>([0,   216, 68,  255]);

pub struct Level {
	pub entities: Vec<Entity>,
	pub width:  usize,
	pub height: usize,
	pub left:   f32,
	pub bottom: f32,
	pub right:  f32,
	pub top:    f32,
}

impl Default for Level {
	fn default() -> Self {
		Level{
			entities: vec![],
			width:  0,
			height: 0,
			left:   0.0,
			bottom: 0.0,
			right:  0.0,
			top:    0.0,
		}
	}
}

fn initialize_level(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
	let background_sprite = SpriteRender {
		sprite_sheet: sprite_sheet_handle.clone(),
		sprite_number: 1,
	};
	let ground_sprite = SpriteRender {
		sprite_sheet: sprite_sheet_handle.clone(),
		sprite_number: 2,
	};
	let start_sprite = SpriteRender {
		sprite_sheet: sprite_sheet_handle.clone(),
		sprite_number: 4,
	};
	let end_sprite = SpriteRender {
		sprite_sheet: sprite_sheet_handle,
		sprite_number: 3,
	};

	let level_image = image::open("resources/levels/level0.png").unwrap().into_rgba();
	let width = level_image.width() as usize;
	let height = level_image.height() as usize;
	let center_x = BLOCK_SIZE * width  as f32 / 2.0;
	let center_y = BLOCK_SIZE * height as f32 / 2.0;
	let mut tile_map = Vec::<Entity>::with_capacity(level_image.width() as usize * level_image.height() as usize);
	for (i, pixel) in level_image.pixels().enumerate() {
		let mut transform = Transform::default();
		transform.set_translation_xyz(BLOCK_SIZE * ((i % width) as f32 + 0.5) - center_x, BLOCK_SIZE * -((i / width) as f32 + 0.5) + center_y, -1.0);
		match *pixel {
			BACKGROUND => {
				tile_map.push(world
					.create_entity()
					.with(Tile::Background)
					.with(transform)
					.build());
			},
			GROUND => {
				tile_map.push(world
					.create_entity()
					.with(ground_sprite.clone())
					.with(Tile::Ground)
					.with(transform)
					.build());
			},
			START => {
				tile_map.push(world
					.create_entity()
					.with(start_sprite.clone())
					.with(Tile::Start)
					.with(transform)
					.build());
			},
			END => {
				tile_map.push(world
					.create_entity()
					.with(end_sprite.clone())
					.with(Tile::End)
					.with(transform)
					.build());
			},
			_ => { panic!("Invalid level bitmap tile color!"); }
		}
	}
	let level = Level {
		entities: tile_map,
		width,
		height,
		left:   -(BLOCK_SIZE * width  as f32 / 2.0),
		bottom: -(BLOCK_SIZE * height as f32 / 2.0),
		right:   (BLOCK_SIZE * width  as f32 / 2.0),
		top:     (BLOCK_SIZE * height as f32 / 2.0),
	};
	world.insert(level);
}

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
	transform.set_translation_xyz(BLOCK_SIZE * -11.0, BLOCK_SIZE * -12.5, 0.0);

	world
		.create_entity()
		.with(sprite_render)
		.with(Player::default())
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
		initialize_player(world, sprite_sheet_handle.clone());

		world.register::<Tile>();
		initialize_level(world, sprite_sheet_handle);
	}
}