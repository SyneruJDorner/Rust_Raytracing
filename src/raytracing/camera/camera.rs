use crate::vec3::Vec3;
//use crate::ray::Ray;
use crate::transform::Transform;
use crate::utils::degrees_to_radians;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Camera
{
    pub transform: Transform,
    //lower_left_corner: Vec3,
    //horizontal: Vec3,
    //vertical: Vec3,
    aspect_ratio: f32,
    pub fov: f32,
    world_up: Vec3
}

impl Camera
{
    #[allow(dead_code)]
    pub fn new() -> Camera
    {
        let camera = Camera 
        {
            transform: Transform::new(),
            aspect_ratio:  degrees_to_radians(90.0),
            fov: degrees_to_radians(90.0),
            world_up: Vec3::new(0.0, 1.0, 0.0)
        };
        return camera;
    }

    #[allow(dead_code)]
    pub fn set_field_of_view(mut self, vfov: f32) -> Camera
    {
        self.fov = vfov;
        return self;
    }

    #[allow(dead_code)]
    pub fn set_aspect_ratio(mut self, aspect_ratio: f32) -> Camera
    {
        self.aspect_ratio = aspect_ratio;
        return self;
    }
    
    #[allow(dead_code)]
    pub fn set_position(mut self, position: Vec3) -> Camera
    {
        self.transform.set_position(position);
        return self;
    }

    #[allow(dead_code)]
    pub fn set_rotation(mut self, rotation: Vec3) -> Camera
    {
        self.transform.set_rotation(rotation);
        return self;
    }

    #[allow(dead_code)]
    pub fn set_scale(mut self, scale: Vec3) -> Camera
    {
        self.transform.set_scale(scale);
        return self;
    }

    // #[allow(dead_code)]
    // pub fn get_ray(&self, origin: Vec3, dir: Vec3) -> Ray
    // {
    //     let ray = Ray::new(origin, dir);
    //     return ray;
    // }


    /*
    void			raytracing(t_app *app)
    {
        int				co[2];
        t_gameobject	camera;
        t_ray			ray;

        init_matrices(app);
        camera = *prepare_camera(app, &ray);
        co[1] = -1;
        while (++co[1] < HEIGHT)
        {
            co[0] = -1;
            while (++co[0] < WIDTH)
            {
                ray.direction.x = (2 * (co[0] + 0.5) * INVW - 1) *
                WIDTH * INVH * ANG;
                ray.direction.y = (1 - 2 * (co[1] + 0.5) * INVH) * ANG;
                ray.direction.z = -1;
                ray.direction = transform_rotation(camera.transform.to_local,
                        ray.direction);
                ft_normalize(&ray.direction);
                putpixelimage(app->mlx.image, WIDTH, co,
                get_colour_hit(&ray, app));
            }
        }
        mlx_put_image_to_window(app->mlx.mlx, app->mlx.window,
                app->mlx.image->img, 0, 0);
    }
    */


    /*
    void render( 
        const Options &options, 
        const std::vector<std::unique_ptr<object>> &objects, 
        const std::vector<std::unique_ptr<light>> &lights) 
    { 
        Matrix44f cameraToWorld; 
        Vec3f *framebuffer = new Vec3f[options.width * options.height]; 
        Vec3f *pix = framebuffer; 
        float scale = tan(deg2rad(options.fov * 0.5)); 
        float imageAspectRatio = options.width / (float)options.height; 
        Vec3f orig; 
        cameraToWorld.multVecMatrix(Vec3f(0), orig); 
        for (uint32_t j = 0; j < options.height; ++j) { 
            for (uint32_t i = 0; i < options.width; ++i) { 
                float x = (2 * (i + 0.5) / (float)options.width - 1) * imageAspectRatio * scale; 
                float y = (1 - 2 * (j + 0.5) / (float)options.height) * scale; 
                Vec3f dir; 
                cameraToWorld.multDirMatrix(Vec3f(x, y, -1), dir); 
                dir.normalize(); 
                *(pix++) = castRay(orig, dir, objects, lights, options, 0); 
            } 
        } 
    
        // Save result to a PPM image (keep these flags if you compile under Windows)
        std::ofstream ofs("./out.ppm", std::ios::out | std::ios::binary); 
        ofs << "P6\n" << options.width << " " << options.height << "\n255\n"; 
        for (uint32_t i = 0; i < options.height * options.width; ++i) { 
            char r = (char)(255 * clamp(0, 1, framebuffer[i].x)); 
            char g = (char)(255 * clamp(0, 1, framebuffer[i].y)); 
            char b = (char)(255 * clamp(0, 1, framebuffer[i].z)); 
            ofs << r << g << b; 
        } 
    
        ofs.close(); 
    
        delete [] framebuffer; 
    }</std::unique_ptr<light></std::unique_ptr<object> 
    */

    // #[allow(dead_code)]
    // fn update(mut self) -> Camera
    // {

        // let theta = self.fov.to_radians();
        // //let h = (theta / 2.0).tan();
        // let half_height  = (theta / 2.0).tan();
        // let half_width  = self.aspect_ratio * half_height;
        // let start_position = self.transform.position;

        // if self.look_at.is_some()
        // {
        //     let w = (self.transform.position - self.look_at.unwrap()).normalize();
        //     let u = (Vec3::cross(&self.world_up, &w)).normalize();
        //     let v = Vec3::cross(&w, &u);

        //     let start_lower_left_corner = start_position - (u * half_width) - (v * half_height) - w;
        //     let start_horizontal = u * 2.0 * half_width;
        //     let start_vertical = v * 2.0 * half_height;
        //     let world_up = Vec3::new(0.0, 1.0, 0.0);

        //     self.transform.position = start_position;
        //     self.horizontal = start_horizontal;
        //     self.vertical = start_vertical;
        //     self.lower_left_corner = start_lower_left_corner;
        //     self.world_up = world_up;
            
        //     return self;
        // }

        // let viewport_height = 2.0;
        // let viewport_width = self.aspect_ratio * viewport_height;
        // let focal_length = 1.0;

        // let start_position = Vec3::new(0.0, 0.0, 0.0);
        // let start_horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        // let start_vertical = Vec3::new(0.0, viewport_height, 0.0);
        // let start_lower_left_corner = start_position - start_horizontal / 2.0 - start_vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        // self.transform.position = start_position;
        // self.horizontal = start_horizontal;
        // self.vertical = start_vertical;
        // self.lower_left_corner = start_lower_left_corner;

    //     return self;
    // }
}