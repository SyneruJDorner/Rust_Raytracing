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


//FUTURE RESEARCH
//Look up rendering into a window
//https://docs.rs/winit/0.10.1
*/

//Settings
#[path = "settings/settings.rs"] mod settings;
pub use settings::Settings as Settings;

//Maths
#[path = "math/constant/constant.rs"] mod constant;
#[path = "math/tuple/tuple.rs"] mod tuple;
#[path = "math/tuple/point/point.rs"] mod point;
#[path = "math/tuple/vector2/vector2.rs"] mod vector2;
#[path = "math/tuple/vector3/vector3.rs"] mod vector3;
#[path = "math/tuple/color/color.rs"] mod color;
#[path = "math/matrix/matrix.rs"] mod matrix;
#[path = "math/transform/transform.rs"] mod transform;
#[path = "math/utils/utils.rs"] mod utils;
pub use constant::*;
pub use tuple::Tuple as Tuple;
pub use point::Point as Point;
pub use vector2::Vector2 as Vector2;
pub use vector3::Vector3 as Vector3;
pub use color::Color as Color;
pub use matrix::Matrix as Matrix;
pub use transform::Transform as Transform;
pub use utils::*;

//Raytracing
#[path = "raytracing/ray/ray.rs"] mod ray;
#[path = "raytracing/camera/camera.rs"] mod camera;
#[path = "raytracing/material/material.rs"] mod material;
#[path = "raytracing/hittable/hittable.rs"] mod hittable;
#[path = "raytracing/hittable/hittablelist.rs"] mod hittablelist;
#[path = "raytracing/shapes/sphere.rs"] mod sphere;
#[path = "raytracing/shapes/plane.rs"] mod plane;
#[path = "raytracing/shapes/triangle.rs"] mod triangle;
#[path = "raytracing/shapes/cube.rs"] mod cube;
#[path = "raytracing/aabb/aabb.rs"] mod aabb;
#[path = "raytracing/draw/draw.rs"] mod draw;
#[path = "raytracing/debug_queue/debug_queue.rs"] mod debug_queue;
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
pub use hittable::HitInfo as HitInfo;
pub use hittable::HitObject as HitObject;
pub use hittablelist::HittableList as HittableList;
pub use sphere::Sphere as Sphere;
pub use plane::Plane as Plane;
pub use triangle::Triangle as Triangle;
pub use cube::Cube as Cube;
pub use aabb::AABB as AABB;

pub use debug_queue::DebugQueue as DebugQueue;
pub use draw::*;

#[path = "cmd_operations/cmd_operations.rs"] mod cmd;
pub use cmd::*;

extern crate pbr;

#[cfg(test)]
mod tests
{
    use super::*;

    // #[test]
    // fn test_mul_tuple_matrix()
    // {
    //     let result_mat = Matrix::identity();
    //     let result = Tuple::new(1.0, 2.0, 3.0, 4.0) * result_mat;
    //     println!("{:?}", result);
    //     let expected = Matrix::set([[5.0,  0.0,  0.0,  0.0],
    //                                 [0.0,  5.0,  0.0,  0.0],
    //                                 [0.0,  0.0,  5.0,  0.0],
    //                                 [0.0,  0.0,  0.0,  5.0]]);
    //     assert!(result.approximate(expected));

    //     let result_mat = Matrix::identity();
    //     let result = result_mat * 5.0;
    //     println!("{:?}", result);
    //     let expected = Matrix::set([[5.0,  0.0,  0.0,  0.0],
    //                                 [0.0,  5.0,  0.0,  0.0],
    //                                 [0.0,  0.0,  5.0,  0.0],
    //                                 [0.0,  0.0,  0.0,  5.0]]);
    //     assert!(result.approximate(expected));
    // }

    //#region Matrix Multiplication 
    #[test]
    fn matrix_mul()
    {
        //=====================================================================//
        let result_mat = Matrix::identity();
        let result = 5.0 * result_mat;
        let expected = Matrix::set([[5.0,  0.0,  0.0,  0.0],
                                    [0.0,  5.0,  0.0,  0.0],
                                    [0.0,  0.0,  5.0,  0.0],
                                    [0.0,  0.0,  0.0,  5.0]]);
        assert!(result.approximate(expected));
        //=====================================================================//
        let result_mat = Matrix::identity();
        let result = result_mat * 5.0;
        let expected = Matrix::set([[5.0,  0.0,  0.0,  0.0],
                                    [0.0,  5.0,  0.0,  0.0],
                                    [0.0,  0.0,  5.0,  0.0],
                                    [0.0,  0.0,  0.0,  5.0]]);
        assert!(result.approximate(expected));
        //=====================================================================//
        let result_mat_1 = Matrix::set([[5.0, 5.0, 3.0, 1.0],
                                        [2.0, 4.0, 7.0, 8.0],
                                        [9.0, 2.0, 6.0, 4.0],
                                        [2.0, 3.0, 1.0, 5.0]]);

        let result_mat_2 = Matrix::set([[1.0, 2.0, 3.0, 4.0],
                                        [1.0, 2.0, 3.0, 4.0],
                                        [1.0, 2.0, 3.0, 4.0],
                                        [1.0, 2.0, 3.0, 4.0]]);                     
        let result = result_mat_1 * result_mat_2;
        let expected = Matrix::set([[14.0,  28.0,  42.0,  56.0],
                                    [21.0,  42.0,  63.0,  84.0],
                                    [21.0,  42.0,  63.0,  84.0],
                                    [11.0,  22.0,  33.0,  44.0]]);
        assert!(result.approximate(expected));
        //=====================================================================//
        let result_mat_1 = Matrix::set([[20.0,  5.5,  6.75, 34.0],
                                        [2.4,   5.2,  1.1,  55.0],
                                        [63.10, 1.1,  5.1,  17.3],
                                        [15.2,  14.5, 8.0,  8.0]]);

        let result_mat_2 = Matrix::set([[8.0,  8.0,  14.5, 15.2],
                                        [17.3, 5.1,  1.1,  63.10],
                                        [55.0, 1.1,  5.2,  2.4],
                                        [34.0, 6.75, 5.5,  20.0]]);                     
        let result = result_mat_1 * result_mat_2;
        let expected = Matrix::set([[1782.4,   424.975,  518.15,   1347.25],
                                    [2039.66,  418.18,   348.74,   1467.24],
                                    [1392.53,  632.795,  1037.83,  1386.77],
                                    [1084.45,  258.35,   321.95,   1325.19]]);
        assert!(result.approximate(expected));
        //=====================================================================//
        let result_mat = Matrix::set([[10.0,  11.0,   12.0,  13.0],
                                      [5.0,   6.0,    7.0,    8.0],
                                      [63.0,  64.0,   65.0,  66.0],
                                      [101.0, 102.0,  55.0,  10.0]]);        
        let result = result_mat * 5.0;
        let expected = Matrix::set([[50.0,  55.0,   60.0,  65.0],
                                    [25.0,  30.0,   35.0,  40.0],
                                    [315.0, 320.0,  325.0, 330.0],
                                    [505.0, 510.0,  275.0, 50.0]]);
        assert!(result.approximate(expected));
    }

    #[test]
    fn matrix_div()
    {
        //=====================================================================//
        let result_mat = Matrix::identity();
        let result = 5.0 / result_mat;
        let expected = Matrix::set([[0.2,  0.0,  0.0,  0.0],
                                    [0.0,  0.2,  0.0,  0.0],
                                    [0.0,  0.0,  0.2,  0.0],
                                    [0.0,  0.0,  0.0,  0.2]]);
        assert!(result.approximate(expected));
        //=====================================================================//
        let result_mat = Matrix::identity();
        let result = result_mat / 5.0;
        let expected = Matrix::set([[0.2,  0.0,  0.0,  0.0],
                                    [0.0,  0.2,  0.0,  0.0],
                                    [0.0,  0.0,  0.2,  0.0],
                                    [0.0,  0.0,  0.0,  0.2]]);
        assert!(result.approximate(expected));
        //=====================================================================//
        let result_mat = Matrix::set([[5.0,  3.0,  1.0,  7.0],
                                      [9.0,  8.0,  7.0,  3.0],
                                      [2.0,  1.0,  5.0,  6.0],
                                      [7.0,  8.0,  6.0,  1.0]]);
        let result = result_mat / 7.0;
        let expected = Matrix::set([[5.0/7.0, 3.0/7.0,  1.0/7.0, 1.0],
                                    [9.0/7.0, 8.0/7.0,  1.0,     3.0/7.0],
                                    [2.0/7.0, 1.0/7.0,  5.0/7.0, 6.0/7.0],
                                    [1.0,     8.0/7.0,  6.0/7.0, 1.0/7.0]]);
        assert!(result.approximate(expected));
        //=====================================================================//
        let result_mat = Matrix::set([[5.0,  3.0,  1.0,  7.0],
                                      [9.0,  8.0,  7.0,  3.0],
                                      [2.0,  1.0,  5.0,  6.0],
                                      [7.0,  8.0,  6.0,  1.0]]);
        let result = 7.0 / result_mat;
        let expected = Matrix::set([[5.0/7.0, 3.0/7.0,  1.0/7.0, 1.0],
                                    [9.0/7.0, 8.0/7.0,  1.0,     3.0/7.0],
                                    [2.0/7.0, 1.0/7.0,  5.0/7.0, 6.0/7.0],
                                    [1.0,     8.0/7.0,  6.0/7.0, 1.0/7.0]]);
        assert!(result.approximate(expected));
        //=====================================================================//
        let result_mat = Matrix::set([[12.5, 13.4,  25.9,  26.4],
                                      [9.5,  78.0,  99.9,  101.1],
                                      [5.5,  65.5,  15.2,  74.9],
                                      [55.1, 55.2,  55.3,  55.4]]);
        let result = result_mat / 2.5;
        let expected = Matrix::set([[5.0,   5.36,  10.36, 10.56],
                                    [3.8,   31.2,  39.96, 40.44],
                                    [2.2,   26.2,  6.08,  29.96],
                                    [22.04, 22.08, 22.12, 22.16]]);
        println!("{:?}", result);
        println!("{:?}", expected);
        assert!(result.approximate(expected));
        //=====================================================================//
    }
}