/*
REF SITES:
https://raytracing.github.io/books/RayTracingInOneWeekend.html
https://github.com/ryankaplan/vec3/blob/master/src/lib.rs
http://www.cplusplus.com/forum/general/243435/
https://github.com/justindd1994/rtv1/tree/master/Constructs/Vector/Vector3/operators
https://github.com/dps/rust-raytracer
*/

#[path = "math/constants/constants.rs"] mod constants;
#[path = "math/vec3/vec3.rs"] mod vec3;
#[path = "math/utils/utils.rs"] mod utils;
#[path = "raytracing/color/color.rs"] mod color;
#[path = "raytracing/ray/ray.rs"] mod ray;
#[path = "raytracing/camera/camera.rs"] mod camera;
#[path = "raytracing/hittable/hittablelist.rs"] mod hittablelist;
#[path = "raytracing/hittable/hittable.rs"] mod hittable;
#[path = "raytracing/shapes/sphere.rs"] mod sphere;
#[path = "raytracing/material/material.rs"] mod material;
#[path = "raytracing/material/submaterials/lamertian.rs"] mod lamberian;
#[path = "raytracing/material/submaterials/metal.rs"] mod metal;

use material::Material as Material;
use material::submaterials::lamertian::Lambertian as Lambertian;
use material::submaterials::metal::Metal as Metal;

use crate::material::Scatterable;

fn ray_color(r: &ray::Ray, world: &hittablelist::HittableList, depth: i32)  -> vec3::Vec3
{
    if depth <= 0
    {
        return vec3::Vec3::new(0.0, 0.0, 0.0);
    }
    
    let closest_object = world.hit_closest_object(r, 0.001, constants::INFINITY as f32);
    
    //Handle all reflections
    if closest_object.is_some()
    {
        let hit_obj = closest_object.unwrap();
        let scatter = hit_obj.material.scatter(&r, &hit_obj);
        if scatter.is_some()
        {
            let scattered = scatter.unwrap().0;
            let attenuation = scatter.unwrap().1;
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return vec3::Vec3::new(1.0, 1.0, 1.0);
    }

    let unit_direction = vec3::Vec3::normalize(&r.direction());
    let t = 0.5*(unit_direction.y + 1.0);
    let color_1 = vec3::Vec3::new(1.0, 1.0, 1.0);
    let color_2 = vec3::Vec3::new(0.5, 0.7, 1.0);
    let sky_color = (1.0 - t) * color_1 + t * color_2;
    return sky_color;
}


fn main()
{
    //Image
    const ASPECT_RATIO:f32 = 16.0 / 9.0;
    const IMAGE_WIDTH:i32 = 1920;
    const IMAGE_HEIGHT:i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    //World
    let mut world: hittablelist::HittableList = hittablelist::HittableList::new();
    
    let material_ground = Material::Lambertian(Lambertian::new(vec3::Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Material::Lambertian(Lambertian::new(vec3::Vec3::new(0.7, 0.3, 0.3)));
    let material_left   = Material::Metal(Metal::new(vec3::Vec3::new(0.8, 0.8, 0.8), 0.3));
    let material_right  = Material::Metal(Metal::new(vec3::Vec3::new(0.8, 0.6, 0.2), 1.0));

    world.add(sphere::Sphere::new(vec3::Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground));
    world.add(sphere::Sphere::new(vec3::Vec3::new(0.0, 0.0, -1.0), 0.5, material_center));
    world.add(sphere::Sphere::new(vec3::Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(sphere::Sphere::new(vec3::Vec3::new(1.0, 0.0, -1.0), 0.5, material_right));

    // Camera
    let cam: camera::Camera = camera::Camera::new();

    //Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for y in (0..IMAGE_HEIGHT).rev()
    {
        for x in 0..IMAGE_WIDTH
        {
            let mut pixel_color = vec3::Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL
            {
                let u = (((x as f32) + utils::random_float(0.0, 1.0)) / (IMAGE_WIDTH-1) as f32) as f32;
                let v = (((y as f32) + utils::random_float(0.0, 1.0)) / (IMAGE_HEIGHT-1) as f32) as f32;
                let r = cam.get_ray(u, v);
                let new_color = pixel_color + ray_color(&r, &world, MAX_DEPTH);
                pixel_color = new_color;
            }
            color::write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}
