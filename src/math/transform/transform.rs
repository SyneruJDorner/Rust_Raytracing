use crate::vec3::Vec3;
use crate::matrix4x4::Matrix4x4;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Transform
{
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    pub world_matrix: Matrix4x4,
    pub local_matrix: Matrix4x4
}

impl Transform
{
    #[allow(dead_code)]
    pub fn new() -> Transform
    {
        Transform
        {
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            world_matrix: Matrix4x4::identity(),
            local_matrix: Matrix4x4::identity()
        }
    }

    //Set the position of the transform
    #[allow(dead_code)]
    pub fn set_position(&mut self, position: Vec3)
    {
        self.position = position;
        self.update_local_matrix();
    }

    #[allow(dead_code)]
    pub fn set_rotation(&mut self, rotation: Vec3)
    {
        self.rotation = rotation;
        self.update_local_matrix();
    }

    #[allow(dead_code)]
    pub fn set_scale(&mut self, scale: Vec3)
    {
        self.scale = scale;
        self.update_local_matrix();
    }

    fn update_local_matrix(mut self) -> Transform
    {
        self.world_matrix = Matrix4x4::identity();

        let scale = self.world_matrix.scale(self.scale);
        let rotation_x = self.world_matrix.rotate_x(self.rotation.x);
        let rotation_y = self.world_matrix.rotate_y(self.rotation.y);
        let rotation_z = self.world_matrix.rotate_z(self.rotation.z);
        let position = self.world_matrix.translate(self.position);
        
        let srt_matrix = scale * (rotation_z * rotation_y * rotation_x) * position;
        self.world_matrix = srt_matrix;
        self.local_matrix = srt_matrix.inverse();
        return self;
    }
}
