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
<br/>

![Prefab](https://raw.githubusercontent.com/justindd1994/Rust_Raytracing/master/ray-tracing-demo.png)<br/>
![Prefab](https://raw.githubusercontent.com/justindd1994/Rust_Raytracing/master/ray-tracing-demo-2.png)<br/>
![Prefab](https://raw.githubusercontent.com/justindd1994/Rust_Raytracing/master/ray-tracing-demo-3.png)<br/>
