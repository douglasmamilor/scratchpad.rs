# Scratchpad.rs

A handrolled CPU-side 2D rendering engine in Rust. This is an ongoing hobbyist project.

## Currently Implemented

### Rendering Primitives

- Lines (Bresenham pixel-perfect and Xiaolin Wu anti-aliased)
- Rectangles (fill and outline)
- Circles and ellipses (fill and outline)
- Triangles (fill, outline, and anti-aliased with 2x2 supersampling)
- Polygons (convex and concave with even-odd and non-zero winding fill rules)
- Flood fill

### Stroke Rendering

- Thick line rendering with configurable width
- Line caps (butt, square, round)
- Line joins (bevel, miter with limit, round)
- Dashed and dotted line patterns
- Polyline stroking

### Color & Shading

- Barycentric coordinate interpolation for per-vertex colors
- Alpha blending with gamma-correct compositing
- Scissor/clipping rectangles

### Texturing

- Textured triangle rasterization with UV mapping
- Nearest-neighbor and bilinear sampling modes
- Texture atlases with automatic packing
- Sprite rendering and batching

### Image Processing

- Convolution filters (box blur, Gaussian blur, sharpen, edge detect, emboss)
- Sobel edge detection
- Color adjustments (brightness, contrast, saturation, gamma)
- Grayscale, invert, threshold, posterize, sepia

### Text Rendering

- Bitmap font loading (BMFont format)
- Text layout with alignment (left, center, right)
- Word wrapping with whitespace preservation
- Anchored text positioning
- Text tinting and drop shadows

### Animation

- Keyframe animation system with per-keyframe easing
- 30+ easing functions (quad, cubic, sine, expo, elastic, bounce, back, etc.)
- Lerp trait for interpolating custom types

### Particle Systems

- Emitter-based particle system
- Configurable emitter shapes (point, line, circle, rectangle)
- Per-particle color, size, rotation, and lifetime
- Preset effects (fire, smoke, rain, sparkles)

### Transforms & Camera

- 3x3 affine transformation matrices
- Transform stack (push/pop state machine)
- Matrix decomposition and recomposition
- Camera with world-to-screen coordinate conversion
- Viewport management with aspect ratio handling

### Math Utilities

- Vec2, Vec3, Mat2, Mat3
- Point and rect types with coordinate space markers
- Simple PRNG (xorshift32)
- Barycentric coordinates and point-in-triangle tests

### Infrastructure

- Framebuffer with ARGB8888 pixel format
- Depth buffer with configurable depth testing
- BMP image loading
- SDL2 window integration
