# 3d-terminal-renderer

[![build](https://github.com/ryanweideman/rust-terminal-rasterizer/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/ryanweideman/rust-terminal-rasterizer/actions/workflows/build.yml)

A simple 3d graphics engine that renders directly to the terminal

## Features
- CPU-only rasterization-based 3D graphics pipeline
- Lighting
- True Color 24bit mode or Ansi 8bit color mode, with optional dithering to reduce color banding
- Controllable camera (WASD & arrow keys)
  - OS key sensitivity may need to be adjusted for best experience
- Adaptive resolution on termimal window resize
  - Decrease the terminal's font size to increase the resolution!
- Model and Scene JSON loading systems
- Cross-Platform support

<img src="/media/demo.gif" width="80%"/>
