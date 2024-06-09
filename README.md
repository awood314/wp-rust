# Will Pirkle (Audio Effects written in) Rust

Rust 🦀 implementations of audio effects from [Will Pirkle - Designing Audio Effect Plugins in C++](https://www.willpirkle.com/)

## Building the VST3/AU Plugin

**Requirements:**

- CMake (3.15)
- [Rust](https://www.rust-lang.org/learn/get-started) (2021)

```
git submodule update --init --recursive
cd plugin
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build --config Release
```
