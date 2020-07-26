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

use std::fmt::{self, Display};

use amethyst::{
	core::{
		timing::Time,
		Transform,
		math::Vector3,
	},
	derive::SystemDesc,
	ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
	input::{InputHandler, BindingTypes},
	renderer::{Camera},
};
use serde::{Serialize, Deserialize};
use crate::components::Dynamic;
use crate::components::Player;
use crate::systems::physics::GRAVITY;
use crate::states::level::{Level, CAMERA_WIDTH, CAMERA_HEIGHT};
use crate::states::level::BLOCK_SIZE;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum MovementBindings {
	Horizontal(i8),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionBindings {
	ShortHop,
	FullHop,
}

impl Display for MovementBindings {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl Display for ActionBindings {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

#[derive(Debug)]
pub struct PlayerBindings;

impl BindingTypes for PlayerBindings {
	type Axis = MovementBindings;
	type Action = ActionBindings;
}

pub const FULL_HOP_TIME:     f32 = 0.3;
pub const SHORT_HOP_HEIGHT:  f32 = 1.25  * BLOCK_SIZE;
pub const FULL_HOP_HEIGHT:   f32 = 2.25 * BLOCK_SIZE;
pub const AERIAL_HOP_HEIGHT: f32 = 1.25 * BLOCK_SIZE;
const JUMP_COUNT: usize = 2;

pub const MAX_GROUND_SPEED: f32 = 6.0 * BLOCK_SIZE;
const GROUND_ACCELERATION: f32 = (MAX_GROUND_SPEED * 2.0) / 0.1;

pub const MAX_AERIAL_SPEED: f32 = 8.0 * BLOCK_SIZE;
const AERIAL_ACCELERATION: f32 = (MAX_AERIAL_SPEED * 2.0) / 0.5;
const AERIAL_JUMP_HORZ_BOOST: f32 = AERIAL_ACCELERATION * 0.3;

#[derive(SystemDesc)]
pub struct PlayerMovementSystem;

impl <'s> System<'s> for PlayerMovementSystem {
	type SystemData = (
		WriteStorage<'s, Dynamic>,
		WriteStorage<'s, Player>,
		Read<'s, InputHandler<PlayerBindings>>,
		Read<'s, Time>,
	);

	fn run(&mut self, (mut dynamics, mut players, input, delta_time): Self::SystemData) {
		for (dynamic, player) in (&mut dynamics, &mut players).join() {
			let velocity = &mut dynamic.velocity;
			let mut movement = 0.0f32;
			for input_id in -1..6 {
				movement += input
					.axis_value(&MovementBindings::Horizontal(input_id))
					.unwrap_or(0.0);

			}
			movement.max(-1.0).min(1.0);
			if movement != 0.0 {
				dynamic.friction_coefficient = 0.0;
			} else {
				dynamic.friction_coefficient = 1.0;
			}

			//////// OLD DEBUGGING STUFF - MAY BE USEFUL LATER ////////
			/*print!("Codes: ");
			for code in input.scan_codes_that_are_down() {
				print!("{}, ", code);
			}
			println!();*/
			///////////////////////////////////////////////////////////

			if dynamic.grounded {
				player.reset_jumps(JUMP_COUNT);

				if (movement < 0.0 && velocity.x > -MAX_GROUND_SPEED)
				|| (movement > 0.0 && velocity.x <  MAX_GROUND_SPEED) {
					velocity.x += movement * GROUND_ACCELERATION * delta_time.delta_seconds();
					velocity.x = velocity.x.max(-MAX_GROUND_SPEED).min(MAX_GROUND_SPEED);
				}

				let SHORT_HOP_SPEED: f32 = (-2.0 * SHORT_HOP_HEIGHT * GRAVITY).sqrt();
				let FULL_HOP_SPEED:  f32 = (-2.0 * FULL_HOP_HEIGHT  * GRAVITY).sqrt();
				if player.jump_ready {
					if input.action_is_down(&ActionBindings::ShortHop).unwrap_or(false) {
						velocity.y = SHORT_HOP_SPEED;
						player.trigger_jump();
					} else if input.action_is_down(&ActionBindings::FullHop).unwrap_or(false) {
						velocity.y = FULL_HOP_SPEED;
						player.trigger_jump();
					}
				}
				velocity.x = velocity.x.max(-MAX_GROUND_SPEED).min(MAX_GROUND_SPEED);
			} else {
				let AERIAL_HOP_SPEED:  f32 = (-2.0 * AERIAL_HOP_HEIGHT * GRAVITY).sqrt();
				if player.jump_ready
				&& player.jump_count > 0
				&& (input.action_is_down(&ActionBindings::ShortHop).unwrap_or(false)
				 || input.action_is_down(&ActionBindings::FullHop ).unwrap_or(false)) {
					velocity.y = AERIAL_HOP_SPEED;
					player.trigger_jump();
					velocity.x += movement * AERIAL_JUMP_HORZ_BOOST;
				} else {
					if !(input.action_is_down(&ActionBindings::ShortHop).unwrap_or(false)
					  || input.action_is_down(&ActionBindings::FullHop ).unwrap_or(false)) {
						player.jump_ready = true;
					}
					velocity.x += movement * AERIAL_ACCELERATION * delta_time.delta_seconds();
				}
				velocity.x = velocity.x.max(-MAX_AERIAL_SPEED).min(MAX_AERIAL_SPEED);
			}
		}
	}
}

#[derive(SystemDesc)]
pub struct CameraFollowSystem;

impl<'s> System<'s> for CameraFollowSystem {
	type SystemData = (
		WriteStorage<'s, Player>,
		ReadStorage<'s, Camera>,
		WriteStorage<'s, Transform>,
		Read<'s, Level>,
	);

	fn run(&mut self, (mut players, cameras, mut transforms, level): Self::SystemData) {
		let mut player_transform = Vector3::new(0.0, 0.0, 0.0);
		for (current_player_transform, _) in (&transforms, &mut players).join() {
			player_transform = current_player_transform.translation().clone();
		}
		for (camera_transform, _) in (&mut transforms, &cameras).join() {
			const HALF_WIDTH:  f32 = CAMERA_WIDTH  / 2.0;
			const HALF_HEIGHT: f32 = CAMERA_HEIGHT / 2.0;
			if player_transform.x - HALF_WIDTH < level.left {
				player_transform.x = level.left + HALF_WIDTH;
			} else if player_transform.x + HALF_WIDTH > level.right {
				player_transform.x = level.right - HALF_WIDTH;
			}

			if player_transform.y - HALF_HEIGHT < level.bottom {
				player_transform.y = level.bottom + HALF_HEIGHT;
			} else if player_transform.y + HALF_HEIGHT > level.top {
				player_transform.y = level.top - HALF_HEIGHT;
			}
			camera_transform.set_translation_xyz(player_transform.x, player_transform.y, 1.0);
		}
	}
}