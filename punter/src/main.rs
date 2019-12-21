const PI: Num = std::f32::consts::PI;
const PI_ON_TWO: Num = PI * 0.5f32;

type Pixel = image::Rgb<u8>;
type Image = image::ImageBuffer<Pixel, Vec<u8>>;

type Num = f32;
type Vec3 = vek::vec::repr_simd::vec3::Vec3<Num>;
type Ray = vek::geom::repr_simd::Ray<Num>;

#[derive(Clone, Copy)]
struct Radians(pub Num);

struct PinholeCamera {
  centre: Vec3,
  direction: Vec3,
  aspect_ratio: Num,
  field_of_view: Radians,
  roll: Radians,
  screen_x_unit_vector: Vec3,
  screen_y_unit_vector: Vec3,
  inverse_effective_distance: Num,
}

trait Camera {
  fn make_ray(&self, normalised_x: Num, normalised_y: Num) -> Ray;

  fn normalise_coordinates(column: u32, row: u32, width: u32, height: u32) -> (Num, Num) {
    let half_width = (width as Num) * 0.5f32;
    let half_height = (height as Num) * 0.5f32;

    (
      ((column as Num) - half_width) / half_width,
      (half_height - (row as Num)) / half_width,
    )
  }
}

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
  let width = img.width();
  let height = img.height();

  for (column, row, pixel) in img.enumerate_pixels_mut() {
    render_pixel(pixel, column, row, width, height);
  }
}

fn render_pixel(pixel: &mut Pixel, column: u32, row: u32, width: u32, height: u32) {
  // Just for testing, we create a camera each time
  // camera is centred on the origin and points in the negative z (forwards)
  // direction.
  let camera = PinholeCamera::from_pixel_dimensions(
    Vec3::zero(),
    Vec3::new(0f32, 0f32, -1f32),
    width,
    height,
    Radians(PI_ON_TWO),
    Radians(0f32),
  );

  let (column, row) = PinholeCamera::normalise_coordinates(column, row, width, height);
  let ray = camera.make_ray(column, row);

  // Just for testing, we visualise x, y, and z as R, G, and B respectively,
  // with (0, 0, 0) scaled to be 50% grey
  pixel.0 = [
    ((ray.direction.x + 1f32) * 0.5f32 * 255f32) as u8,
    ((ray.direction.y + 1f32) * 0.5f32 * 255f32) as u8,
    ((ray.direction.z + 1f32) * 0.5f32 * 255f32) as u8,
  ];
}

impl PinholeCamera {
  pub fn new(
    centre: Vec3,
    direction: Vec3,
    aspect_ratio: Num,
    field_of_view: Radians,
    roll: Radians,
  ) -> Self {
    let inverse_effective_distance = calculate_inverse_effective_distance(field_of_view);
    let (screen_x_unit_vector, screen_y_unit_vector) =
      calculate_screen_coordinate_system(&direction, roll);

    Self {
      centre,
      direction,
      aspect_ratio,
      field_of_view,
      roll,
      screen_x_unit_vector,
      screen_y_unit_vector,
      inverse_effective_distance,
    }
  }

  pub fn from_pixel_dimensions(
    centre: Vec3,
    direction: Vec3,
    width_px: u32,
    height_px: u32,
    field_of_view: Radians,
    roll: Radians,
  ) -> Self {
    Self::new(
      centre,
      direction,
      (height_px as Num) / (width_px as Num),
      field_of_view,
      roll,
    )
  }
}

fn calculate_inverse_effective_distance(field_of_view: Radians) -> Num {
  (field_of_view.0 * 0.5f32).tan()
}

fn calculate_screen_coordinate_system(direction: &Vec3, roll: Radians) -> (Vec3, Vec3) {
  // y cross d
  let unrotated_x_direction = Vec3::new(-direction.z, 0f32, direction.x);
  // (d cross y) cross d
  let unrotated_y_direction = Vec3::new(
    -direction.x * direction.y,
    direction.x * direction.x + direction.z * direction.z,
    -direction.z * direction.y,
  );

  let cos_roll = roll.0.cos();
  let sin_roll = roll.0.sin();

  let x_direction = {
    let mut rotated = unrotated_x_direction * cos_roll - unrotated_y_direction * sin_roll;
    rotated.normalize();
    rotated
  };
  let y_direction = {
    let mut rotated = unrotated_y_direction * cos_roll + unrotated_x_direction * sin_roll;
    rotated.normalize();
    rotated
  };

  (x_direction, y_direction)
}

impl Camera for PinholeCamera {
  fn make_ray(&self, normalised_x: Num, normalised_y: Num) -> Ray {
    let mut ray_direction = self.direction
      + normalised_x * self.inverse_effective_distance * self.screen_x_unit_vector
      + normalised_y * self.inverse_effective_distance * self.screen_y_unit_vector;
    ray_direction.normalize();

    Ray {
      direction: ray_direction,
      origin: self.centre,
    }
  }
}
