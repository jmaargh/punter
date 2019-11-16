---
title: "Getting a skeleton up and running"
date: 2019-11-16T22:43:20Z
---

I was going to make my firt post "what is a path tracer anyway?", but part of
the point of this project is to learn exactly that by doing. So instead, let's
start somewhere a bit more practical.

# Getting a skeleton up and running

To develop this project, I'm personally using
[vscode](https://code.visualstudio.com/)'s in-docker development features. This
essentially means spinning up a custom docker image for all development so that
anything installed is nicely isolated from your host system. I'm also developing
in Rust, so we'll need a rust toolchain too.

## Docker setup

The configuration files for the docker development environment are in
[.devcontainer/](https://github.com/jmaargh/punter/tree/master/.devcontainer).
To make use of these like I do simply

- [install docker](https://docs.docker.com/) for your system
- [install vscode and its container tools extension](https://code.visualstudio.com/docs/remote/containers)
- and open a vscode instance in that container in the normal way.

If you'd rather not use vscode, then you can still build from the same
[Dockerfile](https://github.com/jmaargh/punter/tree/master/.devcontainer/Dockerfile)
and do in-docker development however you'd prefer.


Note that the development image also has (for my convenience) tools for building
this blog, you won't necessarily need those.

## Rust setup

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
