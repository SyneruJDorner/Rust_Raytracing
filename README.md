# Rust_Raytracing
A research project into the world of rust and raytracing.
<br/>

**Idea behind project:**<br/>
This project is to test my knowledge in rust, by building on top of my knowledge of Raytracing, as well as sources to improve my knowledge withing Rust and raytracing in general.
<br/>

**Currently working features:**<br/>
- Basic raytracing of spheres.
- Moving the camera with Vector3.
- Materials: Albedo, Metal, Glass.
- Anti-Aliasing.
- Create new windows with content in it.

**Need to fix:**<br/>
- Rotations and storing the transforms as these seem to be reset after assignment, something I don't understand in Rust I assume.

**Need to Implement:**<br/>
- Lights
- Shadows from light sources.
- Different shapes.
- Loading textures.
- Loading Meshes/Models.
- Look into glTF { https://www.khronos.org/gltf/ }.
- Moving, Rotation and Scaling shapes and models.
- Threading and Multi-Core Processing.
<br/>

**How to run:**<br/>
- Have Rust installed.
- Navigate to the cloned directory.
- Run it using the command "cargo run .\main.rs > image.ppm".
- This will generate an image in .ppm format, you may will need a Mac to open it or use a website to open and view it.
<br/>

**PPM Image Viewer:**<br/>
- https://www.cs.rhodes.edu/welshc/COMP141_F16/ppmReader.html
<br/>

**Reference Links:**<br/>
- https://raytracing.github.io/books/RayTracingInOneWeekend.html
- https://github.com/ryankaplan/vec3/blob/master/src/lib.rs
- http://www.cplusplus.com/forum/general/243435/
- https://github.com/justindd1994/rtv1/tree/master/Constructs/Vector/Vector3/operators
- https://github.com/dps/rust-raytracer
- https://www.andre-gaschler.com/rotationconverter/ { Used to make sure Matrix calcaulations are correct }
- https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-generating-camera-rays/generating-camera-rays { Matching up rays direction with camera's direction }
<br/>
<br/>

![Prefab](https://github.com/justindd1994/Rust_Raytracing/blob/master/images/ray-tracing-demo.png)<br/>
![Prefab](https://github.com/justindd1994/Rust_Raytracing/blob/master/images/ray-tracing-demo-2.png)<br/>
![Prefab](https://github.com/justindd1994/Rust_Raytracing/blob/master/images/ray-tracing-demo-3.png)<br/>
