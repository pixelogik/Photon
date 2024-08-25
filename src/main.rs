
// We are declaring the modules here
pub mod fundamentals;
pub mod raytracing;

// Import all the things from all the modules
use fundamentals::vec3::*;
use fundamentals::light::*;
use fundamentals::geometry::*;
use raytracing::rendering::*;
use std::sync::Arc;
use std::thread;
use std::sync::mpsc;
use std::time::Instant;

// Program entry point
fn main() { 
	const IMAGE_WIDTH: i32 = 1024;
	const IMAGE_HEIGHT: i32 = 1024;

	// Let's set the camera
	let camera = CameraZ {
		location: Vector3 { x: 0.0, y: 0.0, z: 10.0 },
		distance_to_image_plane: 10.0,
		image_plane_width: 10.0
	};

	let mut spheres: Vec::<Sphere> = Vec::new();

	spheres.push(Sphere {
		center: Vector3 { x: 4.4, y: 3.4, z: -10.0},
		radius: 2.0, 
		material_color: LightColor {r: 1.0, g: 1.0, b: 0.0}
	});

	spheres.push(Sphere {
		center: Vector3 { x: -3.0, y: -3.0, z: -10.0},
		radius: 3.0, 
		material_color: LightColor {r: 0.0, g: 1.0, b: 0.0}
	});

	spheres.push(Sphere {
		center: Vector3 { x: 4.0, y: -4.0, z: -22.0},
		radius: 3.0, 
		material_color: LightColor {r: 1.0, g: 0.0, b: 0.0}
	});

	let mut directional_lights: Vec<DirectionalLight> = Vec::new(); 

	let mut point_lights: Vec<PointLight> = Vec::new(); 

	point_lights.push(PointLight {
		position: Vector3 {x: 0.0, y: 0.0, z: -10.0},
		color: LightColor {r: 0.5, g: 0.5, b: 0.5}
	});

	let mut planes: Vec<Plane> = Vec::new(); 

	planes.push(Plane {
		center: Vector3 { x: -10.0, y:0.0, z: 0.0 },
		normal: Vector3 { x: 1.0, y: 0.0, z: 0.0 },
		material_color: LightColor { r: 1.0, g: 1.0, b: 1.0}
	});

	planes.push(Plane {
		center: Vector3 { x: 10.0, y:0.0, z: 0.0 },
		normal: Vector3 { x: -1.0, y: 0.0, z: 0.0 },
		material_color: LightColor { r: 1.0, g: 1.0, b: 1.0}
	});

	planes.push(Plane {
		center: Vector3 { x: 0.0, y:10.0, z: 0.0 },
		normal: Vector3 { x: 0.0, y: -1.0, z: 0.0 },
		material_color: LightColor { r: 1.0, g: 1.0, b: 1.0}
	});

	planes.push(Plane {
		center: Vector3 { x: 0.0, y:-10.0, z: 0.0 },
		normal: Vector3 { x: 0.0, y: 1.0, z: 0.0 },
		material_color: LightColor { r: 1.0, g: 1.0, b: 1.0}
	});

	planes.push(Plane {
		center: Vector3 { x: 0.0, y: 0.0, z: -36.0 },
		normal: Vector3 { x: 0.0, y: 0.0, z: 1.0 },
		material_color: LightColor { r: 1.0, g: 1.0, b: 1.0}
	});

	// Define space to visualize 
	let space = Space { spheres: spheres, planes: planes, directional_lights: directional_lights, point_lights: point_lights };	

	// Compute visualization
	let pixel_lights = render(space, camera, IMAGE_WIDTH, IMAGE_HEIGHT, 8);
	 
	// Write the PPM header line 
	println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

	// Write the PPM pixels
    for &pixel_light in &pixel_lights {
        let ir = (255.99 * pixel_light.r) as u8;
		let ig = (255.99 * pixel_light.g) as u8;
		let ib = (255.99 * pixel_light.b) as u8;			
		println!("{} {} {}", ir, ig, ib);
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer module

	fn assert_vec3_eq(a: &Vector3, b: &Vector3) {
		assert_eq!(a.x, b.x);
		assert_eq!(a.y, b.y);
		assert_eq!(a.z, b.z);
	}

    #[test]
    fn test_basis_computation() {		
		let tolerance: f64 = 0.00000001;

		// Do 10 random basis computations
		for i in 0..10 {
			let n = v3_random_normal();
			let b = v3_compute_basis_for_normal(&n); 

			// Validate that length of all basis vectors is zero (close enough to it)
			assert_eq!((v3_len(&b.b0)-1.0).abs() < tolerance, true);
			assert_eq!((v3_len(&b.b1)-1.0).abs() < tolerance, true);
			assert_eq!((v3_len(&b.b2)-1.0).abs() < tolerance, true);

			// Validate that dot product of all basis vectors is zero (close enough to it)
			assert_eq!(v3_dot_product(&b.b0, &b.b1).abs() < tolerance, true);
			assert_eq!(v3_dot_product(&b.b0, &b.b2).abs() < tolerance, true);
			assert_eq!(v3_dot_product(&b.b1, &b.b2).abs() < tolerance, true);

			// Validate that the basis vectors are not zero 
			assert_eq!(b.b0.x.abs() + b.b0.y.abs() + b.b0.z.abs() > 0.1, true);
			assert_eq!(b.b1.x.abs() + b.b1.y.abs() + b.b1.z.abs() > 0.1, true);
			assert_eq!(b.b2.x.abs() + b.b2.y.abs() + b.b2.z.abs() > 0.1, true);
		}
	}

    #[test]
    fn test_ray_sphere_intersections() {

				// Sphere 1 
				let ray1 = Ray {origin: Vector3 {x: 0.0, y: 0.0, z: 0.0}, direction: Vector3 {x: 0.0, y: 1.0, z: 0.0}};
				let sphere1 = Sphere {center: Vector3 {x: 0.0, y: 10.0, z: 0.0}, radius: 1.0, material_color: LightColor {r: 1.0, g: 1.0, b: 0.0}};
				let intersections1 = get_ray_sphere_intersections(&ray1, &sphere1);

				match intersections1 {
					Some(d) => {
						println!("Intersection 1: {} / {} / {}\n", d.near.position.x, d.near.position.y, d.near.position.z);
						println!("Intersection 2: {} / {} / {}\n", d.far.position.x, d.far.position.y, d.far.position.z);

						assert_vec3_eq(&d.near.position, &Vector3{x: 0.0, y: 9.0, z: 0.0});
						assert_vec3_eq(&d.far.position, &Vector3{x: 0.0, y: 11.0, z: 0.0});
					}
					None => {
						panic!("Intersections is None should be some");
					}
				}

				// Sphere 2
				let ray2 = Ray {origin: Vector3 {x: 0.0, y: -5.0, z: 0.0}, direction: Vector3 {x: 1.0, y: 0.0, z: 0.0}};
				let sphere2 = Sphere {center: Vector3 {x: 5.0, y: -5.0, z: 0.0}, radius: 2.5, material_color: LightColor {r: 1.0, g: 1.0, b: 0.0}};
				
				let intersections2 = get_ray_sphere_intersections(&ray2, &sphere2);

				match intersections2 {
					Some(d) => {
						println!("Intersection 1: {} / {} / {}\n", d.near.position.x, d.near.position.y, d.near.position.z);
						println!("Intersection 2: {} / {} / {}\n", d.far.position.x, d.far.position.y, d.far.position.z);

						assert_vec3_eq(&d.near.position, &Vector3{x: 2.5, y: -5.0, z: 0.0});
						assert_vec3_eq(&d.far.position, &Vector3{x: 7.5, y: -5.0, z: 0.0});
					}
					None => {
						panic!("Intersections is None should be some");
					}
				}

				let ray3 = Ray {origin: Vector3 {x: 5.0, y: 0.0, z: 0.0}, direction: Vector3 {x: 0.0, y: 1.0, z: 0.0}};
				let intersections3 = get_ray_sphere_intersections(&ray3, &sphere2);
				assert_eq!(intersections3.is_none(), true);

				let ray4 = Ray {origin: Vector3 {x: 0.0, y: -2.49, z: 0.0}, direction: Vector3 {x: 1.0, y: 0.0, z: 0.0}};
				let intersections4 = get_ray_sphere_intersections(&ray4, &sphere2);
				assert_eq!(intersections4.is_none(), true);
    }
}

