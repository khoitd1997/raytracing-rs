use raytracing_rs::{
    camera::Camera,
    color::Color,
    hittable_list::HittableList,
    material::{Lambertian, Metal},
    sphere::Sphere,
    vec3::Point3,
};
use std::{cell::RefCell, rc::Rc};

fn main() {
    let mut world = HittableList::new();

    let material_ground = Rc::new(RefCell::new(Lambertian::new(&(Color::new(0.8, 0.8, 0.0)))));
    let material_center = Rc::new(RefCell::new(Lambertian::new(&(Color::new(0.7, 0.3, 0.3)))));
    let material_left = Rc::new(RefCell::new(Metal::new(&(Color::new(0.8, 0.8, 0.8)))));
    let material_right = Rc::new(RefCell::new(Metal::new(&(Color::new(0.8, 0.6, 0.2)))));

    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ))));


    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world);
}
