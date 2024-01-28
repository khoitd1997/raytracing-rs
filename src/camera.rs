use super::{
    color::{write_color, Color},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    rtweekend::{degrees_to_radians, random_double, INFINITY},
    vec3::{Point3, Vec3},
};
use std::{sync::Arc, sync::Mutex, thread};
use num_cpus;

#[derive(Clone)]
pub struct Camera {
    pub image_width: i32,
    pub aspect_ratio: f64,
    pub samples_per_pixel: i32,
    pub max_depth: i32,

    /// Vertical view angle (field of view)
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,

    pub defocus_angle: f64,
    pub focus_dist: f64,

    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            image_width: 100,
            aspect_ratio: 1.0,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, -1.0),
            lookat: Point3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,

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
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
        }
    }
}

fn thread_func(
    camera: Camera,
    world: &(dyn Hittable + Sync),
    j_start: i32,
    j_end: i32,
    output_str_arc: Arc<Mutex<Vec<(i32, String)>>>,
) {
    let mut out_str = String::new();
    eprintln!("Thread for {j_start} starting");
    for j in j_start..j_end {
        for i in 0..camera.image_width {
            let mut pixel_color = Color {
                ..Default::default()
            };
            for _sample in 0..camera.samples_per_pixel {
                let r = camera.get_ray(i, j);
                pixel_color = pixel_color + camera.ray_color(&r, camera.max_depth, world);
            }

            write_color(&mut out_str, &pixel_color, camera.samples_per_pixel);
        }
        eprintln!("{j} is done");
    }

    output_str_arc.lock().unwrap().push((j_start, out_str));
    eprintln!("Thread for {j_start} finished");
    return ();
}

impl Camera {
    pub fn render(&mut self, world: Box<dyn Hittable + Sync + Send>) {
        self.initialize();

        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        let num_threads = (num_cpus::get() * 2 / 3) as i32;
        eprintln!("Spawning {num_threads} threads");
        let j_per_thread = self.image_height / num_threads;
        let output_str_arc = Arc::new(Mutex::new(Vec::new()));
        let world_arc = Arc::new(world);
        let thread_list: Vec<thread::JoinHandle<()>> = (0..num_threads)
            .into_iter()
            .map(|tid| {
                let j_start = j_per_thread * tid;
                let j_end = if tid == num_threads - 1 {
                    self.image_height
                } else {
                    j_per_thread * (tid + 1)
                };
                let my_camera = self.clone();
                let my_world = world_arc.clone();
                let my_output_str_arc = output_str_arc.clone();

                return thread::spawn(move || {
                    thread_func(
                        my_camera,
                        &(*(*my_world)),
                        j_start,
                        j_end,
                        my_output_str_arc,
                    );
                });
            })
            .collect();

        for t in thread_list {
            t.join().unwrap();
        }
        let mut res = output_str_arc.lock().unwrap().clone();
        res.sort_by(|lhs, rhs| lhs.0.partial_cmp(&(rhs.0)).unwrap());

        for (_, s) in res {
            println!("{s}");
        }

        //for j in 0..self.image_height {
        //eprint!("\rScanlines remaining: {} ", self.image_height - j);
        //eprint!("\rDone.              \n");
        //}
    }

    fn initialize(&mut self) {
        self.image_height = ((self.image_width as f64) / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.center = self.lookfrom;

        // Camera
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width =
            viewport_height * ((self.image_width as f64) / (self.image_height as f64));

        self.w = Vec3::unit_vector(&(self.lookfrom - self.lookat));
        self.u = Vec3::unit_vector(&(Vec3::cross(&self.vup, &(self.w))));
        self.v = Vec3::cross(&(self.w), &(self.u));

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = self.u * viewport_width;
        let viewport_v = -(self.v) * viewport_height;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            self.center - (self.w * self.focus_dist) - (viewport_u / 2.0) - (viewport_v / 2.0);
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        let defocus_radius = self.focus_dist * (degrees_to_radians(self.defocus_angle / 2.0)).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord {
            ..Default::default()
        };

        if depth <= 0 {
            return Color {
                ..Default::default()
            };
        }

        if world.hit(r, Interval::new_val(0.001, INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return self.ray_color(&scattered, depth - 1, world) * attenuation;
            }

            return Color::default();
        }

        let unit_direction = Vec3::unit_vector(&(r.direction()));
        let a = (unit_direction.y() + 1.0) * 0.5;
        return Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a;
    }

    /// Get a randomly-sampled camera ray for the pixel at location i,j, originating from
    /// the camera defocus disk.
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center = self.pixel00_loc
            + (self.pixel_delta_u * (i as f64))
            + (self.pixel_delta_v * (j as f64));
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        return Ray::new(&ray_origin, &ray_direction);
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random_double();
        let py = -0.5 + random_double();
        (self.pixel_delta_u * px) + (self.pixel_delta_v * py)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        return self.center + self.defocus_disk_u * p.x() + self.defocus_disk_v * p.y();
    }
}
