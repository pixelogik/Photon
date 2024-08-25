
// Import requirements
use crate::fundamentals::vec3::*;
use crate::fundamentals::light::*;
use crate::fundamentals::geometry::*;
use super::rays::*;
use super::tracing::*;
use rand::prelude::*;
use std::sync::Arc;
use std::thread;
use std::sync::mpsc;
use std::time::Instant;

// Number of additional randomly chosen sub-pixel rays
const RANDOM_SUBPIXEL_RAYS: i32 = 32;
const fRANDOM_SUBPIXEL_RAYS: f64 = 32.0;

// Perspective camera, having looking into the minus z direction, having the up vector looking into the y direction
pub struct CameraZ {
	pub location: Vector3, 
	pub distance_to_image_plane: f64,
	pub image_plane_width: f64
}

// Main rendering function. Renders the full image. The thread_count=N specifies how many threads will be used. 
// The image plane will be split in N non-overlapping intervals, each being computed on a dedicated thread. 
pub fn render(space: Space, camera: CameraZ, width: i32, height: i32, thread_count: i32) -> Vec<LightColor> {

	// First divide the height of the image into threadCount non-overlapping intervals 
	let interval_size = height / thread_count; 

	let mut join_handles: Vec<thread::JoinHandle<()>> = Vec::new();
	let mut channel_receivers: Vec<mpsc::Receiver<Vec<LightColor>>> = Vec::new();
	let mut pixels: Vec<LightColor> = Vec::new();

	let arc_space = Arc::new(space);
	let arc_camera = Arc::new(camera);

	let mut interval_start = 0; 

	for _ in 0..thread_count {
		let (tx, rx) = mpsc::channel();
		channel_receivers.push(rx);

		let t_space = Arc::clone(&arc_space);
		let t_camera = Arc::clone(&arc_camera);

		let t_join_handle = thread::spawn(move || {
			let t_pixel_lights = render_interval(t_space, t_camera, width, height, interval_start, interval_size);
			tx.send(t_pixel_lights).unwrap();
		});

		join_handles.push(t_join_handle);
		interval_start += interval_size;
	}

	for c_receiver in &channel_receivers {
		let c_pixel_lights = c_receiver.recv().unwrap();
		pixels.extend(c_pixel_lights);
	}

	for join_handler in join_handles {
		join_handler.join().unwrap();	
	}

	return pixels; 
}

// Renders a vertical interval of the image plane 
fn render_interval(a_space: Arc<Space>, a_camera: Arc<CameraZ>, width: i32, height: i32, y_start_index: i32, y_count: i32) -> Vec<LightColor> {

	let space: &Space = a_space.as_ref();
    let camera: &CameraZ = a_camera.as_ref();

	let DX: f64 = (width-1) as f64;
	let DY: f64 = (height-1) as f64;

	let mut pixels: Vec<LightColor> = Vec::new();

	// Compute some variables we need for iterating through the image plane pixels
	let pixel_size = camera.image_plane_width / (width as f64);
	let pixel_half = pixel_size * 0.5;
	let w_half = (width as f64) * 0.5;
	let h_half = (height as f64) * 0.5;
	let x_start = camera.location.x - pixel_size*w_half + pixel_half; 
	let y_start = camera.location.y + pixel_size*h_half - pixel_half;
	let plane_z = camera.location.z - camera.distance_to_image_plane;

	// These vars are being set to the pixel center during the iteration
	let mut pixel_ray = Ray {origin: Vector3 {x: x_start, y: y_start, z: plane_z}, direction: Vector3 {x: 0.0, y: 0.0, z: 0.0}};
	let mut ray_len: f64;
	let mut row_y: f64 = y_start-pixel_size*(y_start_index as f64);
	let mut col_x: f64; 
	let mut pixel_light: LightColor;

	let mut pixel_measurements: Vec<WeightedLightColorSummand> = Vec::new(); 
	let mut pixel_measurement_positions: Vec<Vector3> = Vec::new(); 

	let mut rng = rand::thread_rng();	
	let mut r1: f64;
	let mut r2: f64;
	
	let W: f64 = 1.0 / (1.0 + fRANDOM_SUBPIXEL_RAYS);

	for y in (y_start_index..(y_start_index+y_count)).rev() {

		col_x = x_start; 

		for x in 0..width {

			pixel_measurements.clear();
			pixel_measurement_positions.clear();

			// This is the default ray send from the center of the pixel 
			pixel_measurement_positions.push(Vector3 {
				x: col_x, 
				y: row_y, 
				z: 0.0 
			});

			// We also add random subpixel ray origins 
			for k in 0..RANDOM_SUBPIXEL_RAYS {
				r1 = (rng.gen::<f64>()-0.5)*2.0;
				r2 = (rng.gen::<f64>()-0.5)*2.0;

				pixel_measurement_positions.push(Vector3 {
					x: col_x + pixel_half*r1, 
					y: row_y + pixel_half*r2, 
					z: 0.0 
				});				
			}
			
			// For each measurement origin cast a ray 
			for pm in &pixel_measurement_positions {
				pixel_ray.origin.x = pm.x;
				pixel_ray.origin.y = pm.y; 
	
				pixel_ray.direction.x = pixel_ray.origin.x - camera.location.x;
				pixel_ray.direction.y = pixel_ray.origin.y - camera.location.y;
				pixel_ray.direction.z = pixel_ray.origin.z - camera.location.z;
	
				ray_len = v3_len(&pixel_ray.direction);
				pixel_ray.direction.x /= ray_len;
				pixel_ray.direction.y /= ray_len;
				pixel_ray.direction.z /= ray_len;
	
				pixel_light = get_light_for_backward_ray(&pixel_ray, &space, 1);

				pixel_measurements.push(WeightedLightColorSummand {
					light_color: pixel_light.clone(),
					weight: W
				});								
			}

			pixel_light = compute_weighted_light_color(&pixel_measurements);
						
			pixels.push(pixel_light);

			col_x += pixel_size;
		}

		row_y -= pixel_size;
	}	

	return pixels;
}

