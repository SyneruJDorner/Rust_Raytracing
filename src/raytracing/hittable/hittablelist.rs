use crate::Ray;
use crate::Hittable;
use crate::HitRecord;

pub struct HittableList
{
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList
{
    pub fn new() -> HittableList
    {
        HittableList
        {
            objects: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self)
    {
        self.objects.clear();
    }

    #[allow(dead_code)]
    pub fn add(&mut self, object: Box<dyn Hittable>)
    {
        self.objects.push(object);
    }
}

impl Hittable for HittableList
{
    fn hit(&self, world_ray: &Ray) -> Option<HitRecord>
    {
        let mut current_object: Vec<HitRecord> = Vec::new();

        //loop over objects and check which one hit is the closest
        for object in &self.objects
        {
            //If the ray intersected the AABB bounds of the object
            if object.hit_aabb(&world_ray) == true
            {
                let intersections = object.hit(world_ray);
                if intersections.is_some()
                {
                    let hit_objects = intersections.unwrap();
                    
                    if world_ray.previous_uuid == hit_objects.uuid && hit_objects.get_distance() < 0.1
                    {
                        continue;
                    }
                    
                    current_object.push(hit_objects);
                }
            }
        }

        //Determine if we hit anything, is so, sort the hit objects by distance and return
        if current_object.len() > 0
        {
            for i in 0..current_object.len()
            {
                for j in i..current_object.len()
                {
                    if current_object[i].get_distance() > current_object[j].get_distance()
                    {
                        let temp = current_object[i];
                        current_object[i] = current_object[j];
                        current_object[j] = temp;
                    }
                }
            }

            return Some(current_object[0]);
        }

        return None;
    }

    //In the future use this to determine which object was hit and return it to the caller
    fn hit_aabb(&self, _world_ray: &Ray) -> bool
    {
        return false;
    }
}
