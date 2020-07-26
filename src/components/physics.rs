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
	core::math::Vector2,
	ecs::prelude::{Component, DenseVecStorage}
};

pub struct Gravity;

impl Component for Gravity {
	type Storage = DenseVecStorage<Self>;
}

pub struct Dynamic {
	pub velocity: Vector2<f32>,
	pub grounded: bool,
	pub friction_coefficient: f32,
}

impl Default for Dynamic {
	fn default() -> Self {
		Dynamic {
			velocity: Vector2::new(0.0, 0.0),
			grounded: false,
			friction_coefficient: 1.0,
		}
	}
}

impl Component for Dynamic {
	type Storage = DenseVecStorage<Self>;
}

pub struct Static;

impl Component for Static {
	type Storage = DenseVecStorage<Self>;
}
