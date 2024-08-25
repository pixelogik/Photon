

// Import requirements
use super::vec3::*;

// Light color has this structure
#[derive(Copy, Clone)]
pub struct LightColor {
	pub r: f64,
	pub g: f64, 
	pub b: f64,
}

// Point light sources
pub struct PointLight {
	pub position: Vector3, 
	pub color: LightColor
}

// Directional lights (like the sun) with parallel light rays
pub struct DirectionalLight {
	pub direction: Vector3,
	pub color: LightColor
}

// For weighted light color sums
pub struct WeightedLightColorSummand {
	pub light_color: LightColor, 
	pub weight: f64
}

// Returns the weighted sum of the specified summands
pub fn compute_weighted_light_color(summands: &Vec<WeightedLightColorSummand>) -> LightColor {

	let mut r: f64 = 0.0;
	let mut g: f64 = 0.0;
	let mut b: f64 = 0.0;

	for summand in summands {
		r += summand.light_color.r * summand.weight; 
		g += summand.light_color.g * summand.weight; 
		b += summand.light_color.b * summand.weight; 
	}

	return LightColor { r: r, g: g, b: b};
}