use super::{
    color::{write_color, Color},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    rtweekend::{random_double, INFINITY},
    vec3::{Point3, Vec3},
};

pub struct Camera {
    pub image_width: i32,
    pub aspect_ratio: f64,
    pub samples_per_pixel: i32,
    pub max_depth: i32,

    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            image_width: 100,
            aspect_ratio: 1.0,
            samples_per_pixel: 10,
            max_depth: 10,

            image_height: 0,
            center: Point3 {
                ..Default::default()
            },
            pixel00_loc: Point3 {
                ..Default::default()
            },
            pixel_delta_u: Vec3 {
                ..Default::default()
            },
            pixel_delta_v: Vec3 {
                ..Default::default()
            },
        }
    }
}

impl Camera {
    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color {
                    ..Default::default()
                };
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color = pixel_color + self.ray_color(&r, self.max_depth, world);
                }

                let mut out_str = String::new();
                write_color(&mut out_str, &pixel_color, self.samples_per_pixel);
                println!("{}", out_str);
            }
            eprint!("\rDone.              \n");
        }
    }

    fn initialize(&mut self) {
        self.image_height = ((self.image_width as f64) / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.center = Point3 {
            ..Default::default()
        };

        // Camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width =
            viewport_height * ((self.image_width as f64) / (self.image_height as f64));
        let camera_center = Point3::new(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        // Calculate the location of the upper left pixel
        let viewport_upper_left = camera_center
            - Vec3::new(0.0, 0.0, focal_length)
            - (viewport_u / 2.0)
            - (viewport_v / 2.0);
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord {
            ..Default::default()
        };

        if depth <= 0 { return Color{..Default::default()}; }

        if world.hit(r, Interval::new_val(0.001, INFINITY), &mut rec) {
            let direction = rec.normal + Vec3::random_unit_vector();
            return self.ray_color(&(Ray::new(&(rec.p), &direction)), depth - 1, world) * 0.5;
        }

        let unit_direction = Vec3::unit_vector(&(r.direction()));
        let a = (unit_direction.y() + 1.0) * 0.5;
        return Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a;
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center = self.pixel00_loc
            + (self.pixel_delta_u * (i as f64))
            + (self.pixel_delta_v * (j as f64));
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        return Ray::new(&ray_origin, &ray_direction);
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random_double();
        let py = -0.5 + random_double();
        (self.pixel_delta_u * px) + (self.pixel_delta_v * py)
    }
}
