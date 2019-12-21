+++
title = "A pinhole camera"
date = 2019-12-08
+++

The first thing we need if we're going to render anything is a camera. Before
that, we're going to need to decide on an abstration of a camera to use.

## Why the pinhole?

The pinhole Camera is the simplest type of perspective camera. For those not
familiar, the idea is to imagine an opaque plane with an infintessimally small
hole (apperture) at the camera's "centre". In this idealisation, the only light
let through to from your scene must therefore pass through exacty the centre
point. With a real pinhole camera, you'd then put a screen behind (and parallel
to) the apperture and see a projection of the scene on this screen. This image
will be interverted in both axes, as you can see by tracing any light ray that
makes it through the apperture (hey, that's what we're doing, ray tracing!).

img

We normally complicate this idealisation a little bit though. For a start, our
screen isn't going to be infinite, it's going to be a finite rectangle. We
define this finite size by:

- a _field of view_ (or FOV) \\(\phi\\): the angular width of the screen with
  respect to the apperture centre
- an _aspect ratio_ \\(\rho\\): the ratio of width divided by height
- a _roll_ angle \\(\psi\\): the angle about the camera's direction by which the
  rectangular screen is rotated

Note how the distance of the screen from the centre doesn't matter so long as
you define \\(\phi\\) and \\(\rho\\).

Finally, this deal of inverting the image on a screen behind the apperture is
unnecessarily confusing when we're only making a virtual pinhole camera (rather
than a real one). Instead, I'm going to imagine a "virtual screen" _between_ the
camera centre and the screen. This virtual screen intercepts only those light
rays that would have passed through the centre and hit the real scene. However,
it has the advantage of both being in front of the centre and projecting a
non-inverted image, both of which I find more intuitive.

img

To summarise, you can define a pinhole camera with just the following

- \\(\vec{C}\\) the 3d vector for the position of the centre of the camera
  (the pinhole)
- \\(\hat{\vec{d}}\\) the 3d unit vector for the direction the camera is facing
  (I'm using the caret "\\(\hat{ }\\)" to identify unit vectors)
- \\(\phi\\) the field of view
- \\(\rho\\) the aspect ratio
- \\(\psi\\) the roll angle

Giving us the following Rust struct:

```Rust
struct PinholeCamera {
  centre: Vec3,
  direction: Vec3,
  aspect_ratio: Num,
  field_of_view: Radians,
  roll: Radians,
}
```

## What does a camera do?

The role of a camera in a path tracer is really just to translate pixels into
rays that we can sample the light from. A "ray" in this context is simply a
directed infinite line in space: a position (vector) and a direction (vector).
If we tell the Camera we want to sample a pixel at \\((x,y)\\), it gives us a
ray describing where the light has to come from to influence that pixel, and
we can do the rest without the camera.

So, in Rust

```Rust
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
```

Here I'm using `normalised_x` and `normalised_y` as be the positions from
the centre of the virtual screen where

- `normalised_x = 1.0` means the right edge
- `normalised_x = -1.0` means the left edge
- `normalised_y = aspect_ratio` means the top edge
- `normalised_y = -aspect_ratio` means the bottom edge

The utility function `normalise_coordinates` can be used to convert pixel
coordinates (with \\((0, 0)\\) in the top left) to these normalised coordinates.

Again, the camera doesn't have a given size, so it doesn't make sense to sully
the camera's primary function (`make_ray`) with width and height information,
the utility function is a nice way to isolate this implementation detail.

## Implementing the `Camera` trait for `PinholeCamera`

This is going to be a lot of maths, sorry (not sorry) if that isn't your thing.

### Chosen convetions

These are arbitrary, so it's good to record them somewhere:

- Angles (unless otherwise stated) are in radians
- Image pixel coordinates are \\((0, 0)\\) in the top left and increase right
  and down
- Image plane coordinates are \\(x\\)-right and \\(y\\)-up in most non-pixel
  units
- A right-handed 3d coordinate system
  - \\(x\\) is "right"
  - \\(y\\) is "up"
  - \\(z\\) is "backward"
  - (this has been chosen for right-handedness and so \\(x\\) and \\(y\\) match
    the usual convention in an image plane)
- Horizontal always comes before vertical (e.g. x,y; column,row), then depth
  (e.g. x,y,z)
- Field of view is the angle subtended between left and right edges of the image
  from the camera centre
- All angles around an axis are right-handed rotations

### Derivation

The ray position is origin, we just choose the camera centre \\(\vec{C}\\) to be
the ray origin. Then the ray direction, \\(\hat{\vec{v}}\\), is just the unit vector that points from
this origin to the point on the virtual screeen described by `normalised_x`
(\\(x\\)) and `normalised_y` (\\(y\\)). This requires a little derivation.

First, we need vectors that describe the plane of the virtual screen. Suppose
for simplicity that the roll \\(\psi = 0\\) and let \\(\hat{\vec{w}}^\prime\\) be the
unit vector in the positive \\(x\\) direction of the screen and
\\(\hat{\vec{h}}^\prime\\) in the positive \\(y\\) direction.

With \\(\psi = 0\\), then \\(\hat{\vec{w}}^\prime\\) must be perpendicular to
both \\(\hat{\vec{d}}\\) (so it's in the virtual screen) and also to
\\(\hat{\vec{y}} = (0, 1, 0)\\)---think about it for a moment, that's what we
mean by "horizontal". This is neat because we can just use the cross product to
get \\(\hat{\vec{w}}^\prime\\)

$$ \hat{\vec{w}}^\prime = \langle \hat{\vec{d}} \times \hat{\vec{y}} \rangle $$

(where I'm using \\(\langle\rangle\\) to denote the action of normalising a
vector). \\(\hat{\vec{h}}^\prime\\) is now easy: it's got to be both
perpendicular to \\(\hat{\vec{w}}^\prime\\) and \\(\hat{\vec{d}}\\):

$$ \hat{\vec{h}}^\prime = \langle \hat{\vec{w}}^\prime \times \hat{\vec{d}} \rangle $$

In this way, \\(\hat{\vec{w}}^\prime\\) and \\(\hat{\vec{h}}\\) describe the
image plane coordinate system (x and y in the image plane respectively), in the
world 3d coordinate system.

So what if \\(\psi\neq 0\\)? We can follow still make use of
\\(\hat{\vec{w}}^\prime\\) and \\(\hat{\vec{h}}\\) and then just rotate this
image coordinate system around \\(\hat{\vec{d}}\\) by \\(\psi\\). This rotation
gives the following pair of unit vectors describing the image plane coordinate
system even when \\(\psi = 0\\):

$$
\begin{align}
\hat{\vec{w}} &= \hat{\vec{w}}^\prime \cos\psi - \hat{\vec{h}}^\prime \sin\psi \\\\
\hat{\vec{h}} &= \hat{\vec{h}}^\prime \cos\psi + \hat{\vec{w}}^\prime \sin\psi
\end{align}
$$

(note that we don't need to renormalise, rotation is a norm-preserving
transformation).

To go from these angles to a Ray through \\((x, y)\\), we can work out the
"effective distance" \\(D\\) from \\(\vec{C}\\) to the virtual screen which
would mean that one unit along the directions \\(\pm\hat{\vec{w}}\\) will give
points that make an angle \\(\phi\\) with \\(\vec{C}\\):

img

This is just basic trigonometry

$$ D^{-1} = \tan(\phi/2). $$

So with these tools, we can just sum some vectors to get from the centre to the
virtual screen point and find the ray direction \\(\hat{\vec{v}}\\):

$$
\hat{\vec{v}} = \langle D\hat{\vec{d}} + x \hat{\vec{w}} + y \hat{\vec{h}} \rangle
$$

### Coding it up

When starting to code this, we can notice two things immediately:

- Multiplying by \\(D^{-1}\\) is probably going to be faster than inverting to
  \\(D\\).
- \\(\hat{\vec{w}}\\), \\(\hat{\vec{h}}\\), and \\(D\\) can be pre-calculated at
  the point of camera creation, leaving the hot path to only scale vectors,
  sum them, and normalise.

So, focussing on the precalculation of \\(\hat{\vec{w}}\\) and
\\(\hat{\vec{h}}\\), we can also notice that since
\\(\hat{\vec{y}} = (0, 1, 0)\\) the cross-products used to calculate them can be
simply worked out by hand. If \\(\hat{\vec{d}} = (a, b, c)\\) then

$$
\begin{align}
\hat{\vec{w}}^\prime &= \langle ( -c, 0, a ) \rangle \\\\
\hat{\vec{h}}^\prime &= \langle ( -ab, a^2 + c^2, -cb ) \rangle
\end{align}
$$

(side note: this shows that \\(\hat{\vec{h}}^\prime\\) can never point in the
negative-\\(y\\) direction, which makes sense when you consider how we derived
it).

So, we can add the following to the `PinholeCamera` struct:

```Rust
  screen_x_unit_vector: Vec3,  // w hat
  screen_y_unit_vector: Vec3,  // h hat
  inverse_effective_distance: Num,  // inverse D
```

and add a `new` function for the struct:

```Rust
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
}

fn calculate_inverse_effective_distance(field_of_view: Radians) -> Num {
  (field_of_view.0 * 0.5f32).tan()  // tan(phi/2)
}

fn calculate_screen_coordinate_system(direction: &Vec3, roll: Radians) -> (Vec3, Vec3) {
  // d cross y
  let unrotated_x_direction = Vec3::new(-direction.z, 0f32, direction.x);  // w prime (unnormalised)
  // (d cross y) cross d
  let unrotated_y_direction = Vec3::new(
    -direction.x * direction.y,
    direction.x * direction.x + direction.z * direction.z,
    -direction.z * direction.y,
  );  // h prime (unnormalised)

  let cos_roll = roll.0.cos();
  let sin_roll = roll.0.sin();

  let x_direction = {  // w
    let mut rotated = unrotated_x_direction * cos_roll - unrotated_y_direction * sin_roll;
    rotated.normalize();
    rotated
  };
  let y_direction = {  // h
    let mut rotated = unrotated_y_direction * cos_roll + unrotated_x_direction * sin_roll;
    rotated.normalize();
    rotated
  };

  (x_direction, y_direction)
}
```

With all of this precalculation, the `make_ray` function becomes a pretty simple
transliteration of our equations:

```Rust
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
```

To test this, I'm going to temporarily mess up our `render_pixel` function
from before:

```Rust
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
```

this produces the following lovely image

img

which shows that things are working. Consider, it, along the top of the image (
where y coordinates should be relatively high) there is a decent green
component throughout, while along the bottom the green is greatly desaturated.
Similarly, along the right edge for a large red component and the left for a
desaturated red. There is little blue throughout as we're pointing close to the
negative z direction.

If we flip to point along +z, we get

img

which also checks out under similar considerations.
