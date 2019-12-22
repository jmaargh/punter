use crate::camera;
use crate::scene::Scene;
use crate::Pixel;

pub struct Shot {
  width_px: u32,
  height_px: u32,
  camera: Box<dyn camera::Camera>,
  scene: Scene,
}

impl Shot {
  pub fn new(width_px: u32, height_px: u32, camera: Box<dyn camera::Camera>, scene: Scene) -> Self {
    Self {
      width_px,
      height_px,
      camera,
      scene,
    }
  }

  pub fn render_pixel(&self, column: u32, row: u32) -> Pixel {
    let (column, row) = camera::normalise_coordinates(column, row, self.width_px, self.height_px);
    let ray = self.camera.make_ray(column, row);

    // Just for testing, we visualise x, y, and z as R, G, and B respectively,
    // with (0, 0, 0) scaled to be 50% grey
    image::Rgb::<u8>([
      ((ray.direction.x + 1f32) * 0.5f32 * 255f32) as u8,
      ((ray.direction.y + 1f32) * 0.5f32 * 255f32) as u8,
      ((ray.direction.z + 1f32) * 0.5f32 * 255f32) as u8,
    ])
  }
}
