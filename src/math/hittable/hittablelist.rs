/*
#ifndef HITTABLE_LIST_H
#define HITTABLE_LIST_H

#include "hittable.h"

#include <memory>
#include <vector>

using std::shared_ptr;
using std::make_shared;

class hittable_list : public hittable {
    public:
        hittable_list() {}
        hittable_list(shared_ptr<hittable> object) { add(object); }

        void clear() { objects.clear(); }
        void add(shared_ptr<hittable> object) { objects.push_back(object); }

        virtual bool hit(
            const ray& r, double t_min, double t_max, hit_record& rec) const override;

    public:
        std::vector<shared_ptr<hittable>> objects;
};

bool hittable_list::hit(const ray& r, double t_min, double t_max, hit_record& rec) const {
    hit_record temp_rec;
    bool hit_anything = false;
    auto closest_so_far = t_max;

    for (const auto& object : objects) {
        if (object->hit(r, t_min, closest_so_far, temp_rec)) {
            hit_anything = true;
            closest_so_far = temp_rec.t;
            rec = temp_rec;
        }
    }

    return hit_anything;
}

#endif
*/

use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::sphere::Sphere;

pub struct HittableList
{
    pub objects: Vec<Sphere>
    //Create a list variable here to hold hittable objects
    //pub list: Vec<Box<dyn Hittable>>
    //Create an array variable here to hold hittable objects
    //pub array: [dyn Hittable]
}

impl HittableList
{
    pub fn new() -> HittableList
    {
        HittableList { objects: Vec::new() }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self)
    {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Sphere)
    {
        self.objects.push(object);
    }

    pub fn hit_closest_object(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>
    {
        let mut closest_so_far: f32 = t_max;
        let mut current_object: Option<HitRecord> = None;


        //loop over objects and check which one hit is the closest
        for object in &self.objects
        {
            let hit = object.hit(ray, t_min, closest_so_far);
            if hit != None
            {
                let current_hit_distance = hit.unwrap().distance;
                if current_hit_distance < closest_so_far
                {
                    closest_so_far = current_hit_distance;
                    current_object = hit;
                }
            }
        }

        return current_object;
    }
}

// impl Hittable for HittableList
// {
//     fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>
//     {
//         let temp_rec = HitRecord::new();
//         let mut hit_anything: bool = false;
//         let mut closest_so_far: f32 = t_max;

//         for object in &self.objects
//         {
//             let hit = object.hit(ray, t_min, closest_so_far);
//             if hit == 
//             {
//                 hit_anything = true;
//                 closest_so_far = hit_record.t;
//                 hit_record = &temp_rec;
//             }
//         }

//         return hit_anything;
//     }
// }