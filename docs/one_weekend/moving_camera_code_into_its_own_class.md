Before continuing, now is a good time to consolidate our camera and scene-render code into a single
new class: the `camera` class.
The camera class will be responsible for two important jobs:

  1. Construct and dispatch rays into the world.
  2. Use the results of these rays to construct the rendered image.

In this refactoring, we'll collect the `ray_color()` function, along with the image, camera, and
render sections of our main program.
The new camera class will contain two public methods `initialize()` and `render()`, plus two
private helper methods `get_ray()` and `ray_color()`.

Ultimately, the camera will follow the simplest usage pattern that we could think of: it will be
default constructed no arguments, then the owning code will modify the camera's public variables
through simple assignment, and finally everything is initialized by a call to the `initialize()`
function. This pattern is chosen instead of the owner calling a constructor with a ton of parameters
or by defining and calling a bunch of setter methods. Instead, the owning code only needs to set
what it explicitly cares about. Finally, we could either have the owning code call `initialize()`,
or just have the camera call this function automatically at the start of `render()`. We'll use the
second approach.

After main creates a camera and sets default values, it will call the `render()` method.
The `render()` method will prepare the camera for rendering and then execute the render loop.

Here's the skeleton of our new `camera` class:

```c++ title="The camera class skeleton"
#ifndef CAMERA_H
#define CAMERA_H

#include "rtweekend.h"

#include "color.h"
#include "hittable.h"

class camera {
    public:
    /* Public Camera Parameters Here */

    void render(const hittable& world) {
        ...
    }

    private:
    /* Private Camera Variables Here */

    void initialize() {
        ...
    }

    color ray_color(const ray& r, const hittable& world) const {
        ...
    }
};

#endif
```

To begin with, let's fill in the `ray_color()` function from `main.cc`:

```c++ title="The camera::ray_color function" hl_lines="7-17"
class camera {
    ...

    private:
    ...


    color ray_color(const ray& r, const hittable& world) const {
        hit_record rec;

        if (world.hit(r, interval(0, infinity), rec)) {
            return 0.5 * (rec.normal + color(1,1,1));
        }

        vec3 unit_direction = unit_vector(r.direction());
        auto a = 0.5*(unit_direction.y() + 1.0);
        return (1.0-a)*color(1.0, 1.0, 1.0) + a*color(0.5, 0.7, 1.0);
    }
};

#endif
```

Now we move almost everything from the `main()` function into our new camera class.
The only thing remaining in the `main()` function is the world construction.
Here's the camera class with newly migrated code:

```c++ title="The working camera class" hl_lines="6 10-31 34-63"
...
#include "rtweekend.h"

#include "color.h"
#include "hittable.h"


#include <iostream>

class camera {
    public:
    double aspect_ratio = 1.0;  // Ratio of image width over height
    int    image_width  = 100;  // Rendered image width in pixel count

    void render(const hittable& world) {
        initialize();

        std::cout << "P3\n" << image_width << ' ' << image_height << "\n255\n";

        for (int j = 0; j < image_height; ++j) {
            std::clog << "\rScanlines remaining: " << (image_height - j) << ' ' << std::flush;
            for (int i = 0; i < image_width; ++i) {
                auto pixel_center = pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
                auto ray_direction = pixel_center - center;
                ray r(center, ray_direction);

                color pixel_color = ray_color(r, world);
                write_color(std::cout, pixel_color);
            }
        }

        std::clog << "\rDone.                 \n";
    }

    private:
    int    image_height;   // Rendered image height
    point3 center;         // Camera center
    point3 pixel00_loc;    // Location of pixel 0, 0
    vec3   pixel_delta_u;  // Offset to pixel to the right
    vec3   pixel_delta_v;  // Offset to pixel below

    void initialize() {
        image_height = static_cast<int>(image_width / aspect_ratio);
        image_height = (image_height < 1) ? 1 : image_height;

        center = point3(0, 0, 0);

        // Determine viewport dimensions.
        auto focal_length = 1.0;
        auto viewport_height = 2.0;
        auto viewport_width = viewport_height * (static_cast<double>(image_width)/image_height);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        auto viewport_u = vec3(viewport_width, 0, 0);
        auto viewport_v = vec3(0, -viewport_height, 0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        pixel_delta_u = viewport_u / image_width;
        pixel_delta_v = viewport_v / image_height;

        // Calculate the location of the upper left pixel.
        auto viewport_upper_left =
            center - vec3(0, 0, focal_length) - viewport_u/2 - viewport_v/2;
        pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    }

    color ray_color(const ray& r, const hittable& world) const {
        ...
    }
};

#endif
```

And here's the much reduced main:

```c++ title="The new main, using the new camera"
#include "rtweekend.h"

#include "camera.h"
#include "hittable_list.h"
#include "sphere.h"

int main() {
    hittable_list world;

    world.add(make_shared<sphere>(point3(0,0,-1), 0.5));
    world.add(make_shared<sphere>(point3(0,-100.5,-1), 100));

    camera cam;

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width  = 400;

    cam.render(world);
}
```

Running this newly refactored program should give us the same rendered image as before.