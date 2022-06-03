/*
REF SITES:
https://raytracing.github.io/books/RayTracingInOneWeekend.html
https://github.com/ryankaplan/vec3/blob/master/src/lib.rs
http://www.cplusplus.com/forum/general/243435/
https://github.com/justindd1994/rtv1/tree/master/Constructs/Vector/Vector3/operators
https://github.com/dps/rust-raytracer
*/

#[path = "math/constants/constants.rs"] mod constants;
#[path = "math/utils/utils.rs"] mod utils;
#[path = "math/vec3/vec3.rs"] mod vec3;
#[path = "math/color/color.rs"] mod color;
#[path = "math/ray/ray.rs"] mod ray;
#[path = "math/camera/camera.rs"] mod camera;
#[path = "math/hittable/hittablelist.rs"] mod hittablelist;
#[path = "math/hittable/hittable.rs"] mod hittable;
#[path = "math/shapes/sphere.rs"] mod sphere;

fn ray_color(r: &ray::Ray, world: &hittablelist::HittableList, depth: i32)  -> vec3::Vec3
{
    let closest_object = world.hit_closest_object(r, 0.001, constants::INFINITY as f32);

    if depth <= 0
    {
        return vec3::Vec3::new(0.0, 0.0, 0.0);
    }

    if closest_object != None
    {
        let hit_obj = closest_object.unwrap();
        let target: vec3::Vec3 = hit_obj.point + vec3::Vec3::random_in_hemisphere(&hit_obj.normal);
        let bounce_ray = ray::Ray::new(hit_obj.point, target - hit_obj.point);
        return 0.5 * ray_color(&bounce_ray, world, depth - 1);
    }
    let unit_direction = vec3::Vec3::normalize(&r.direction());
    let t = 0.5*(unit_direction.y + 1.0);
    let color_1 = vec3::Vec3::new(1.0, 1.0, 1.0);
    let color_2 = vec3::Vec3::new(0.5, 0.7, 1.0);
    let sky_color = (1.0 - t) * color_1 + t * color_2;
    return sky_color
}


fn main()
{
    //Time
    // use std::time::Instant;
    // let now = Instant::now();

    //Image
    const ASPECT_RATIO:f32 = 16.0 / 9.0;
    const IMAGE_WIDTH:i32 = 1920;
    const IMAGE_HEIGHT:i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    //World
    let mut world: hittablelist::HittableList = hittablelist::HittableList::new();
    world.add(sphere::Sphere::new(vec3::Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(sphere::Sphere::new(vec3::Vec3::new(0.0, -100.5, -1.0), 100.0));

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
