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

use core::default::Default;
use amethyst::{
	ecs::prelude::{Component, DenseVecStorage},
};

pub struct Player {
	pub jump_ready: bool,
	pub jump_count: usize,
}

impl Player {
	pub fn reset_jumps(&mut self, jump_count: usize) {
		self.jump_ready = true;
		self.jump_count = jump_count;
	}

	pub fn trigger_jump(&mut self) {
		self.jump_ready  = false;
		self.jump_count -= 1;
	}
}
impl Default for Player {
	fn default() -> Player {
		return Player{
			jump_ready: true,
			jump_count: 0,
		};
	}
}

impl Component for Player {
	type Storage = DenseVecStorage<Self>;
}
