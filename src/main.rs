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

#[path = "settings/settings.rs"] mod settings;
#[path = "math/constants/constants.rs"] mod constants;
#[path = "math/matrix/matrix4x4.rs"] mod matrix4x4;
#[path = "math/vec3/vec3.rs"] mod vec3;
#[path = "math/aabb/aabb.rs"] mod aabb;
#[path = "math/transform/transform.rs"] mod transform;
#[path = "math/utils/utils.rs"] mod utils;
#[path = "raytracing/color/color.rs"] mod color;
#[path = "raytracing/ray/ray.rs"] mod ray;
#[path = "raytracing/camera/camera.rs"] mod camera;
#[path = "raytracing/hittable/hittablelist.rs"] mod hittablelist;
#[path = "raytracing/hittable/hittable.rs"] mod hittable;
#[path = "raytracing/shapes/sphere.rs"] mod sphere;
#[path = "raytracing/shapes/plane.rs"] mod plane;
#[path = "raytracing/material/material.rs"] mod material;
#[path = "raytracing/material/submaterials/lamertian.rs"] mod lamberian;
#[path = "raytracing/material/submaterials/metal.rs"] mod metal;
#[path = "raytracing/material/submaterials/glass.rs"] mod glass;

use settings::Settings as Settings;
use vec3::Vec3 as Vec3;
use camera::Camera as Camera;
use hittablelist::HittableList as HittableList;
use sphere::Sphere as Sphere;
use plane::Plane as Plane;
use material::Material as Material;
use material::submaterials::lamertian::Lambertian as Lambertian;
#[allow(unused_imports)]
use material::submaterials::metal::Metal as Metal;
#[allow(unused_imports)]
use material::submaterials::glass::Glass as Glass;
#[allow(unused_imports)]
use material::submaterials::emmision::Emmision as Emmision;

/*
fn random_sphere_demo() -> HittableList
{
    let mut world: HittableList = HittableList::new();
    
    let ground_material = Material::Lambertian(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(Vec3::new(0.0,      -1000.0,     0.0),  1000.0,  ground_material));

    for a in -11..11
    {
        for b in -11..11
        {
            let choose_mat = utils::random_float(0.0, 1.0);
            let center = Vec3::new(a as f32 + 0.9 * utils::random_float(0.0, 1.0), 0.2, b as f32 + 0.9 * utils::random_float(0.0, 1.0));
            
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9
            {
                if choose_mat < 0.8
                {
                    // diffuse
                    let albedo = Vec3::new(utils::random_float(0.0, 1.0) * utils::random_float(0.0, 1.0), utils::random_float(0.0, 1.0) * utils::random_float(0.0, 1.0), utils::random_float(0.0, 1.0) * utils::random_float(0.0, 1.0));
                    world.add(Sphere::new(center, 0.2, Material::Lambertian(Lambertian::new(albedo))));
                }
                else if choose_mat < 0.95
                {
                    // metal
                    let albedo = Vec3::new(0.5 * (1.0 + utils::random_float(0.0, 1.0)), 0.5 * (1.0 + utils::random_float(0.0, 1.0)), 0.5 * (1.0 + utils::random_float(0.0, 1.0)));
                    let fuzz = utils::random_float(0.0, 1.0) * 0.5;
                    world.add(Sphere::new(center, 0.2, Material::Metal(Metal::new(albedo, fuzz))));
                }
                else
                {
                    // glass
                    world.add(Sphere::new(center, 0.2, Material::Glass(Glass::new(1.5))));
                    //world.add(Sphere::new(center, 0.15, Material::Glass(Glass::new(1.5))));
                }
            }
        }
    }

    let material1 = Material::Glass(Glass::new(1.5));
    world.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Material::Lambertian(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));
    //world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 0.95, material2));

    let material3 = Material::Metal(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    return world;
}
*/

fn lighting_demo() -> HittableList
{
    let mut world: HittableList = HittableList::new();
   
    let ground_material = Material::Lambertian(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0,      -1000.0,     0.0),  1000.0,  ground_material)));

    let glass_mat = Material::Glass(Glass::new(1.5));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, glass_mat)));

    let albedo_mat = Material::Lambertian(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, albedo_mat)));

    let metal_mat = Material::Metal(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, metal_mat)));

    let difflight = Material::Emmition(Emmision::new(Vec3::new(1.0, 1.0, 1.0)));
    let mut transform_plane = transform::Transform::new();
    transform_plane.set_position(vec3::Vec3::new(-2.0, 2.0, -5.0));
    //transform_plane.set_rotation(vec3::Vec3::new(45.0, 45.0, 0.0));
    transform_plane.set_scale(vec3::Vec3::new(1.0, 1.0, 1.0));
    world.add(Box::new(Plane::new(transform_plane, 8.0, 8.0, difflight)));
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
    camera.set_position(Vec3::new(5.0, 1.5, 3.0));
    camera.set_rotation(Vec3::new(15.0, -45.0, 0.0));
    camera.set_scale(Vec3::new(1.0, 1.0, 1.0));
    camera.set_camera_from_settings();

    //Render
    camera.trace(world);
}
