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
	core::{
		timing::Time,
		math::Vector3,
	},
	core::{Transform},
	derive::SystemDesc,
	ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};
use crate::components::Dynamic;
use crate::components::Gravity;
use crate::components::Tile;
use crate::systems::player::FULL_HOP_TIME;
use crate::systems::player::FULL_HOP_HEIGHT;
use crate::systems::player::MAX_GROUND_SPEED;
use crate::systems::player::MAX_AERIAL_SPEED;
use crate::states::level::Level;
use crate::states::level::BLOCK_SIZE;

pub const FRICTION:       f32 = MAX_GROUND_SPEED / 0.1;
pub const AIR_RESISTANCE: f32 = MAX_AERIAL_SPEED / 0.5;

pub const GRAVITY: f32 = -2.0 * FULL_HOP_HEIGHT / (FULL_HOP_TIME * FULL_HOP_TIME);

#[derive(SystemDesc)]
pub struct ForceSystem;

fn apply_resistance(velocity: f32, resistance: f32) -> f32 {
	if velocity < 0.0 {
		(velocity + resistance).min(0.0)
	} else if velocity > 0.0 {
		(velocity - resistance).max(0.0)
	} else {
		0.0
	}
}

impl<'s> System<'s> for ForceSystem {
	type SystemData = (
		WriteStorage<'s, Dynamic>,
		ReadStorage<'s, Gravity>,
		Read<'s, Time>,
	);

	fn run(&mut self, (mut dynamics, gravities, delta_time): Self::SystemData) {
		for dynamic in (&mut dynamics).join() {
			let velocity = &mut dynamic.velocity;
			velocity.x = apply_resistance(velocity.x, match dynamic.grounded { true => FRICTION, false => AIR_RESISTANCE, } * dynamic.friction_coefficient * delta_time.delta_seconds());
		}
		for (dynamic, _) in (&mut dynamics, &gravities).join() {
			dynamic.velocity.y += GRAVITY * delta_time.delta_seconds();
		}
	}
}

#[derive(SystemDesc)]
pub struct CollisionSystem;

fn attempt_collision(object: &mut Vector3<f32>, level: &Read<Level>, tiles: &ReadStorage<Tile>) -> u32 {
	let left   = (((object.x - level.left  ) / BLOCK_SIZE - 0.5).floor() as usize).min(level.width  - 1).max(0);
	let bottom = (((object.y - level.bottom) / BLOCK_SIZE - 0.5).floor() as usize).min(level.height - 1).max(0);
	let right  = (((object.x - level.left  ) / BLOCK_SIZE - 0.5).ceil()  as usize).min(level.width  - 1).max(0);
	let top    = (((object.y - level.bottom) / BLOCK_SIZE - 0.5).ceil()  as usize).min(level.height - 1).max(0);
	let mut closest = Option::<(usize, usize)>::None;
	let mut distance = (BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE * BLOCK_SIZE);
	for i in left..=right {
		for j in bottom..=top {
			match (&mut closest, tiles.get(level.entities[(level.height - j - 1) * level.width + i]).unwrap()) {
				(None, Tile::Ground) | (None, Tile::Start) | (None, Tile::End) => {
					closest = Some((i, j));
					let dist_x = (object.x - level.left  ) - (i as f32 + 0.5) * BLOCK_SIZE;
					let dist_y = (object.y - level.bottom) - (j as f32 + 0.5) * BLOCK_SIZE;
					distance = (dist_x, dist_y, (dist_x * dist_x + dist_y * dist_y).sqrt());
				},
				(Some(_), Tile::Ground) | (Some(_), Tile::Start) | (Some(_), Tile::End) => {
					let dist_x = (object.x - level.left  ) - (i as f32 + 0.5) * BLOCK_SIZE;
					let dist_y = (object.y - level.bottom) - (j as f32 + 0.5) * BLOCK_SIZE;
					let current_distance = (dist_x * dist_x + dist_y * dist_y).sqrt();
					if current_distance < distance.2 {
						closest = Some((i, j));
						distance = (dist_x, dist_y, current_distance);
					}
				}
				(_, Tile::Background) => {},
			}
		}
	}
	if let Some((x, y)) = closest {
		if distance.0.abs() > distance.1.abs() {
			object.x += (BLOCK_SIZE - distance.0.abs()) * distance.0.signum();
			(2 - distance.0.signum() as i32) as u32
		} else {
			object.y += (BLOCK_SIZE - distance.1.abs()) * distance.1.signum();
			(3 - distance.1.signum() as i32) as u32
		}
	} else {
		0
	}
}

impl<'s> System<'s> for CollisionSystem {
	type SystemData = (
		WriteStorage<'s, Transform>,
		WriteStorage<'s, Dynamic>,
		ReadStorage<'s, Tile>,
		Read<'s, Level>,
		Read<'s, Time>,
	);

	fn run(&mut self, (mut transforms, mut dynamics, tiles, level, delta_time): Self::SystemData) {
		for (transform, dynamic) in (&mut transforms, &mut dynamics).join() {
			let translation = transform.translation_mut();
			translation.x += dynamic.velocity.x * delta_time.delta_seconds();
			translation.y += dynamic.velocity.y * delta_time.delta_seconds();
			dynamic.grounded = false;
			let mut result = 1;
			while result != 0 {
				result = attempt_collision(translation, &level, &tiles);
				match result {
					1 => {
						dynamic.velocity.x = 0.0;
					},
					2 => {
						dynamic.velocity.y = 0.0;
						dynamic.grounded = true;
					},
					3 => {
						dynamic.velocity.x = 0.0;
					},
					4 => {
						dynamic.velocity.y = 0.0;
					},
					_ => {}
				}
			}
		}
	}
}
