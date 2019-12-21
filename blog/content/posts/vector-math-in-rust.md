+++
title = "Vector math in rust"
date = 2019-12-08
+++

Path tracing is basically just a lot of 3d geometry, with some RGB colour
calculations. To a mathematician, this is almost all just linear algebra, so
we better find a good linear algebra library to manage the fiddly details
efficiently for us.

Somewhat frustratingly, there are
[SO SO MANY](http://arewegameyet.com/categories/math/) different math libraries 
in Rust. Many of these are:

- actively maintained,
- support the linear algebra we need,
- supported by (some of the) community.

Arguably, this is a good thing for Rust---there's
[even a crate just to translate between them](https://github.com/pistondevelopers/vecmath)---but
it does make just chosing something for a fun project a bit of a chore.

Long story short: I decided to go with [vek](https://crates.io/crates/vek). The
big motivator for this was the SIMD support.

For those who aren't aware,
[SIMD](https://en.wikipedia.org/wiki/SIMD) stands for
"Single Instruction, Multiple Data" and refers to special processor instructions
that can work on a handful of inputs simultaneously (yes, I know I'm
massively simplifying). This can provide ridiculous speedups for a lot of
computer graphics code for the CPU, because these sorts of operations are a
very nice match for the sort of linear algebra involved. (Of course, if you were
really looking for crazy speedups, you'd move to the GPU, but that's for another
day).

It seems SIMD in Rust has been going through a bit of a hard time lately. Or at
least a not-entirely-standardised time. A cursory look seems to suggest this
means that some of the "bigger" Rust math libraries (e.g.,
[nalgebra]() and [cgmath]()) have decided to delay their support. vek, on the
other hand, proudly displays SIMD support.

Also, a quick look through the [wiki](https://github.com/yoanlcq/vek/wiki)
displays design goals (and anti-goals) which I identify with. Basically a nice
tradeoff between performance, a simple API, and restricting yourself to the
important things.

## Code?

The only [code changes](https://github.com/jmaargh/punter/commit/551183c2f4f8e935114b1bac5e31026a85bccce6)
to speak of here are a line in `Cargo.toml`

```toml
vek = {version = "0.9.10", features = ["repr_simd"]}
```

and a change to the development `Dockerfile`

```Dockerfile
    RUST_VERSION=nightly-2019-11-17
```

This is because SIMD is currently only supported on the Rust nightly toolchain.
Note that you need the `"repr_simd"` feature to use vek SIMD.

## Typedefs

Moving forward, I'm going to manage most third-party types using typedefs or
Newtype patterns (the latter to enforce stricter type safety). For example

```Rust
type Num = f32;
type Vec3 = vek::vec::repr_simd::vec3::Vec3<Num>;
type Ray = vek::geom::repr_simd::Ray<Num>;

struct Radians(pub Num);
```
