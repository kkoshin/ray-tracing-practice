Now our final feature: _defocus blur_.
Note, photographers call this _depth of field_, so be sure to only use the term _defocus blur_ among
your raytracing friends.

The reason we have defocus blur in real cameras is because they need a big hole (rather than just a
pinhole) through which to gather light.
A large hole would defocus everything, but if we stick a lens in front of the film/sensor, there
will be a certain distance at which everything is in focus.
Objects placed at that distance will appear in focus and will linearly appear blurrier the further
they are from that distance.
You can think of a lens this way: all light rays coming _from_ a specific point at the focus
distance -- and that hit the lens -- will be bent back _to_ a single point on the image sensor.

We call the distance between the camera center and the plane where everything is in perfect focus
the _focus distance_.
Be aware that the focus distance is not usually the same as the focal length -- the _focal length_
is the distance between the camera center and the image plane.
For our model, however, these two will have the same value, as we will put our pixel grid right on
the focus plane, which is _focus distance_ away from the camera center.

In a physical camera, the focus distance is controlled by the distance between the lens and the
film/sensor.
That is why you see the lens move relative to the camera when you change what is in focus (that may
happen in your phone camera too, but the sensor moves).
The “aperture” is a hole to control how big the lens is effectively.
For a real camera, if you need more light you make the aperture bigger, and will get more blur for
objects away from the focus distance.
For our virtual camera, we can have a perfect sensor and never need more light, so we only use an
aperture when we want defocus blur.

### A Thin Lens Approximation
A real camera has a complicated compound lens. For our code, we could simulate the order: sensor,
then lens, then aperture. Then we could figure out where to send the rays, and flip the image after
it's computed (the image is projected upside down on the film). Graphics people, however, usually
use a thin lens approximation:

![Camera lens model](https://raytracing.github.io/images/fig-1.21-cam-lens.jpg)

We don’t need to simulate any of the inside of the camera -- for the purposes of rendering an image
outside the camera, that would be unnecessary complexity.
Instead, I usually start rays from an infinitely thin circular "lens", and send them toward the
pixel of interest on the focus plane (`focal_length` away from the lens), where everything on that
plane in the 3D world is in perfect focus.

In practice, we accomplish this by placing the viewport in this plane.
Putting everything together:

  1. The focus plane is orthogonal to the camera view direction.
  2. The focus distance is the distance between the camera center and the focus plane.
  3. The viewport lies on the focus plane, centered on the camera view direction vector.
  4. The grid of pixel locations lies inside the viewport (located in the 3D world).
  5. Random image sample locations are chosen from the region around the current pixel location.
  6. The camera fires rays from random points on the lens through the current image sample location.

![Camera focus plane](https://raytracing.github.io/images/fig-1.22-cam-film-plane.jpg)

### Generating Sample Rays
Without defocus blur, all scene rays originate from the camera center (or `lookfrom`).
In order to accomplish defocus blur, we construct a disk centered at the camera center.
The larger the radius, the greater the defocus blur.
You can think of our original camera as having a defocus disk of radius zero (no blur at all), so
all rays originated at the disk center (`lookfrom`).

So, how large should the defocus disk be?
Since the size of this disk controls how much defocus blur we get, that should be a parameter of the
camera class.
We could just take the radius of the disk as a camera parameter, but the blur would vary depending
on the projection distance.
A slightly easier parameter is to specify the angle of the cone with apex at viewport center and
base (defocus disk) at the camera center.
This should give you more consistent results as you vary the focus distance for a given shot.

Since we'll be choosing random points from the defocus disk, we'll need a function to do that:
`random_in_unit_disk()`.
This function works using the same kind of method we use in `random_in_unit_sphere()`, just for two
dimensions.

```c++ title="Generate random point inside unit disk" hl_lines="5-11"
inline vec3 unit_vector(vec3 u) {
    return v / v.length();
}

inline vec3 random_in_unit_disk() {
    while (true) {
        auto p = vec3(random_double(-1,1), random_double(-1,1), 0);
        if (p.length_squared() < 1)
            return p;
    }
}
```

Now let's update the camera to originate rays from the defocus disk:

```c++ title="Camera with adjustable depth-of-field (dof)" hl_lines="13 14 25 26 38 55 58-61 66 67 72 79-83"
class camera {
  public:
    double aspect_ratio      = 1.0;  // Ratio of image width over height
    int    image_width       = 100;  // Rendered image width in pixel count
    int    samples_per_pixel = 10;   // Count of random samples for each pixel
    int    max_depth         = 10;   // Maximum number of ray bounces into scene

    double vfov     = 90;              // Vertical view angle (field of view)
    point3 lookfrom = point3(0,0,-1);  // Point camera is looking from
    point3 lookat   = point3(0,0,0);   // Point camera is looking at
    vec3   vup      = vec3(0,1,0);     // Camera-relative "up" direction

    double defocus_angle = 0;  // Variation angle of rays through each pixel
    double focus_dist = 10;    // Distance from camera lookfrom point to plane of perfect focus

    ...

  private:
    int    image_height;    // Rendered image height
    point3 center;          // Camera center
    point3 pixel00_loc;     // Location of pixel 0, 0
    vec3   pixel_delta_u;   // Offset to pixel to the right
    vec3   pixel_delta_v;   // Offset to pixel below
    vec3   u, v, w;         // Camera frame basis vectors
    vec3   defocus_disk_u;  // Defocus disk horizontal radius
    vec3   defocus_disk_v;  // Defocus disk vertical radius

    void initialize() {
        image_height = static_cast<int>(image_width / aspect_ratio);
        image_height = (image_height < 1) ? 1 : image_height;

        center = lookfrom;

        // Determine viewport dimensions.
        // auto focal_length = (lookfrom - lookat).length();
        auto theta = degrees_to_radians(vfov);
        auto h = tan(theta/2);
        auto viewport_height = 2 * h * focus_dist;
        auto viewport_width = viewport_height * (static_cast<double>(image_width)/image_height);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        w = unit_vector(lookfrom - lookat);
        u = unit_vector(cross(vup, w));
        v = cross(w, u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        vec3 viewport_u = viewport_width * u;    // Vector across viewport horizontal edge
        vec3 viewport_v = viewport_height * -v;  // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors to the next pixel.
        pixel_delta_u = viewport_u / image_width;
        pixel_delta_v = viewport_v / image_height;

        // Calculate the location of the upper left pixel.
        auto viewport_upper_left = center - (focus_dist * w) - viewport_u/2 - viewport_v/2;
        pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        auto defocus_radius = focus_dist * tan(degrees_to_radians(defocus_angle / 2));
        defocus_disk_u = u * defocus_radius;
        defocus_disk_v = v * defocus_radius;
    }


    ray get_ray(int i, int j) const {
        // Get a randomly-sampled camera ray for the pixel at location i,j, originating from
        // the camera defocus disk.

        auto pixel_center = pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
        auto pixel_sample = pixel_center + pixel_sample_square();

        auto ray_origin = (defocus_angle <= 0) ? center : defocus_disk_sample();
        auto ray_direction = pixel_sample - ray_origin;

        return ray(ray_origin, ray_direction);
    }

    ...
    point3 defocus_disk_sample() const {
        // Returns a random point in the camera defocus disk.
        auto p = random_in_unit_disk();
        return center + (p[0] * defocus_disk_u) + (p[1] * defocus_disk_v);
    }

    color ray_color(const ray& r, int depth, const hittable& world) const {
    ...
};
```

Using a large aperture:

```c++ title="Scene camera with depth-of-field" hl_lines="16 17"
int main() {
    ...

    camera cam;

    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth         = 50;

    cam.vfov     = 20;
    cam.lookfrom = point3(-2,2,1);
    cam.lookat   = point3(0,0,-1);
    cam.vup      = vec3(0,1,0);


    cam.defocus_angle = 10.0;
    cam.focus_dist    = 3.4;

    cam.render(world);
}
```

We get:

![Spheres with depth-of-field](https://raytracing.github.io/images/img-1.22-depth-of-field.png)