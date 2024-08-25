
// Import requirements
use super::vec3::*;
use super::light::*;

// Ideal spheres
pub struct Sphere {
	pub center: Vector3,
	pub radius: f64,
	pub material_color: LightColor
}

// Ideal planes
pub struct Plane {
	pub center: Vector3,
	pub normal: Vector3,
	pub material_color: LightColor
}

pub struct Space {
	pub spheres: Vec<Sphere>,
	pub planes: Vec<Plane>,
	pub directional_lights: Vec<DirectionalLight>,
	pub point_lights: Vec<PointLight>
}

