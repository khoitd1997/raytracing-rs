use raytracing_rs::vec3::Vec3;

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

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let r = (i as f64) / ((image_width - 1) as f64);
            let g = (j as f64) / ((image_height - 1) as f64);
            let b = 0 as f64;

            let pixel_color = Color::new(r, g, b);
            let mut out_str = String::new();
            write_color(&mut out_str, &pixel_color);

            println!("{}", out_str);
        }
        eprint!("\rDone.              \n");
    }
}
