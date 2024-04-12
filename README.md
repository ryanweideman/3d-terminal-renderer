# 3d-terminal-renderer

[![build](https://github.com/ryanweideman/rust-terminal-rasterizer/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/ryanweideman/rust-terminal-rasterizer/actions/workflows/build.yml)

A simple 3d graphics engine built in Rust that renders directly to the terminal.

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

<img src="/media/demo.gif" width="100%"/>

## Scene Customization
Scene objects and lighting can be configured in ```demo.json```, like as follows:
```
{
    "objects": [
        {
            "type": "SpinningObject",
            "model": "cube.json",
            "origin": [1.3, -0.3, 0.75],
            "rotation_axis": [0.5, 0.0, 1.0],
            "rotation_angle": 0.0,
            "angular_velocity": -1.5,
            "scale": 0.9
        }
    ],
    "lights": [
        {
            "type": "PointLight",
            "origin": [0.0, 0.7, 0.0],
            "intensity": 5.0,
            "linear_attenuation": 0.05,
            "quadratic_attenuation": 0.4,
            "color": [255, 255, 255]
        },
        {
            "type": "AmbientLight",
            "intensity": 0.38,
            "color": [255, 255, 255]
        }
    ]
}
```

## Custom Models
Custom models can be included in the ```models``` directory. Model geometry is specified in a simple JSON based format. Here's an example model of a square composed of two red triangles:
```
{
    "geometry": [
      {
        "vertices": [
          [-0.5, -0.5, 0.0],
          [0.5, -0.5, 0.0],
          [0.5, 0.5, 0.0]
        ],
        "color": [255, 0, 0]
      },
      {
        "vertices": [
          [-0.5, -0.5, 0.0],
          [0.5, 0.5, 0.0],
          [-0.5, 0.5, 0.0]
        ],
        "color": [255, 0, 0]
      }
    ]
}
```
