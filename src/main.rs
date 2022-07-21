#[path = "./lib.rs"] mod lib;
use crate::lib::*;

#[allow(dead_code)]
fn lighting_demo() -> HittableList
{
    let mut world: HittableList = HittableList::new();

    //PLANET/GROUND
    let mut ground_sphere = Sphere::new();
    ground_sphere.transform.set_scale(1000.0, 1000.0, 1000.0);
    ground_sphere.transform.set_position(0.0, -1000.0, 0.0);
    let ground_material = Material::Lambertian(Lambertian::new(0.5, 0.5, 0.5));
    ground_sphere.material = ground_material;
    world.add(Box::new(ground_sphere));


    //CENTERED SPHERE
    let mut glass_sphere = Sphere::new();
    glass_sphere.transform.set_scale(1.0, 1.0, 1.0);
    glass_sphere.transform.set_rotation(0.0, 45.0, 0.0);
    glass_sphere.transform.set_position(0.0, 1.0, 3.0);
    let glass_mat = Material::Glass(Glass::new(1.5));
    glass_sphere.material = glass_mat;
    world.add(Box::new(glass_sphere));


    //FORWARD SIDE SPHERES
    let mut albedo_sphere = Sphere::new();
    albedo_sphere.transform.set_scale(1.0, 1.0, 1.0);
    albedo_sphere.transform.set_rotation(0.0, 0.0, 0.0);
    albedo_sphere.transform.set_position(-2.5, 1.0, 2.5);
    let albedo_mat = Material::Lambertian(Lambertian::new(0.4, 0.2, 0.1));
    albedo_sphere.material = albedo_mat;
    world.add(Box::new(albedo_sphere));

    let mut metal_sphere = Sphere::new();
    metal_sphere.transform.set_scale(1.0, 1.0, 1.0);
    metal_sphere.transform.set_rotation(0.0, 90.0, 0.0);
    metal_sphere.transform.set_position(2.5, 1.0, 2.5);
    let metal_mat = Material::Metal(Metal::new(0.7, 0.6, 0.5, 0.0));
    metal_sphere.material = metal_mat;
    world.add(Box::new(metal_sphere));


    //BACK SPHERE
    let mut albedo_sphere = Sphere::new();
    albedo_sphere.transform.set_scale(1.0, 1.0, 1.0);
    albedo_sphere.transform.set_rotation(0.0, 0.0, 0.0);
    albedo_sphere.transform.set_position(0.0, 1.0, -6.0);
    let albedo_mat = Material::Lambertian(Lambertian::new(0.0, 1.0, 0.0));
    albedo_sphere.material = albedo_mat;
    world.add(Box::new(albedo_sphere));


    //FRONT TRIANGLES
    let mut albedo_triangle = Triangle::new();
    albedo_triangle.transform.set_scale(1.0, 1.0, 1.0);
    albedo_triangle.transform.set_rotation(90.0, 0.0, -45.0);
    albedo_triangle.transform.set_position(1.0, 1.0, 4.0);
    let albedo_mat = Material::Lambertian(Lambertian::from_color(Color::pink()));
    albedo_triangle.material = albedo_mat;
    world.add(Box::new(albedo_triangle));

    let mut albedo_triangle = Plane::new();
    albedo_triangle.transform.set_scale(1.0, 1.0, 1.0);
    albedo_triangle.transform.set_rotation(90.0, 0.0, -45.0);
    albedo_triangle.transform.set_position(-1.0, 1.0, 4.0);
    let albedo_mat = Material::Lambertian(Lambertian::from_color(Color::teal()));
    albedo_triangle.material = albedo_mat;
    world.add(Box::new(albedo_triangle));


    // //FRONT CUBE
    // let mut albedo_cube = Cube::new();
    // albedo_cube.transform.set_scale(1.0, 1.0, 1.0);
    // albedo_cube.transform.set_rotation(90.0, 0.0, 0.0);
    // albedo_cube.transform.set_position(0.0, 1.0, 0.0);
    // let albedo_mat = Material::Lambertian(Lambertian::from_color(Color::pink()));
    // albedo_cube.material = albedo_mat;
    // world.add(Box::new(albedo_cube));


    //PLANES
    let mut emmision_plane = Plane::new();
    emmision_plane.transform.set_scale(4.0, 2.0, 2.0);
    emmision_plane.transform.set_rotation(90.0, 90.0, 0.0);
    emmision_plane.transform.set_position(4.0, 2.0, 4.0);
    let difflight_1 = Material::Emmition(Emmision::from_color(Color::red()));
    emmision_plane.material = difflight_1;
    world.add(Box::new(emmision_plane));

    let mut emmision_plane = Plane::new();
    emmision_plane.transform.set_scale(4.0, 2.0, 2.0);
    emmision_plane.transform.set_rotation(90.0, 90.0, 0.0);
    emmision_plane.transform.set_position(-4.0, 2.0, 4.0);
    let difflight_2 = Material::Emmition(Emmision::from_color(Color::blue()));
    emmision_plane.material = difflight_2;
    world.add(Box::new(emmision_plane));

    return world;
}

fn main()
{
    use std::time::Instant;
    let now = Instant::now();

    // Image
    Settings::load();

    // World
    let world = lighting_demo();
    
    // Camera
    let mut camera = Camera::new();
    camera.transform.set_scale(1.0, 1.0, 1.0);
    camera.transform.set_rotation(0.0, 0.0, 0.0);
    camera.transform.set_position(0.0, 1.5, 7.5);

    // Render
    camera.trace(world);

    let elapsed = now.elapsed();
    println!("Render time: {:.2?}", elapsed);
}