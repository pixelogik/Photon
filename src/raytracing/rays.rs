
// Import requirements
use crate::fundamentals::vec3::*;
use crate::fundamentals::light::*;
use crate::fundamentals::geometry::*;

// Light rays, backward rays, all the rays
pub struct Ray {
	pub origin: Vector3,
	pub direction: Vector3
}

// Result type for ray-surface-intersection calculation
pub struct RaySurfaceIntersection {
	pub position: Vector3, 
	pub normal: Vector3,
	pub distance: f64,
	pub material_color: LightColor
}

// Result type for ray-surface-intersection calculation
pub struct RaySurfaceIntersections {
	// The intersection nearer to the ray origin
	pub near: RaySurfaceIntersection,
	// The intersection further away from the ray origin then near
	pub far: RaySurfaceIntersection
}

// Computes the two intersections of an ray and a sphere, if existing. Otherwise returns. Ray must be normalized!
pub fn get_ray_sphere_intersections(ray: &Ray, sphere: &Sphere) -> Option<RaySurfaceIntersections> {
	let ray_origin_to_sphere_center = v3_delta(&sphere.center, &ray.origin);
	let projection_onto_normalized_ray_len = v3_dot_product(&ray.direction, &ray_origin_to_sphere_center);

	// If the center of the sphere is behind the ray origin we handle it as non-hit for now. For later when rays might
	// have origins within a sphere we have to get rid of this optimization.
	if projection_onto_normalized_ray_len < 0.0 {
		return None;
	}

	let projection_onto_normalized_ray = v3_sum(&ray.origin, &v3_scale(&ray.direction, projection_onto_normalized_ray_len)); 
	let sphere_center_to_projection = v3_delta(&projection_onto_normalized_ray, &sphere.center);
	let sphere_radius_squared = sphere.radius * sphere.radius; 
	let k = sphere_center_to_projection;
	let sphere_center_to_projection_len_squared = k.x*k.x + k.y*k.y + k.z*k.z;

	// If the projection of the center into the ray is further away than the radius of the sphere, there is no hit.
	if sphere_center_to_projection_len_squared > sphere_radius_squared {
		return None;
	}

	let distance_from_projection_to_hits = (sphere_radius_squared - sphere_center_to_projection_len_squared).sqrt(); 
	let near_distance_to_ray_origin = projection_onto_normalized_ray_len - distance_from_projection_to_hits;
	let far_distance_to_ray_origin = projection_onto_normalized_ray_len + distance_from_projection_to_hits;
	let near = v3_sum(&ray.origin, &v3_scale(&ray.direction, near_distance_to_ray_origin)); 
	let far = v3_sum(&ray.origin, &v3_scale(&ray.direction, far_distance_to_ray_origin));

	let near_normal = v3_normalize( &v3_delta(&near, &sphere.center));
	let far_normal = v3_normalize( &v3_delta(&far, &sphere.center));
		
	return Some(
		RaySurfaceIntersections {
			near: RaySurfaceIntersection {position: near, normal: near_normal, distance: near_distance_to_ray_origin, material_color: sphere.material_color}, 
			far: RaySurfaceIntersection {position: far, normal: far_normal, distance: far_distance_to_ray_origin, material_color: sphere.material_color}
		}
	);
}

// Computes the intersection of an ray and a plane. 
pub fn get_ray_plane_intersection(ray: &Ray, plane: &Plane) -> Option<RaySurfaceIntersection> {
	let plane_center_to_ray_origin = v3_delta(&ray.origin, &plane.center);
	let dot_a = v3_dot_product(&plane_center_to_ray_origin, &plane.normal);	
	let dot_b = v3_dot_product(&ray.direction, &plane.normal);	

	// In this case the ray is parallel to the plane and there is no hit (we ignore the special case that the ray is within the plane)
	if dot_b == 0.0 {
		return None;
	}

	let L = - dot_a / dot_b; 

	// If the hit point is "behind" the ray origin, there is not hit. The threshold of 0.01 helps with rounding errors
	if L <= 0.01 {
		return None;
	}

	let hit_position = v3_sum(&ray.origin, &v3_scale(&ray.direction, L));

	return Some(
		RaySurfaceIntersection {
			position: hit_position, 
			normal: plane.normal.clone(),
			distance: L,
			material_color: plane.material_color
		}
	);

	/*
	CALCULATION: 

	p = plane center to ray origin 
	r = ray direction 

	point on ray from perspective of plane center:
	P = p + r * L

	projection of point on ray to plane normal (distance to plane)
	d = P.x * n.x + P.y * n.y + P.z * n.z 

	hit on plane is at d = 0
	0 = P.x * n.x + P.y * n.y + P.z * n.z 
	0 = (p.x + r.x * L) * n.x + (p.y + r.y * L) * n.y + (p.z + r.z * L) * n.z
	0 = p.x*n.x + r.x*L*n.x + p.y*n.y + r.y*L*n.y + p.z*n.z + r.z*L*n.z
	- (p.x*n.x+p.y*n.y+p.z*n.z) = L * (r.x*n.x + r.y*n.y + r.z*n.z)
	L = - (p.x*n.x+p.y*n.y+p.z*n.z) / (r.x*n.x + r.y*n.y + r.z*n.z)
	*/
}

// This function returns the nearest surface intersection for a ray. If there is no intersection, None is returned.
pub fn get_nearest_surface_intersection_for_ray(ray: &Ray, space: &Space) -> Option<RaySurfaceIntersection> {

	let mut nearest_hit: Option<RaySurfaceIntersection> = None;
	let mut nearest_hit_distance: f64 = f64::MAX;	

	for sphere in &space.spheres {
		match get_ray_sphere_intersections(ray, &sphere) {
			Some(d) => {
				if d.near.distance < nearest_hit_distance {
					nearest_hit_distance = d.near.distance;
					nearest_hit = Some(d.near);
				}
			}
			None => {
			}
		}
	}

	for plane in &space.planes {
		match get_ray_plane_intersection(ray, &plane) {
			Some(d) => {
				if d.distance < nearest_hit_distance {
					nearest_hit_distance = d.distance;
					nearest_hit = Some(d);
				}
			}
			None => {
			}
		}
	}

	return nearest_hit; 
}
