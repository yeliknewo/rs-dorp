use glium::uniforms::{AsUniformValue, UniformValue};
use std::ops::{Index, IndexMut, Add, Sub, Mul};
use std::fmt::{Display, Formatter, Error};

use math::{Vec4};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
	vals: [f32; 3],
}

impl Vec3 {
    
	pub fn zero() -> Vec3 {
		Vec3::from([0.0; 3])
	}

    
	pub fn one() -> Vec3 {
		Vec3::from([1.0; 3])
	}

    
	pub fn get_vals(&self) -> [f32; 3] {
		self.vals
	}

    
	pub fn dot(&self, other: Vec3) -> f32 {
		let mut sum = 0.0;

		for i in 0..3 {
			sum += self[i] * other[i];
		}

		sum
	}

    
	pub fn to_vec4(&self, w: f32) -> Vec4 {
		Vec4::from([self[0], self[1], self[2], w])
	}
}

impl From<[f32; 3]> for Vec3 {
    
	fn from(vals: [f32; 3]) -> Vec3 {
		Vec3{
			vals: vals,
		}
	}
}

impl From<Vec4> for Vec3 {
    
	fn from(vec4: Vec4) -> Vec3 {
		Vec3::from([vec4[0], vec4[1], vec4[2]])
	}
}

impl AsUniformValue for Vec3 {
    
	fn as_uniform_value(&self) -> UniformValue {
		UniformValue::Vec3(self.vals)
	}
}

impl Index<usize> for Vec3 {
	type Output = f32;

    
	fn index(&self, index: usize) -> &f32 {
		&self.vals[index]
	}
}

impl IndexMut<usize> for Vec3 {
    
	fn index_mut(&mut self, index: usize) -> &mut f32 {
		&mut self.vals[index]
	}
}

impl Add<Vec3> for Vec3 {
	type Output = Vec3;

    
	fn add(self, other: Vec3) -> Vec3 {
		Vec3::from([self[0] + other[0], self[1] + other[1], self[2] + other[2]])
	}
}

impl Sub<Vec3> for Vec3 {
	type Output = Vec3;

    
	fn sub(self, other: Vec3) -> Vec3 {
		Vec3::from([self[0] - other[0], self[1] - other[1], self[2] - other[2]])
	}
}

impl Mul<Vec3> for Vec3 {
	type Output = Vec3;

    
	fn mul(self, other: Vec3) -> Vec3 {
		Vec3::from([self[0] * other[0], self[1] * other[1], self[2] * other[2]])
	}
}

impl Mul<f32> for Vec3 {
	type Output = Vec3;

    
	fn mul(self, other: f32) -> Vec3 {
		Vec3::from([self[0] * other, self[1] * other, self[2] * other])
	}
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
        write!(f, "({}, {}, {})", self[0], self[1], self[2])
    }
}
