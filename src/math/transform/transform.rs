use crate::vec3::Vec3;
use crate::matrix4x4::Matrix4x4;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Transform
{
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    pub world_matrix: Matrix4x4,
    pub local_matrix: Matrix4x4,
    pub right: Vec3,
    pub forward: Vec3,
    pub up: Vec3
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
            local_matrix: Matrix4x4::identity(),
            right: Vec3::new(1.0, 0.0, 0.0),
            forward: Vec3::new(0.0, 1.0, 0.0),
            up: Vec3::new(0.0, 0.0, 1.0)
        }
    }

    //Set the position of the transform
    #[allow(dead_code)]
    pub fn set_position(&mut self, position: Vec3)
    {
        self.position = position;
        self.update_transform();
    }

    #[allow(dead_code)]
    pub fn set_rotation(&mut self, rotation: Vec3)
    {
        self.rotation = rotation;
        self.update_transform();
    }

    #[allow(dead_code)]
    pub fn set_scale(&mut self, scale: Vec3)
    {
        self.scale = scale;
        self.update_transform();
    }

    fn update_transform(&mut self) -> Transform
    {
        self.world_matrix = Matrix4x4::identity();

        let scale = self.world_matrix.scale(self.scale);
        let rotation_x = self.world_matrix.rotate_x(self.rotation.x);
        let rotation_y = self.world_matrix.rotate_y(self.rotation.y);
        let rotation_z = self.world_matrix.rotate_z(self.rotation.z);
        let position = self.world_matrix.translate(self.position);
        
        let srt_matrix = scale * (rotation_x * rotation_y * rotation_z) * position;
        self.world_matrix = srt_matrix;

        self.right = Vec3::new(self.world_matrix.matrix[0][0], self.world_matrix.matrix[1][0], self.world_matrix.matrix[2][0]);
        self.forward = Vec3::new(self.world_matrix.matrix[0][1], self.world_matrix.matrix[1][1], self.world_matrix.matrix[2][1]);
        self.up = Vec3::new(self.world_matrix.matrix[0][2], self.world_matrix.matrix[1][2], self.world_matrix.matrix[2][2]);

        return *self;
    }
}
