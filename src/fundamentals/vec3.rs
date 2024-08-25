
// Import random generator 
use rand::prelude::*;

// Positions and directions in space are using this structure
#[derive(Copy, Clone)]
pub struct Vector3 {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

// Basis in three dimensional vector space
pub struct Basis3 {
	pub b0: Vector3,
	pub b1: Vector3,
	pub b2: Vector3,
}

// Computes dot product of two vectors
pub fn v3_dot_product(a: &Vector3, b: &Vector3) -> f64 {
	return a.x*b.x + a.y*b.y + a.z*b.z; 
}

// Computes length of vector
pub fn v3_len(a: &Vector3) -> f64 {
	return (a.x*a.x+a.y*a.y+a.z*a.z).sqrt();
}

// Normalizes vector 
pub fn v3_normalize(a: &Vector3) -> Vector3 {
	let len = v3_len(a);
	return Vector3 {x: a.x/len, y: a.y/len, z: a.z/len};
}

// Computes a-b
pub fn v3_delta(a: &Vector3, b: &Vector3) -> Vector3 {
	Vector3 {x: a.x-b.x, y: a.y-b.y, z: a.z-b.z}
}

// Computes a+b
pub fn v3_sum(a: &Vector3, b: &Vector3) -> Vector3 {
	Vector3 {x: a.x+b.x, y: a.y+b.y, z: a.z+b.z}
}

// Computes a*s
pub fn v3_scale(a: &Vector3, s: f64) -> Vector3 {
	Vector3 {x: a.x*s, y: a.y*s, z: a.z*s}
}

// Computes cross product of two vectors
pub fn v3_cross_product(a: &Vector3, b: &Vector3) -> Vector3 {
	return Vector3 {
		x: a.y*b.z - a.z*b.y,
		y: a.z*b.x - a.x*b.z,
		z: a.x*b.y - a.y*b.x
	};
}

// Returns random vector (non-normalized)
pub fn v3_random() -> Vector3 {
	let mut rng = rand::thread_rng();	
	return Vector3 { x: rng.gen::<f64>(), y: rng.gen::<f64>(), z: rng.gen::<f64>()};
}

// Returns random normalized vector 
pub fn v3_random_normal() -> Vector3 {	
	return v3_normalize(&v3_random());
}

// Computes normalized basis in which the first axis vector is equal to n
pub fn v3_compute_basis_for_normal(n: &Vector3) -> Basis3 {

	// Add random vector to the normal 
	let some_random_point = v3_sum(&n, &v3_random());

	// Project random position onto the normal to get the distance to the plane the normal is defining
	let projection_of_point_onto_n: f64 = v3_dot_product(&n, &some_random_point);

	// Get point on plane by subtracting the negatively scaled normal
	let point_on_plane = v3_sum(&some_random_point, &v3_scale(&n, -projection_of_point_onto_n));

	// Get second axis vector as normalized vector to that point on the plane 
	let b1 = v3_normalize(&point_on_plane);

	// The third axis vector we get by computing the cross product of the normal and b1
	let b2 = v3_normalize(&v3_cross_product(&n, &b1));

	return Basis3 {
		b0: n.clone(),
		b1: b1,
		b2: b2
	}
}
