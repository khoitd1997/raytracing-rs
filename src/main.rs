use raytracing_rs::{
    camera::Camera,
    color::Color,
    hittable::Hittable,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    rtweekend::{random_double, random_double_range},
    sphere::Sphere,
    vec3::{Point3, Vec3},
};

fn main() {
    let mut world = HittableList::new();

    let ground_material = Box::new(Lambertian::new(&(Color::new(0.5, 0.5, 0.5))));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                random_double() * 0.9 + (a as f64),
                0.2,
                random_double() * 0.9 + (b as f64),
            );

            if choose_mat < 0.8 {
                let albedo = Color::new_random() * Color::new_random();
                let sphere_material = Box::new(Lambertian::new(&albedo));
                world.add(Box::new(Sphere::new(
                    center,
                    0.2,
                    sphere_material,
                )));
            } else if choose_mat < 0.95 {
                let albedo = Color::new_random_range(0.5, 1.0);
                let fuzz = random_double_range(0.0, 0.5);
                let sphere_material = Box::new(Metal::new(&albedo, fuzz));
                world.add(Box::new(Sphere::new(
                    center,
                    0.2,
                    sphere_material,
                )));
            } else {
                let sphere_material = Box::new(Dielectric::new(1.5));
                world.add(Box::new(Sphere::new(
                    center,
                    0.2,
                    sphere_material,
                )));
            }
        }
    }

    let material1 = Box::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Box::new(Lambertian::new(&(Color::new(0.4, 0.2, 0.1))));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Box::new(Metal::new(&(Color::new(0.7, 0.6, 0.5)), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )) as Box<dyn Hittable + Sync + Send>);

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(Box::new(world) as Box<dyn Hittable + Sync + Send>);
}
