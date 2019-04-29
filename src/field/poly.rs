// Derived from RustySecrets Project at
// https://github.com/SpinResearch/RustySecrets.git
//
// BSD 3-Clause License
//
// Copyright (c) 2016-2018, Spin Research
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// * Redistributions of source code must retain the above copyright notice, this
//   list of conditions and the following disclaimer.
//
// * Redistributions in binary form must reproduce the above copyright notice,
//   this list of conditions and the following disclaimer in the documentation
//   and/or other materials provided with the distribution.
//
// * Neither the name of the copyright holder nor the names of its
//   contributors may be used to endorse or promote products derived from
//   this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// Modifications Copyright 2019 ???
//
// TODO: LICENSE TEXT
//

use crate::field::gf256::Gf256;
use std::fmt;

static MAX_COEFFS: usize = 256;

pub struct Poly {
	pub coeffs: Vec<Gf256>,
}

impl fmt::Debug for Poly {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let _ = write!(f, "(");
		for i in 0..self.coeffs.len() {
			let _ = write!(f, "{:?}, ", self.coeffs[i]);
		}
		writeln!(f, ")")
	}
}

impl Poly {
	pub fn new(coeffs: Vec<Gf256>) -> Self {
		Self { coeffs }
	}

	pub fn _evaluate_at_zero(&self) -> Gf256 {
		self.coeffs[0]
	}

	pub fn evaluate_at(&self, x: Gf256) -> Gf256 {
		assert!(self.coeffs.len() < MAX_COEFFS);

		let mut result = Gf256::zero();

		for (i, c) in self.coeffs.iter().enumerate() {
			result += *c * x.pow(i as u8);
		}

		result
	}
}
