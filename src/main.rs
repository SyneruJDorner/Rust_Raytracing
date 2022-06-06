/*
REF SITES:
https://raytracing.github.io/books/RayTracingInOneWeekend.html
https://github.com/ryankaplan/vec3/blob/master/src/lib.rs
http://www.cplusplus.com/forum/general/243435/
https://github.com/justindd1994/rtv1/tree/master/Constructs/Vector/Vector3/operators
https://github.com/dps/rust-raytracer

//Used to make sure Matrix calcaulations are correct
https://www.andre-gaschler.com/rotationconverter/

//Matching up rays ddirection with camera's direction
https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-generating-camera-rays/generating-camera-rays
*/

#[path = "math/constants/constants.rs"] mod constants;
#[path = "math/vec3/vec3.rs"] mod vec3;
#[path = "math/matrix/matrix4x4.rs"] mod matrix4x4;
#[path = "math/transform/transform.rs"] mod transform;
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
#[path = "raytracing/material/submaterials/glass.rs"] mod glass;

use map_3d::deg2rad;

use matrix4x4::Matrix4x4;
use vec3::Vec3 as Vec3;
use ray::Ray as Ray;
use camera::Camera as Camera;
use hittablelist::HittableList as HittableList;
use sphere::Sphere as Sphere;
use material::Material as Material;
use material::submaterials::lamertian::Lambertian as Lambertian;
#[allow(unused_imports)]
use material::submaterials::metal::Metal as Metal;
#[allow(unused_imports)]
use material::submaterials::glass::Glass as Glass;

fn main()
{
    //Image
    const ASPECT_RATIO:f32 = 16.0 / 9.0;
    const FOV: f32 = 90.0;
    const IMAGE_WIDTH:i32 = 800;
    const IMAGE_HEIGHT:i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 10;
    const MAX_DEPTH: i32 = 5;

    //World
    let r = (constants::PI / 4.0).cos() as f32;
    let mut world: HittableList = HittableList::new();

    let material_left  = Material::Lambertian(Lambertian::new(Vec3::new(0.0, 0.0, 1.0)));
    let material_right = Material::Lambertian(Lambertian::new(Vec3::new(1.0, 0.0, 0.0)));

    world.add(Sphere::new(Vec3::new(-r, 0.0, -1.0), r, material_left));
    world.add(Sphere::new(Vec3::new( r, 0.0, -1.0), r, material_right));
    
    // let material_ground = Material::Lambertian(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    // let material_center = Material::Lambertian(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    // let material_left   = Material::Glass(Glass::new(1.5));
    // let material_right  = Material::Metal(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));

    // world.add(Sphere::new(Vec3::new(0.0,      -100.5,     -1.0),  100.0,  material_ground));
    // world.add(Sphere::new(Vec3::new(0.0,      0.0,        -1.0),  0.5,    material_center));
    // world.add(Sphere::new(Vec3::new(-1.0,     0.0,        -1.0),  0.5,    material_left));
    // world.add(Sphere::new(Vec3::new(-1.0,     0.0,        -1.0),  -0.4,   material_left));
    // world.add(Sphere::new(Vec3::new(1.0,      0.0,        -1.0),  0.5,    material_right));

    // Camera
    let camera = Camera::new();
    camera.set_position(Vec3::new(5.0, 0.0, 20.0));
    camera.set_rotation(Vec3::new(0.0, 45.0, 0.0));
    camera.set_scale(Vec3::new(1.0, 1.0, 1.0));
    camera.set_aspect_ratio(ASPECT_RATIO);
    camera.set_field_of_view(FOV);

    // println!("{} {} {}\n", camera.transform.position.x, camera.transform.position.y, camera.transform.position.z);
    // println!("{} {} {}\n", camera.transform.rotation.x, camera.transform.rotation.y, camera.transform.rotation.z);
    // println!("{} {} {}\n", camera.transform.scale.x, camera.transform.scale.y, camera.transform.scale.z);

    //Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    //let cameraToWorld: Matrix4x4; 
    let scale: f32 = (deg2rad((FOV * 0.5).into()) as f64).tan() as f32; 
    //let aspect_ratio = IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32; 
    let camera_to_world: Matrix4x4 = Matrix4x4::identity();
    let ray_origin = camera_to_world.multiply_vec3_matrix(Vec3::new(100.0, 0.0, -100.0));
    for y in (0..IMAGE_HEIGHT).rev()
    {
        for x in 0..IMAGE_WIDTH
        {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL
            {
                let x = (2.0 * (x as f32 + 0.5) / IMAGE_WIDTH as f32 - 1.0) * ASPECT_RATIO * scale;
                let y = (1.0 - 2.0 * (y as f32 + 0.5) / IMAGE_HEIGHT as f32) * scale;
                let dir: Vec3 = camera_to_world.multiply_dir_matrix(Vec3::new(x, y, -1.0)).normalize();
                let ray = Ray::new(ray_origin, dir);
                //let r = camera.get_ray(u, v);
                //castRay(orig, dir, objects, lights, options, 0)
                let new_color = pixel_color + Ray::calcaulte_ray(&ray, &world, MAX_DEPTH);
                pixel_color = new_color;
            }
            color::write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }



    // for y in (0..IMAGE_HEIGHT).rev()
    // {
    //     for x in 0..IMAGE_WIDTH
    //     {
    //         let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
    //         for _ in 0..SAMPLES_PER_PIXEL
    //         {
    //             let u = (((x as f32) + utils::random_float(0.0, 1.0)) / (IMAGE_WIDTH - 1) as f32) as f32;
    //             let v = (((y as f32) + utils::random_float(0.0, 1.0)) / (IMAGE_HEIGHT - 1) as f32) as f32;
    //             let r = camera.get_ray(u, v);
    //             let new_color = pixel_color + Ray::calcaulte_ray(&r, &world, MAX_DEPTH);
    //             pixel_color = new_color;
    //         }
    //         color::write_color(pixel_color, SAMPLES_PER_PIXEL);
    //     }
    // }
}
