---
title: "Getting a skeleton up and running"
date: 2019-11-16T22:43:20Z
---

I was going to make my firt post "what is a path tracer anyway?", but part of
the point of this project is to learn exactly that by doing. So instead, let's
start somewhere a bit more practical.

## Setup

To develop this project, I'm personally using
[vscode](https://code.visualstudio.com/)'s in-docker development features. This
essentially means spinning up a custom docker image for all development so that
anything installed is nicely isolated from your host system. I'm also developing
in Rust, so we'll need a rust toolchain too.

### Docker setup

The configuration files for the docker development environment are in
[.devcontainer/](https://github.com/jmaargh/punter/tree/master/.devcontainer).
To make use of these like I do simply

- [install docker](https://docs.docker.com/) for your system
- [install vscode and its container tools extension](https://code.visualstudio.com/docs/remote/containers)
- and open a vscode instance in that container in the normal way.

If you'd rather not use vscode, then you can still build from the same
[Dockerfile](https://github.com/jmaargh/punter/tree/master/.devcontainer/Dockerfile)
and do in-docker development however you'd prefer.

There are a couple of things to note about setting up the Dockerfile:

- [rustup](https://rustup.rs/) is only configured to install Rust tools for the
  current user, so in the Dockerfile we have to do something a bit more
  esoteric. I decided to follow the
  [official Rust Dockerfile](https://github.com/rust-lang/docker-rust/blob/8d0f25416858e2c1f59511a15c2bd0445b402caa/1.39.0/buster/Dockerfile)
  in installing to `/usr/local`.
- In
  [devcontainer.json](https://github.com/jmaargh/punter/tree/master/.devcontainer/devcontainer.json)
  there is some important config even if you're not using vscode. In particular:
  docker flags to run as the non-root user `dev` (`"-u", "dev"`), to expose the
  Hugo development server on port 1313 (`"appPort": 1313`), and to mount your
  user's ssh config for pushing/pulling to git
  (`"-v", "${env:HOME}/.ssh/:/home/dev/.ssh:ro"`).

Note that the development image also has (for my convenience) tools for building
this blog, you won't necessarily need those.

### Rust setup

At this stage, I'm planning on using an up-to-date Rust stable compiler for
Ubuntu 18.04 64bit. Targeting Rust 2018. If you're not using the development
docker container, the [Rust docs](https://www.rust-lang.org/tools/install) can
tell you how to set things up much better than I ever could. The tooling's
great, trust it.

If you'd rather not use docker at all, I'm afraid I'm going to leave you on
your own for setting up a development environment and toolchain, I'm sure you
can work it out from the [Rust docs](https://www.rust-lang.org/tools/install).
Throughout, I'm going to write any instructions assuming an Ubuntu 18.04 (or
similar) environment. It shouldn't be too hard to follow along for mac, Windows,
or another distribution, but, again, you're on your own.

## A skeleton application

I'm starting with a simple binary crate in the
[`punter/`](https://github.com/jmaargh/punter/tree/master/punter/)
subdirectory. This was set up with a simple

```bash
$ cargo new --bin punter
```

Plus, because the crate isn't at the root of the git repository, I've put the
following `Cargo.toml` at the root

```toml
[workspace]
members = ["punter"]
```

this gives me flexibility to create as many sibling creates as I want as we move
forward.

The skeleton in the
[`punter/src/main.rs`](https://github.com/jmaargh/punter/tree/master/punter/src/main.rs)
module makes use of the [image crate](https://crates.io/crates/image). A path
tracer works by running an algorithm for each pixel in the image being rendered,
an algorithm that calculates the light incident on that pixel in the virtual
scene. The skeleton I've set up simply mocks out that process in the most
basic way (where each pixel just returns black).

```rust
fn main() {
  let mut img = make_image(1024, 768);
  render_image(&mut img);
  save_png(&img, std::path::Path::new("image.png"));
}
```

This neatly outlines what we're doing: creating an image buffer, rendering to
it, and saving it out to disk.

```rust
type Pixel = image::Rgb<u8>;
type Image = image::ImageBuffer<Pixel, Vec<u8>>;

fn make_image(width: u32, height: u32) -> Image {
  image::ImageBuffer::from_raw(width, height, vec![0u8; (width * height * 3) as usize]).unwrap()
}

fn save_png(img: &Image, path: &std::path::Path) {
  img.save_with_format(path, image::PNG).unwrap();
}
```

A cursory glance at the image crate code seems to suggest that using an RGB
buffer of bytes will be nice and efficient, so that's what I'm going for. The
typedefs may help refactoring later.

```rust
fn render_image(img: &mut Image) {
  for (column, row, pixel) in img.enumerate_pixels_mut() {
    render_pixel(pixel, row, column);
  }
}

fn render_pixel(pixel: &mut Pixel, row: u32, column: u32) {
  pixel.0 = [128u8; 3];
}
```

This seems to be the idiomatic way to write pixel-by-pixel using the image
crate. The `enumerate_pixels_mut()` method is nice, and the `image::Rgb<u8>`
struct is just a
[newtype pattern](https://doc.rust-lang.org/1.0.0/style/features/types/newtype.html)
over a `[u8; 3]` array.

Running this with `$ cargo run`  will give a nice 1024×768 pixel 50% grey image
called `image.png`.

And that's it! All we need to do now is write the real `render_pixel` function
and we're done! Easy!
