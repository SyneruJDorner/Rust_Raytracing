use crate::{Matrix, Point};
use uuid::Uuid;
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DebugAABB
{
    pub uuid: Uuid,
    pub aabb_points: [Point; 8],
    pub projection_matrix: Matrix,
}

impl DebugAABB
{
    pub fn new() -> Self
    {
        return DebugAABB
        {
            uuid: Uuid::new_v4(),
            aabb_points: [Point::new(0.0, 0.0, 0.0); 8],
            projection_matrix: Matrix::identity(),
        };
    }
}

lazy_static!
{
    //pub static mut debug_QUEUE: &'static mut Vec<DebugAABB> = &mut Vec::new();
    pub static ref DEBUG_QUEUE: RwLock<Vec<DebugAABB>> = RwLock::new(Vec::new());
}

#[derive(Default)]
#[allow(non_snake_case)]
pub struct DebugQueue
{

}

impl DebugQueue
{
    //Insert new element to the end of debug_QUEUE array []
    pub fn add_to_debug_queue(uuid: Uuid, aabb_points: [Point; 8], projection_matrix: Matrix)
    {
        let mut debug_queue = DEBUG_QUEUE.write().unwrap();

        //check if uuid already exists in the queue
        for aabb_info in debug_queue.iter()
        {
            if aabb_info.uuid == uuid
            {
                return;
            }
        }

        debug_queue.push(DebugAABB
        {
            uuid: uuid,
            aabb_points: aabb_points,
            projection_matrix: projection_matrix,
        });
    }

    pub fn debug_queue_len() -> usize
    {
        let read = DEBUG_QUEUE.read().unwrap();
        return read.len();
    }
}

// #[derive(Default, Clone, Debug, PartialEq)]
// #[allow(non_snake_case)]
// pub struct PostProcessingQueue
// {
//     pub queue: Vec<AABBInfo>
// }

// impl PostProcessingQueue
// {    
//     pub fn new() -> Self
//     {
//         return PostProcessingQueue
//         {
//             queue: Vec::new()
//         };
//     }

//     #[allow(dead_code)]
//     pub fn add_to_queue(&mut self, uuid: Uuid, vertices: [Point; 8])
//     {
//         //Check if the uuid already exists in the queue, if it does, return
//         if self.queue.iter().any(|x| x.uuid == uuid)
//         {
//             return;
//         }

//         let mut aabb_info = AABBInfo::new();
//         aabb_info.uuid = uuid;
//         aabb_info.aabb_points = vertices;
//         self.queue.push(aabb_info);
//     }

//     #[allow(dead_code)]
//     pub fn get_queue(&self) -> &Vec<AABBInfo>
//     {
//         return &self.queue;
//     }
// }