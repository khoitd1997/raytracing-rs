use raytracing_rs::{
    hittable::HitRecord,
    hittable_list::HittableList,
    ray::Ray,
    rtweekend::INFINITY,
    sphere::Sphere,
    vec3::{Point3, Vec3},
};
use std::{cell::RefCell, rc::Rc};

type Color = Vec3;

fn write_color(out: &mut String, pixel_color: &Color) {
    out.push_str(
        format!(
            "{} {} {}\n",
            (255.999 * pixel_color.x()) as i32,
            (255.999 * pixel_color.y()) as i32,
            (255.999 * pixel_color.z()) as i32,
        )
        .as_str(),
    )
}

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction = r.direction().unit_vector();
    let a = (unit_direction.y() + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;

    // Calculate the image height, and ensure that it's at least 1.
    let image_width = 400;
    let image_height_temp = ((image_width as f64) / aspect_ratio) as i32;
    let image_height = if image_height_temp < 1 {
        1
    } else {
        image_height_temp
    };

    let mut world = HittableList::new();
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
    ))));

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ((image_width as f64) / (image_height as f64));
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    // Calculate the location of the upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    println!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * (i as f64)) + (pixel_delta_v * (j as f64));
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(&camera_center, &ray_direction);
            let pixel_color = ray_color(&r, &world);
            let mut out_str = String::new();
            write_color(&mut out_str, &pixel_color);

            println!("{}", out_str);
        }
        eprint!("\rDone.              \n");
    }
}
