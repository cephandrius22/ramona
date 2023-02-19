This is my attempt at following along with https://raytracing.github.io/books/RayTracingInOneWeekend.html#overview.

I've written a couple raytracers in C for school and even worked through parts of this book before, but I wanted
to work through it again while writing my own code in Rust. I think there is a Rust equivalent to "Raytracing in a Weekend"
which I plan to look at, but I wanted to reason through it myself at first.

I think the end goal here is to have a raytracer that can run in real-time or produce
an image file. It seems likely that some type of hardware acceleration will be
necessary. I hope to add multiple renderers/methods of acceleration (Metal, Vulkan, OpenCL, CUDA). That is a lofty goal but I think I'd learn a lot about those technologies + it would
force me to have some sane code architecture to support multiple renderers which is something
I want to focus on.

### TODO
- [X] Finish Raytracing in One Weekend (this seems like a good stopping point to begin the other tasks)
- [ ] Implement some other renderer (Metal may be a good first choice)
