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
	core::timing::Time,
	core::{Transform},
	derive::SystemDesc,
	ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};
use crate::components::Dynamic;
use crate::components::Static;
use crate::components::Gravity;
use crate::systems::player::FULL_HOP_TIME;
use crate::systems::player::FULL_HOP_HEIGHT;
use crate::systems::player::MAX_GROUND_SPEED;
use crate::systems::player::MAX_AERIAL_SPEED;
use crate::states::level::CAMERA_HEIGHT;

pub const FRICTION:       f32 = MAX_GROUND_SPEED / 0.25;
pub const AIR_RESISTANCE: f32 = MAX_AERIAL_SPEED / 1.0;

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

impl<'s> System<'s> for CollisionSystem {
	type SystemData = (
		WriteStorage<'s, Transform>,
		WriteStorage<'s, Dynamic>,
		ReadStorage<'s, Static>,
		Read<'s, Time>,
	);

	fn run(&mut self, (mut transforms, mut dynamics, statics, delta_time): Self::SystemData) {
		for (transform, dynamic) in (&mut transforms, &mut dynamics).join() {
			let translation = transform.translation_mut();
			translation.x += dynamic.velocity.x * delta_time.delta_seconds();
			translation.y += dynamic.velocity.y * delta_time.delta_seconds();
			const FLOOR: f32 = -CAMERA_HEIGHT / 2.0 + 8.0;
			if translation.y < FLOOR {
				translation.y = FLOOR;
				dynamic.velocity.y = 0.0;
				dynamic.grounded = true;
			} else {
				dynamic.grounded = false;
			}
		}
	}
}
