use crate::{Matrix, Point};
use uuid::Uuid;
use lazy_static::lazy_static;
use std::sync::RwLock;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DebugAABB
{
    pub uuid: Uuid,
    pub aabb_points: [Point; 8],
    pub projection_matrix: Matrix,
}

lazy_static!
{
    pub static ref DEBUG_QUEUE: RwLock<Vec<DebugAABB>> = RwLock::new(Vec::new());
}

#[derive(Default)]
#[allow(non_snake_case)]
pub struct DebugQueue { }

impl DebugQueue
{
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn debug_queue_len() -> usize
    {
        let read = DEBUG_QUEUE.read().unwrap();
        return read.len();
    }
}
