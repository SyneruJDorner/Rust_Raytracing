/*
REF SITES:
https://raytracing.github.io/books/RayTracingInOneWeekend.html
https://github.com/ryankaplan/vec3/blob/master/src/lib.rs
http://www.cplusplus.com/forum/general/243435/
https://github.com/justindd1994/rtv1/tree/master/Constructs/Vector/Vector3/operators
https://github.com/dps/rust-raytracer
https://www.codersblock.org/blog/multiplayer-fps-part-7

//Used to make sure Matrix calcaulations are correct
https://www.andre-gaschler.com/rotationconverter/

//Matching up rays direction with camera's direction
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

use vec3::Vec3 as Vec3;
//use ray::Ray as Ray;
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
    const FOV: f32 = 75.0;
    const IMAGE_WIDTH:i32 = 1920;
    const IMAGE_HEIGHT:i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    //World
    let mut world: HittableList = HittableList::new();
    
    let material_ground = Material::Lambertian(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Material::Lambertian(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left   = Material::Glass(Glass::new(1.5));
    let material_right  = Material::Metal(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));

    world.add(Sphere::new(Vec3::new(0.0,      -100.5,     -1.0),  100.0,  material_ground));
    world.add(Sphere::new(Vec3::new(0.0,      0.0,        -1.0),  0.5,    material_center));
    world.add(Sphere::new(Vec3::new(-1.0,     0.0,        -1.0),  0.5,    material_left));
    world.add(Sphere::new(Vec3::new(-1.0,     0.0,        -1.0),  -0.4,   material_left));
    world.add(Sphere::new(Vec3::new(1.0,      0.0,        -1.0),  0.5,    material_right));

    // Camera
    let mut camera = Camera::new();
    camera.set_position(Vec3::new(150.0, -200.0, -100.0));
    camera.set_rotation(Vec3::new(45.0, 45.0, 0.0));
    camera.set_scale(Vec3::new(1.0, 1.0, 1.0));
    camera.set_aspect_ratio(ASPECT_RATIO);
    camera.set_field_of_view(FOV);
    camera.prepare_ray();

    //Render
    camera.trace(world, IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH)
}
