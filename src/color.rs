use super::vec3::Vec3;
use super::interval::Interval;

pub type Color = Vec3;

pub fn write_color(out: &mut String, pixel_color: &Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / (samples_per_pixel as f64);
    r *= scale;
    g *= scale;
    b *= scale;

    let intensity = Interval::new_val(0.0, 0.999);

    out.push_str(
        format!(
            "{} {} {}\n",
            (256.0 * intensity.clamp(r)) as i32,
            (256.0 * intensity.clamp(g)) as i32,
            (256.0 * intensity.clamp(b)) as i32,
        )
        .as_str(),
    )
}

