
![photon](https://github.com/user-attachments/assets/207d65ca-114b-4fb7-88da-b002a4943941)

# This is Photon

My little raytracer, written in Rust, was created to gain first experience with Rust. It was fun 🤖

My learning purpose is done with this project so further improvements that are obviously missing, will not be done.

# How to build

```
cargo build
```

# How to render

The output image is being written in the text format PPM to the standard output stream. The geometry, light and raytracing parameters are in the code in various files.

```
cargo run > image.ppm
```

# Output

You can easily change the geometry (as long as you want spheres or planes) and the light configuration in main.rs as well as raytracing parameters like 
- number of rays per pixel (monte carlo sampling, average value is taken)
- number of rays for each global illumination computation on a surface hit point
- resolution of output image

This is what it renders with the current configuration:

![output](https://github.com/user-attachments/assets/fd2a88be-3d56-4365-8ebf-93d1f728c6ac)
