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
	core::{Transform, SystemDesc},
	derive::SystemDesc,
	ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
	input::{InputHandler, BindingTypes, Bindings},
};
use serde::{Serialize, Deserialize};
use crate::components::Player;

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

#[derive(SystemDesc)]
pub struct PlayerSystem;

impl <'s> System<'s> for PlayerSystem {
	type SystemData = (
		WriteStorage<'s, Transform>,
		ReadStorage<'s, Player>,
		Read<'s, InputHandler<PlayerBindings>>,
	);

	fn run(&mut self, (mut transforms, players, input): Self::SystemData) {
		for (transform, _) in (&mut transforms, &players).join() {
			let mut movement = 0.0f32;
			for input_id in -1..6 {
				movement += input
					.axis_value(&MovementBindings::Horizontal(input_id))
					.unwrap_or(0.0);
			}

			if movement != 0.0 {
				println!("Movement: {}", movement);
			}
			if input.action_is_down(&ActionBindings::FullHop).unwrap_or(false) {
				println!("Yump yump!");
			}
		}
	}
}