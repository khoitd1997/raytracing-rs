use raytracing_rs::{camera::Camera, hittable_list::HittableList, sphere::Sphere, vec3::Point3};
use std::{cell::RefCell, rc::Rc};

fn main() {
    let mut world = HittableList::new();
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
    ))));

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;

    cam.render(&world);
}
