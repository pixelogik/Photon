
// Import requirements
use crate::fundamentals::vec3::*;
use crate::fundamentals::light::*;
use crate::fundamentals::geometry::*;
use super::rays::*;
use rand::prelude::*;

// The amount of rays that are being cast for global illumination per surface point
const GLOBAL_RAYS: i32 = 8;
const fGLOBAL_RAYS: f64 = 8.0;

// This function returns the light being received at the specified point on a body surface
pub fn get_light_at_surface_point(position: &Vector3, normal: &Vector3, space: &Space, recursion_counter: i32) -> LightColor {

	let mut ray_to_light: Vector3; 
	let mut absorbed_light_intensity: f64; 
	let mut result_color = LightColor {r: 0.0, g: 0.0, b: 0.0};	
	let mut direct_light_summands: Vec<WeightedLightColorSummand> = Vec::new(); 
	let mut global_light_summands: Vec<WeightedLightColorSummand> = Vec::new(); 

	for light in &space.directional_lights {
		ray_to_light = v3_normalize(&v3_scale(&light.direction, -1.0));
		absorbed_light_intensity = v3_dot_product(&ray_to_light, &normal);

		// Only if dot product is positive this light is having an impact
		if absorbed_light_intensity > 0.0 {					

			// Determine if the light source is visible or if something is occluding it
			let nearest_hit: Option<RaySurfaceIntersection> = get_nearest_surface_intersection_for_ray(&Ray {origin: position.clone(), direction: ray_to_light }, &space);

			match nearest_hit {
				Some(d) => {
					// Something is occluding the light from the surface point, so not light is being received
				}
				None => {
					
					direct_light_summands.push(WeightedLightColorSummand {
						light_color: light.color,
						weight: absorbed_light_intensity
					});
				}
			}
		}
	}

	let mut distance_to_light: f64; 

	for light in &space.point_lights {
		ray_to_light = v3_delta(&light.position, &position);
		distance_to_light = v3_len(&ray_to_light);
		ray_to_light = v3_normalize(&ray_to_light);

		absorbed_light_intensity = v3_dot_product(&ray_to_light, &normal);

		// Only if dot product is positive this light is having an impact
		if absorbed_light_intensity > 0.0 {					

			// Determine if the light source is visible or if something is occluding it
			let nearest_hit: Option<RaySurfaceIntersection> = get_nearest_surface_intersection_for_ray(&Ray {origin: position.clone(), direction: ray_to_light }, &space);

			match nearest_hit {
				Some(d) => {
					
					// If surface point is further away than the light, the light is being absorbed
					if d.distance > distance_to_light {
						direct_light_summands.push(WeightedLightColorSummand {
							light_color: light.color,
							weight: absorbed_light_intensity
						});	
					}
				}
				None => {
					direct_light_summands.push(WeightedLightColorSummand {
						light_color: light.color,
						weight: absorbed_light_intensity
					});
				}
			}
		}
	}	
	
	if recursion_counter > 0 {
		let basis = v3_compute_basis_for_normal(normal); 
		let mut rng = rand::thread_rng();	

		let mut v1: Vector3; 
		let mut v2: Vector3; 

		let mut r1: f64;
		let mut r2: f64;

		let mut absorbed_light_intensity: f64;
		let mut ray_direction: Vector3; 		

		for k in 0..GLOBAL_RAYS {
			r1 = rng.gen::<f64>()-0.5;
			r2 = rng.gen::<f64>()-0.5;
	
			v1 = v3_scale(&basis.b1, r1);
			v2 = v3_scale(&basis.b2, r2);	

			ray_direction = v3_normalize(&v3_sum(&v3_sum(&v1, &v2), normal));			
			absorbed_light_intensity = v3_dot_product(&ray_direction, normal);
			
		 	let incoming_light_color = get_light_for_backward_ray(&Ray { origin: position.clone(), direction: ray_direction}, space, recursion_counter - 1);

			 global_light_summands.push(WeightedLightColorSummand {
				light_color: incoming_light_color,
				weight: absorbed_light_intensity
			});			
		}
	}

	let mut global_sum = compute_weighted_light_color(&global_light_summands);
	let mut direct_sum = compute_weighted_light_color(&direct_light_summands);	

	return LightColor { r: direct_sum.r + global_sum.r / fGLOBAL_RAYS, g: direct_sum.g + global_sum.g / fGLOBAL_RAYS, b: direct_sum.b + global_sum.b / fGLOBAL_RAYS};
}

// This function returns the light that is being received for the specified backward ray 
pub fn get_light_for_backward_ray(ray: &Ray, space: &Space, recursion_counter: i32) -> LightColor {

	let nearest_hit: Option<RaySurfaceIntersection> = get_nearest_surface_intersection_for_ray(&ray, &space);
	let mut result_color = LightColor {r: 0.0, g: 0.0, b: 0.0};
	
	match nearest_hit {
		Some(d) => {
			let light_at_hit = get_light_at_surface_point(&d.position, &d.normal, &space, recursion_counter);
			result_color.r = d.material_color.r * light_at_hit.r;
			result_color.g = d.material_color.g * light_at_hit.g;
			result_color.b = d.material_color.b * light_at_hit.b;
		}
		None => {
		}
	}

	return result_color; 
}
