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

#[path = "settings/settings.rs"] mod settings;
#[path = "math/constant/constant.rs"] mod constant;
#[path = "math/tuple/tuple.rs"] mod tuple;
#[path = "math/tuple/point/point.rs"] mod point;
#[path = "math/tuple/vector3/vector3.rs"] mod vector3;
#[path = "math/tuple/color/color.rs"] mod color;
#[path = "math/matrix/matrix.rs"] mod matrix;
#[path = "math/transform/transform.rs"] mod transform;
#[path = "math/utils/utils.rs"] mod utils;
#[path = "raytracing/ray/ray.rs"] mod ray;
#[path = "raytracing/camera/camera.rs"] mod camera;
#[path = "raytracing/material/material.rs"] mod material;
#[path = "raytracing/hittable/hittable.rs"] mod hittable;
#[path = "raytracing/hittable/hittablelist.rs"] mod hittablelist;
#[path = "raytracing/shapes/sphere.rs"] mod sphere;
#[path = "raytracing/shapes/plane.rs"] mod plane;

pub use settings::Settings as Settings;
pub use constant::*;
pub use tuple::Tuple as Tuple;
pub use point::Point as Point;
pub use vector3::Vector3 as Vector3;
pub use color::Color as Color;
pub use matrix::Matrix as Matrix;
pub use transform::Transform as Transform;
pub use utils::*;
pub use ray::Ray as Ray;
pub use camera::Camera as Camera;
pub use material::Material as Material;
pub use material::Scatterable as Scatterable;
pub use material::Emmitable as Emmitable;
pub use material::Normalable as Normalable;
pub use material::submaterials::lambertian::Lambertian as Lambertian;
pub use material::submaterials::metal::Metal as Metal;
pub use material::submaterials::glass::Glass as Glass;
pub use material::submaterials::emmision::Emmision as Emmision;
pub use hittable::Hittable as Hittable;
pub use hittable::HitRecord as HitRecord;
pub use hittablelist::HittableList as HittableList;
pub use sphere::Sphere as Sphere;
pub use plane::Plane as Plane;