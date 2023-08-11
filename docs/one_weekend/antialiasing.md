If you zoom into the rendered images so far, you might notice the harsh "stair step" nature of edges
in our rendered images.
This stair-stepping is commonly referred to as "aliasing", or "jaggies".
When a real camera takes a picture, there are usually no jaggies along edges, because the edge
pixels are a blend of some foreground and some background.
Consider that unlike our rendered images, a true image of the world is continuous.
Put another way, the world (and any true image of it) has effectively infinite resolution.
We can get the same effect by averaging a bunch of samples for each pixel.

With a single ray through the center of each pixel, we are performing what is commonly called _point
sampling_.
The problem with point sampling can be illustrated by rendering a small checkerboard far away.
If this checkerboard consists of an 8&times;8 grid of black and white tiles, but only four rays hit
it, then all four rays might intersect only white tiles, or only black, or some odd combination.
In the real world, when we perceive a checkerboard far away with our eyes, we perceive it as a gray
color, instead of sharp points of black and white.
That's because our eyes are naturally doing what we want our ray tracer to do: integrate the
(continuous function of) light falling on a particular (discrete) region of our rendered image.

Clearly we don't gain anything by just resampling the same ray through the pixel center multiple
times -- we'd just get the same result each time.
Instead, we want to sample the light falling _around_ the pixel, and then integrate those samples to
approximate the true continuous result.
So, how do we integrate the light falling around the pixel?

We'll adopt the simplest model: sampling the square region centered at the pixel that extends
halfway to each of the four neighboring pixels.
This is not the optimal approach, but it is the most straight-forward.
(See [_A Pixel is Not a Little Square_][square-pixels] for a deeper dive into this topic.)

![Pixel samples](https://raytracing.github.io/images/fig-1.08-pixel-samples.jpg)

### Some Random Number Utilities
We're going to need need a random number generator that returns real random numbers.
This function should return a canonical random number, which by convention falls in the range
$0 ≤ n < 1$.
The “less than” before the 1 is important, as we will sometimes take advantage of that.

A simple approach to this is to use the `rand()` function that can be found in `<cstdlib>`, which
returns a random integer in the range 0 and `RAND_MAX`.
Hence we can get a real random number as desired with the following code snippet, added to
`rtweekend.h`:

```c++ title="random_double() functions" hl_lines="2 13-21"
#include <cmath>
#include <cstdlib>
#include <limits>
#include <memory>
...

// Utility Functions

inline double degrees_to_radians(double degrees) {
    return degrees * pi / 180.0;
}


inline double random_double() {
    // Returns a random real in [0,1).
    return rand() / (RAND_MAX + 1.0);
}

inline double random_double(double min, double max) {
    // Returns a random real in [min,max).
    return min + (max-min)*random_double();
}
```

C++ did not traditionally have a standard random number generator, but newer versions of C++ have
addressed this issue with the `<random>` header (if imperfectly according to some experts).
If you want to use this, you can obtain a random number with the conditions we need as follows:

```c++ title="random_double(), alternate implemenation"
#include <random>

inline double random_double() {
    static std::uniform_real_distribution<double> distribution(0.0, 1.0);
    static std::mt19937 generator;
    return distribution(generator);
}
```

### Generating Pixels with Multiple Samples
For a single pixel composed of multiple samples, we'll select samples from the area surrounding the
pixel and average the resulting light (color) values together.

First we'll update the `write_color()` function to account for the number of samples we use: we need
to find the average across all of the samples that we take.
To do this, we'll add the full color from each iteration, and then finish with a single division (by
the number of samples) at the end, before writing out the color.
To ensure that the color components of the final result remain within the proper $[0,1]$ bounds,
we'll add and use a small helper function: `interval::clamp(x)`.

```c++ title="The interval::clamp() utility function" hl_lines="9-13"
class interval {
    public:
    ...

    bool surrounds(double x) const {
        return min < x && x < max;
    }


    double clamp(double x) const {
        if (x < min) return min;
        if (x > max) return max;
        return x;
    }
    ...
};
```

And here's the updated `write_color()` function that takes the sum total of all light for the pixel
and the number of samples involved:

```c++ title="The multi-sample write_color() function"
void write_color(std::ostream &out, color pixel_color, int samples_per_pixel) {
    auto r = pixel_color.x();
    auto g = pixel_color.y();
    auto b = pixel_color.z();

    // Divide the color by the number of samples.
    auto scale = 1.0 / samples_per_pixel;
    r *= scale;
    g *= scale;
    b *= scale;

    // Write the translated [0,255] value of each color component.
    static const interval intensity(0.000, 0.999);
    out << static_cast<int>(256 * intensity.clamp(r)) << ' '
        << static_cast<int>(256 * intensity.clamp(g)) << ' '
        << static_cast<int>(256 * intensity.clamp(b)) << '\n';
}
```

Now let's update the camera class to define and use a new `camera::get_ray(i,j)` function, which
will generate different samples for each pixel.
This function will use a new helper function `pixel_sample_square()` that generates a random sample
point within the unit square centered at the origin.
We then transform the random sample from this ideal square back to the particular pixel we're
currently sampling.

```c++ title="Camera with samples-per-pixel parameter" hl_lines="5 15-20 33-50"
class camera {
    public:
    double aspect_ratio      = 1.0;  // Ratio of image width over height
    int    image_width       = 100;  // Rendered image width in pixel count
    int    samples_per_pixel = 10;   // Count of random samples for each pixel

    void render(const hittable& world) {
        initialize();

        std::cout << "P3\n" << image_width << ' ' << image_height << "\n255\n";

        for (int j = 0; j < image_height; ++j) {
            std::clog << "\rScanlines remaining: " << (image_height - j) << ' ' << std::flush;
            for (int i = 0; i < image_width; ++i) {
                color pixel_color(0,0,0);
                for (int sample = 0; sample < samples_per_pixel; ++sample) {
                    ray r = get_ray(i, j);
                    pixel_color += ray_color(r, world);
                }
                write_color(std::cout, pixel_color, samples_per_pixel);
            }
        }

        std::clog << "\rDone.                 \n";
    }
    ...
    private:
    ...
    void initialize() {
        ...
    }


    ray get_ray(int i, int j) const {
        // Get a randomly sampled camera ray for the pixel at location i,j.

        auto pixel_center = pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
        auto pixel_sample = pixel_center + pixel_sample_square();

        auto ray_origin = center;
        auto ray_direction = pixel_sample - ray_origin;

        return ray(ray_origin, ray_direction);
    }

    vec3 pixel_sample_square() const {
        // Returns a random point in the square surrounding a pixel at the origin.
        auto px = -0.5 + random_double();
        auto py = -0.5 + random_double();
        return (px * pixel_delta_u) + (py * pixel_delta_v);
    }

    ...
};

#endif
```

(In addition to the new `pixel_sample_square()` function above, you'll also find the function
`pixel_sample_disk()` in the Github source code. This is included in case you'd like to experiment
with non-square pixels, but we won't be using it in this book. `pixel_sample_disk()` depends on the
function `random_in_unit_disk()` which is defined later on.)

Main is updated to set the new camera parameter.

```c++ title="Setting the new samples-per-pixel parameter" hl_lines="8"
int main() {
    ...

    camera cam;

    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 400;
    cam.samples_per_pixel = 100;

    cam.render(world);
}
```

Zooming into the image that is produced, we can see the difference in edge pixels.

![Before and after antialiasing](https://raytracing.github.io/images/img-1.06-antialias-before-after.png)