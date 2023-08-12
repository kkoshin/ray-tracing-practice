Lighting is a key component of raytracing. Early simple raytracers used abstract light sources, like
points in space, or directions. Modern approaches have more physically based lights, which have
position and size. To create such light sources, we need to be able to take any regular object and
turn it into something that emits light into our scene.

### Emissive Materials
First, let’s make a light emitting material. We need to add an emitted function (we could also add
it to `hit_record` instead -- that’s a matter of design taste). Like the background, it just tells
the ray what color it is and performs no reflection. It’s very simple:

```c++ title="A diffuse light class"
class diffuse_light : public material {
  public:
    diffuse_light(shared_ptr<texture> a) : emit(a) {}
    diffuse_light(color c) : emit(make_shared<solid_color>(c)) {}

    bool scatter(const ray& r_in, const hit_record& rec, color& attenuation, ray& scattered)
    const override {
        return false;
    }

    color emitted(double u, double v, const point3& p) const override {
        return emit->value(u, v, p);
    }

  private:
    shared_ptr<texture> emit;
};
```

So that I don’t have to make all the non-emitting materials implement `emitted()`, I have the base
class return black:

```c++ title="New emitted function in class material" hl_lines="5-7"
class material {
  public:
    ...

    virtual color emitted(double u, double v, const point3& p) const {
        return color(0,0,0);
    }

    virtual bool scatter(
        const ray& r_in, const hit_record& rec, color& attenuation, ray& scattered
    ) const = 0;
};
```

### Adding Background Color to the Ray Color Function
Next, we want a pure black background so the only light in the scene is coming from the emitters. To
do this, we’ll add a background color parameter to our `ray_color` function, and pay attention to
the new `color_from_emission` value.

```c++ title="ray_color function with background and emitting materials" hl_lines="7 20-33"
class camera {
  public:
    double aspect_ratio      = 1.0;  // Ratio of image width over height
    int    image_width       = 100;  // Rendered image width in pixel count
    int    samples_per_pixel = 10;   // Count of random samples for each pixel
    int    max_depth         = 10;   // Maximum number of ray bounces into scene
    color  background;               // Scene background color

    ...

  private:
    ...
    color ray_color(const ray& r, int depth, const hittable& world) const {
        hit_record rec;

        // If we've exceeded the ray bounce limit, no more light is gathered.
        if (depth <= 0)
            return color(0,0,0);

        // If the ray hits nothing, return the background color.
        if (!world.hit(r, interval(0.001, infinity), rec))
            return background;

        ray scattered;
        color attenuation;
        color color_from_emission = rec.mat->emitted(rec.u, rec.v, rec.p);

        if (!rec.mat->scatter(r, rec, attenuation, scattered))
            return color_from_emission;

        color color_from_scatter = attenuation * ray_color(scattered, depth-1, world);

        return color_from_emission + color_from_scatter;
    }
};
```

`main()` is updated to set the background color for the prior scenes:

```c++ title="Specifying new background color" hl_lines="9 21 33 45 57"
void random_spheres() {
    ...
    camera cam;

    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth         = 50;
    cam.background        = color(0.70, 0.80, 1.00);
    ...
}

void two_spheres() {
    ...
    camera cam;

    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth         = 50;
    cam.background        = color(0.70, 0.80, 1.00);
    ...
}

void earth() {
    ...
    camera cam;

    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth         = 50;
    cam.background        = color(0.70, 0.80, 1.00);
    ...
}

void two_perlin_spheres() {
    ...
    camera cam;

    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth         = 50;
    cam.background        = color(0.70, 0.80, 1.00);
    ...
}

void quads() {
    ...
    camera cam;

    cam.aspect_ratio      = 1.0;
    cam.image_width       = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth         = 50;
    cam.background        = color(0.70, 0.80, 1.00);
    ...
}
```

Since we're removing the code that we used to determine the color of the sky when a ray hit it, we
need to pass in a new color value for our old scene renders. We've elected to stick with a flat
bluish-white for the whole sky. You could always pass in a boolean to switch between the previous
skybox code versus the new solid color background. We're keeping it simple here.

### Turning Objects into Lights
If we set up a rectangle as a light:

```c++ title="A simple rectangle light" hl_lines="1-27 30 36"
void simple_light() {
    hittable_list world;

    auto pertext = make_shared<noise_texture>(4);
    world.add(make_shared<sphere>(point3(0,-1000,0), 1000, make_shared<lambertian>(pertext)));
    world.add(make_shared<sphere>(point3(0,2,0), 2, make_shared<lambertian>(pertext)));

    auto difflight = make_shared<diffuse_light>(color(4,4,4));
    world.add(make_shared<quad>(point3(3,1,-2), vec3(2,0,0), vec3(0,2,0), difflight));

    camera cam;

    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth         = 50;
    cam.background        = color(0,0,0);

    cam.vfov     = 20;
    cam.lookfrom = point3(26,3,6);
    cam.lookat   = point3(0,2,0);
    cam.vup      = vec3(0,1,0);

    cam.defocus_angle = 0;

    cam.render(world);
}

int main() {
    switch (6) {
        case 1:  random_spheres();     break;
        case 2:  two_spheres();        break;
        case 3:  earth();              break;
        case 4:  two_perlin_spheres(); break;
        case 5:  quads();              break;
        case 6:  simple_light();       break;
    }
}
```

We get:

![Scene with rectangle light source](https://raytracing.github.io/images/img-2.17-rect-light.png)

Note that the light is brighter than $(1,1,1)$. This allows it to be bright enough to light things.

Fool around with making some spheres lights too.

```c++ title="A simple rectangle light plus illuminating ball" hl_lines="4"
void simple_light() {
    ...
    auto difflight = make_shared<diffuse_light>(color(4,4,4));
    world.add(make_shared<sphere>(point3(0,7,0), 2, difflight));
    world.add(make_shared<quad>(point3(3,1,-2), vec3(2,0,0), vec3(0,2,0), difflight));
    ...
}
```

![Scene with rectangle and sphere light sources](https://raytracing.github.io/images/img-2.18-rect-sphere-light.png)

### Creating an Empty “Cornell Box”
The “Cornell Box” was introduced in 1984 to model the interaction of light between diffuse surfaces.
Let’s make the 5 walls and the light of the box:

```c++ title="Cornell box scene, empty" hl_lines="1-32 35 42"
void cornell_box() {
    hittable_list world;

    auto red   = make_shared<lambertian>(color(.65, .05, .05));
    auto white = make_shared<lambertian>(color(.73, .73, .73));
    auto green = make_shared<lambertian>(color(.12, .45, .15));
    auto light = make_shared<diffuse_light>(color(15, 15, 15));

    world.add(make_shared<quad>(point3(555,0,0), vec3(0,555,0), vec3(0,0,555), green));
    world.add(make_shared<quad>(point3(0,0,0), vec3(0,555,0), vec3(0,0,555), red));
    world.add(make_shared<quad>(point3(343, 554, 332), vec3(-130,0,0), vec3(0,0,-105), light));
    world.add(make_shared<quad>(point3(0,0,0), vec3(555,0,0), vec3(0,0,555), white));
    world.add(make_shared<quad>(point3(555,555,555), vec3(-555,0,0), vec3(0,0,-555), white));
    world.add(make_shared<quad>(point3(0,0,555), vec3(555,0,0), vec3(0,555,0), white));

    camera cam;

    cam.aspect_ratio      = 1.0;
    cam.image_width       = 600;
    cam.samples_per_pixel = 200;
    cam.max_depth         = 50;
    cam.background        = color(0,0,0);

    cam.vfov     = 40;
    cam.lookfrom = point3(278, 278, -800);
    cam.lookat   = point3(278, 278, 0);
    cam.vup      = vec3(0,1,0);

    cam.defocus_angle = 0;

    cam.render(world);
}

int main() {
    switch (7) {
        case 1:  random_spheres();     break;
        case 2:  two_spheres();        break;
        case 3:  earth();              break;
        case 4:  two_perlin_spheres(); break;
        case 5:  quads();              break;
        case 6:  simple_light();       break;
        case 7:  cornell_box();        break;
    }
}
```

We get:

![Empty Cornell box](https://raytracing.github.io/images/img-2.19-cornell-empty.png)

This image is very noisy because the light is small.