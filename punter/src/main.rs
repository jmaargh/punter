mod camera;
mod scene;
mod shot;
mod types;

use types::*;

type Pixel = image::Rgb<u8>;
type Image = image::ImageBuffer<Pixel, Vec<u8>>;

fn main() {
  let mut img = make_image(1024, 768);

  let shot = {
    let camera = camera::PinholeCamera::from_pixel_dimensions(
      Vec3::zero(),
      Vec3::new(0f32, 0f32, -1f32),
      img.width(),
      img.height(),
      Radians(PI_ON_TWO),
      Radians(0f32),
    );
    let scene = scene::Scene {};
    shot::Shot::new(img.width(), img.height(), Box::new(camera), scene)
  };

  render_image(&mut img, shot);

  save_png(&img, std::path::Path::new("image.png"));
}

fn make_image(width: u32, height: u32) -> Image {
  image::ImageBuffer::from_raw(width, height, vec![0u8; (width * height * 3) as usize]).unwrap()
}

fn save_png(img: &Image, path: &std::path::Path) {
  img.save_with_format(path, image::PNG).unwrap();
}

fn render_image(img: &mut Image, shot: shot::Shot) {
  for (column, row, pixel) in img.enumerate_pixels_mut() {
    pixel.0 = shot.render_pixel(column, row).0;
  }
}
