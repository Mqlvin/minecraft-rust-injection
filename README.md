# Rust-Minecraft Client
This proof-of-concept demonstrates using Rust with `jni-simple` to load a native shared libray into the Minecraft JVM, and modify in-game values at runtime.
This technique is commonly used in C++ based Minecraft "injection clients", which provide a DLL or EXE injected into the game to add cheats.

My proof of concept demonstrates the possibilities of this, within Rust. Why is this good? I don't know - everything is wrapped in `unsafe {}` anyway.

![Gif](https://s12.gifyu.com/images/bhaO7.gif)

## How to use (Linux)
This code is specific to Vanilla Minecraft 1.8.9, as it implements 1.8.9 deobfuscation mappings.

To start, launch Minecraft 1.8.9. Then simply run `cargo build && sudo ./inject.sh`.

## How to use (Windows)
This isn't available for Windows (yet?).
I am exploring options to integrate Windows and Linux support within a single codebase.
