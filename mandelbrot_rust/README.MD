# Mandelbrot Set Visualizer

A simple program for visualizing the Mandelbrot set with zoom functionality and the ability to set a center for detailed fractal exploration.

## How to Run

1. Build and run the project:
   ```bash
   cargo run --release
   ```

2. The generated image will be saved as `mandelbrot_zoom.png`.

## Parameters

- **Zoom**: Controls the zoom level, allowing you to focus on smaller details in the Mandelbrot set. The higher the zoom value, the more detailed the view.

  Example:
  ```rust
  let zoom = 10.0;  // Zoom in 10x
  ```

- **Center (center_re, center_im)**: The real and imaginary coordinates around which you want to zoom in and explore.

  Example:
  ```rust
  let center_re = -0.75;  // Real part of the center
  let center_im = 0.1;    // Imaginary part of the center
  ```

- **Resolution (width, height)**: The number of points (pixels) calculated horizontally and vertically. A higher resolution gives a more detailed image.

  Example:
  ```rust
  let width = 1000;
  let height = 1000;
  ```