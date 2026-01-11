# Rust-Minecraft Client (cross-platform)
This proof-of-concept demonstrates using Rust with `jni-simple` to load a native shared libray into the Minecraft JVM, and modify in-game values at runtime.
This technique is commonly used in C++ based Minecraft "injection clients", which provide a DLL or EXE injected into the game to add cheats.

My proof of concept demonstrates the possibilities of this, within Rust. Why is this good? I don't know - everything is wrapped in `unsafe {}` anyway.

![Demo GIF](https://s12.gifyu.com/images/bhaO7.gif)

---

### How to use (Linux)
- Ensure Minecraft is running
- Simply run `cargo build && sudo ./inject.sh`

### How to use (Windows)
- Run `cargo build` to build the DLL
- Using Process Hacker 2 (or another DLL-injector), inject the DLL into the Minecraft process
