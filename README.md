# rust-terminal-rasterizer

# homebrew-8bit-cpu
[![build](https://github.com/ryanweideman/rust-terminal-rasterizer/actions/workflows/ci.yml/badge.svg)](https://github.com/ryanweideman/rust-terminal-rasterizer/actions/workflows/ci.yml)

A simple 3d graphics engine that renders directly to the terminal

## Features
- Full software-only rasterization-based 3D graphics pipeline
- Lighting
- True Color 24bit mode or Ansi 8bit color mode, with optional dithering to reduce color banding
- Adaptive resolution on termimal window resize
  - Decrease the terminal's font size to increase the resolution!
- Keyboard controllable camera (WASD & arrow keys)
  - OS key sensitivity may need to be adjusted for best experience
- Model and Scene JSON loading systems
- Cross-Platform support

<img src="/media/demo.gif" width="80%"/>
