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
        self.local_matrix = Matrix4x4::identity();
        self.world_matrix = Matrix4x4::identity();
        self.world_matrix = self.world_matrix.scale(self.scale);
        self.world_matrix = self.world_matrix.rotate_x(self.rotation.x);
        self.world_matrix = self.world_matrix.rotate_y(self.rotation.y);
        self.world_matrix = self.world_matrix.rotate_z(self.rotation.z);
        self.world_matrix = self.world_matrix.translate(self.position);
        self.local_matrix = self.world_matrix.inverse();
        return self;
    }
}
