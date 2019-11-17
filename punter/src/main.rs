type Pixel = image::Rgb<u8>;
type Image = image::ImageBuffer<Pixel, Vec<u8>>;

fn main() {
  let mut img = make_image(1024, 768);
  render_image(&mut img);
  save_png(&img, std::path::Path::new("image.png"));
}

fn make_image(width: u32, height: u32) -> Image {
  image::ImageBuffer::from_raw(width, height, vec![0u8; (width * height * 3) as usize]).unwrap()
}

fn save_png(img: &Image, path: &std::path::Path) {
  img.save_with_format(path, image::PNG).unwrap();
}

fn render_image(img: &mut Image) {
  for (column, row, pixel) in img.enumerate_pixels_mut() {
    render_pixel(pixel, row, column);
  }
}

fn render_pixel(pixel: &mut Pixel, row: u32, column: u32) {
  pixel.0 = [128u8; 3];
}
