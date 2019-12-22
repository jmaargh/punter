pub type Num = f32;
pub type Vec3 = vek::vec::repr_simd::vec3::Vec3<Num>;
pub type Ray = vek::geom::repr_simd::Ray<Num>;

#[derive(Clone, Copy)]
pub struct Radians(pub Num);

pub const PI: Num = std::f64::consts::PI as Num;
pub const PI_ON_TWO: Num = std::f64::consts::FRAC_PI_2 as Num;
