+++
title = "Aside: Debugging"
date = 2019-11-19
+++

Just a quick note on getting the vscode interactive debugger playing nicely with
the in-docker development.

You can probably work out for yourself that you need to install the LLDB
debugging extension in the container (the Dockerfile already had a line to
install LLDB itself), so in `devcontainer.json`

```json
"extensions": [
  ...
  "vadimcn.vscode-lldb",
  ...
]
```

But, if you now spin up the container and try to start debugging, you'll be
greeted with a lovely error like:

> process launch failed: 'A' packet returned an error: 8

This is fantastically unhelpful, but the error is actually coming from LLDB
itself. The error "8" appears to be `ENOEXEC` (defined in
[`errno-base.h`](https://github.com/torvalds/linux/blob/6f52b16c5b29b89d92c0e7236f4655dc8491ad70/include/uapi/asm-generic/errno-base.h)) or "exec format error". This is a clue, because
it's an error from the `exec` family of system calls, which are used to spawn
new processes.

Long story short, debuggers like `gdb` and `lldb` use the syscall `ptrace` to
attach to their target processes. By default, docker containers are stripped of
many priviliges (which is part of the isolation which is the whole reason we
use docker). One of these stripped privileges, though, is exactly `ptrace`.

Following the docker [documentation](https://docs.docker.com/engine/reference/run/),
we can give ourselves this privilege back with the docker argument
`--cap-add=SYS_PTRACE`. Hooray!

But...

Even this wasn't enough for me. Looking more closely at the Docker documentation
you find this

> The default seccomp profile will adjust to the selected capabilities, in order
> to allow use of facilities allowed by the capabilities, so you should not have
> to adjust this, since Docker 1.12. In Docker 1.10 and 1.11 this did not happen
> and it may be necessary to use a custom seccomp profile or use --security-opt
> seccomp=unconfined when adding capabilities.

Right, I'm using an up-to-date Docker, so this shouldn't apply? Anyway, trying
out `--security-opt seccomp=unconfined` and rebuilding the container...
debugging works! devcontainer.json:

```json
"runArgs": [
  ...
  "--cap-add=SYS_PTRACE",
  "--security-opt",
  "seccomp=unconfined"
]
```

I'm not entirely sure why this didn't seem to work as documented, but I've spent
too much time on this already and have lost all motivation to learn more right
now.
