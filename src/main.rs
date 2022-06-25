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

//Working with world space conversion
https://github.com/isaacery/DistributedRaytracer/blob/bf05f148d364afd7eef274bcdf5bbd3ead6c1b24/RayTracer/core/RayTracer.cpp
https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-generating-camera-rays/generating-camera-rays

//Working with Sheering, Scaling, Rotation and Position
https://www.youtube.com/watch?v=uRJCi0dlU4U
https://github.com/iweinbau/Ray-Tracing
*/

#[path = "./lib.rs"] mod lib;
use crate::lib::*;


// #[path = "./deps.rs"] mod deps;
// use deps::Settings as Settings;
// use deps::Camera as Camera;
// use deps::HittableList as HittableList;
// use deps::Material as Material;
// use deps::Lambertian as Lambertian;
// use deps::Metal as Metal;
// use deps::Glass as Glass;
// use deps::Emmision as Emmision;
// use deps::Sphere as Sphere;

// #[path = "settings/settings.rs"] mod settings;
// #[path = "math/constant/constant.rs"] mod constant;
// #[path = "math/tuple/tuple.rs"] mod tuple;
// #[path = "math/tuple/point/point.rs"] mod point;
// #[path = "math/tuple/vector3/vector3.rs"] mod vector3;
// #[path = "math/tuple/color/color.rs"] mod color;
// #[path = "math/matrix/matrix.rs"] mod matrix;
// #[path = "math/transform/transform.rs"] mod transform;
// #[path = "math/utils/utils.rs"] mod utils;
// #[path = "raytracing/ray/ray.rs"] mod ray;
// #[path = "raytracing/camera/camera.rs"] mod camera;
// #[path = "raytracing/material/material.rs"] mod material;
// #[path = "raytracing/material/submaterials/lambertian.rs"] mod lambertian;
// #[path = "raytracing/material/submaterials/metal.rs"] mod metal;
// #[path = "raytracing/material/submaterials/glass.rs"] mod glass;
// #[path = "raytracing/hittable/hittable.rs"] mod hittable;
// #[path = "raytracing/hittable/hittablelist.rs"] mod hittablelist;
// #[path = "raytracing/shapes/sphere.rs"] mod sphere;
// // #[path = "raytracing/shapes/plane.rs"] mod plane;

// use settings::Settings as Settings;
// use camera::Camera as Camera;
// use hittablelist::HittableList as HittableList;
// use material::Material as Material;
// use material::submaterials::lambertian::Lambertian as Lambertian;
// use material::submaterials::metal::Metal as Metal;
// use material::submaterials::glass::Glass as Glass;
// use material::submaterials::emmision::Emmision as Emmision;
// use sphere::Sphere as Sphere;

#[allow(dead_code)]
fn lighting_demo() -> HittableList
{
    let mut world: HittableList = HittableList::new();

    let mut ground_sphere = Sphere::new();
    ground_sphere.transform.set_scale(1000.0, 1000.0, 1000.0);
    ground_sphere.transform.set_position(0.0, -1000.0, 0.0);
    let ground_material = Material::Lambertian(Lambertian::new(0.5, 0.5, 0.5));
    ground_sphere.material = ground_material;
    world.add(Box::new(ground_sphere));

    let mut glass_sphere = Sphere::new();
    glass_sphere.transform.set_scale(1.0, 1.0, 1.0);
    glass_sphere.transform.set_rotation(0.0, 0.0, 0.0);
    glass_sphere.transform.set_position(0.0, 1.0, 3.0);
    let glass_mat = Material::Glass(Glass::new(1.5));
    glass_sphere.material = glass_mat;
    world.add(Box::new(glass_sphere));

    let mut albedo_sphere = Sphere::new();
    albedo_sphere.transform.set_scale(1.0, 1.0, 1.0);
    albedo_sphere.transform.set_rotation(0.0, 0.0, 0.0);
    albedo_sphere.transform.set_position(-4.0, 1.0, 0.0);
    let albedo_mat = Material::Lambertian(Lambertian::new(0.4, 0.2, 0.1));
    albedo_sphere.material = albedo_mat;
    world.add(Box::new(albedo_sphere));

    let mut metal_sphere = Sphere::new();
    metal_sphere.transform.set_scale(1.0, 1.0, 1.0);
    metal_sphere.transform.set_rotation(0.0, 90.0, 0.0);
    metal_sphere.transform.set_position(4.0, 1.0, 0.0);
    let metal_mat = Material::Metal(Metal::new(0.7, 0.6, 0.5, 0.0));
    metal_sphere.material = metal_mat;
    world.add(Box::new(metal_sphere));

    let mut albedo_sphere = Sphere::new();
    albedo_sphere.transform.set_scale(1.0, 1.0, 1.0);
    albedo_sphere.transform.set_rotation(0.0, -90.0, 0.0);
    albedo_sphere.transform.set_position(-2.0, 1.0, -6.0);
    let albedo_mat = Material::Emmition(Emmision::new(1.0, 0.0, 0.0));
    albedo_sphere.material = albedo_mat;
    world.add(Box::new(albedo_sphere));

    let mut albedo_sphere = Sphere::new();
    albedo_sphere.transform.set_scale(1.0, 1.0, 1.0);
    albedo_sphere.transform.set_rotation(0.0, 0.0, 0.0);
    albedo_sphere.transform.set_position(0.0, 1.0, -6.0);
    let albedo_mat = Material::Lambertian(Lambertian::new(0.0, 1.0, 0.0));
    albedo_sphere.material = albedo_mat;
    world.add(Box::new(albedo_sphere));

    let mut albedo_sphere = Sphere::new();
    albedo_sphere.transform.set_scale(1.0, 1.0, 1.0);
    albedo_sphere.transform.set_rotation(0.0, 0.0, 0.0);
    albedo_sphere.transform.set_position(2.0, 1.0, -6.0);
    let albedo_mat = Material::Emmition(Emmision::new(0.0, 0.0, 1.0));
    albedo_sphere.material = albedo_mat;
    world.add(Box::new(albedo_sphere));

    // let difflight_1 = Material::Emmition(Emmision::new(1.0, 0.0, 0.0));
    // let mut transform_plane = transform::Transform::new();
    // transform_plane.set_scale(1.0, 1.0, 1.0);
    // transform_plane.set_rotation(0.0, 90.0, 0.0);
    // transform_plane.set_position(-5.0, 3.0, -5.0);
    // world.add(Box::new(Plane::new(transform_plane, difflight_1)));

    // let difflight_2 = Material::Emmition(Emmision::new(0.0, 0.0, 1.0));
    // transform_plane.set_scale(1.0, 1.0, 1.0);
    // transform_plane.set_rotation(0.0, 90.0, 0.0);
    // transform_plane.set_position(5.0, 3.0, -5.0);
    // world.add(Box::new(Plane::new(transform_plane, difflight_2)));

    // let difflight_3 = Material::Emmition(Emmision::new(0.0, 1.0, 0.0));
    // transform_plane.set_scale(1.0, 1.0, 1.0);
    // transform_plane.set_rotation(0.0, 0.0, 0.0);
    // transform_plane.set_position(0.0, 3.0, -8.0);
    // world.add(Box::new(Plane::new(transform_plane, difflight_3)));

    // let difflight_4 = Material::Emmition(Emmision::new(1.0, 1.0, 1.0));
    // transform_plane.set_scale(1.0, 1.0, 1.0);
    // transform_plane.set_rotation(90.0, 0.0, 0.0);
    // transform_plane.set_position(0.0, 6.0, 0.0);
    // world.add(Box::new(Plane::new(transform_plane, difflight_4)));

    return world;
}

fn main()
{
    //Image
    Settings::load();

    //World
    //let world = random_sphere_demo();
    let world = lighting_demo();
    
    // Camera
    let mut camera = Camera::new();
    camera.transform.set_scale(1.0, 1.0, 1.0);
    camera.transform.set_rotation(0.0, 0.0, 0.0);
    camera.transform.set_position(0.0, 1.5, 6.0);
    //camera.set_camera_from_settings();

    //Render
    camera.trace(world);
}