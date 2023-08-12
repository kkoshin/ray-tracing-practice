### A Final Render
Let’s make the image on the cover of this book -- lots of random spheres.

```c++ title="Final scene" hl_lines="4-42 46-57"
int main() {
    hittable_list world;

    auto ground_material = make_shared<lambertian>(color(0.5, 0.5, 0.5));
    world.add(make_shared<sphere>(point3(0,-1000,0), 1000, ground_material));

    for (int a = -11; a < 11; a++) {
        for (int b = -11; b < 11; b++) {
            auto choose_mat = random_double();
            point3 center(a + 0.9*random_double(), 0.2, b + 0.9*random_double());

            if ((center - point3(4, 0.2, 0)).length() > 0.9) {
                shared_ptr<material> sphere_material;

                if (choose_mat < 0.8) {
                    // diffuse
                    auto albedo = color::random() * color::random();
                    sphere_material = make_shared<lambertian>(albedo);
                    world.add(make_shared<sphere>(center, 0.2, sphere_material));
                } else if (choose_mat < 0.95) {
                    // metal
                    auto albedo = color::random(0.5, 1);
                    auto fuzz = random_double(0, 0.5);
                    sphere_material = make_shared<metal>(albedo, fuzz);
                    world.add(make_shared<sphere>(center, 0.2, sphere_material));
                } else {
                    // glass
                    sphere_material = make_shared<dielectric>(1.5);
                    world.add(make_shared<sphere>(center, 0.2, sphere_material));
                }
            }
        }
    }

    auto material1 = make_shared<dielectric>(1.5);
    world.add(make_shared<sphere>(point3(0, 1, 0), 1.0, material1));

    auto material2 = make_shared<lambertian>(color(0.4, 0.2, 0.1));
    world.add(make_shared<sphere>(point3(-4, 1, 0), 1.0, material2));

    auto material3 = make_shared<metal>(color(0.7, 0.6, 0.5), 0.0);
    world.add(make_shared<sphere>(point3(4, 1, 0), 1.0, material3));

    camera cam;

    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth         = 50;

    cam.vfov     = 20;
    cam.lookfrom = point3(13,2,3);
    cam.lookat   = point3(0,0,0);
    cam.vup      = vec3(0,1,0);

    cam.defocus_angle = 0.6;
    cam.focus_dist    = 10.0;

    cam.render(world);
}
```

(Note that the code above differs slightly from the project sample code: the `samples_per_pixel` is
set to 500 above for a high-quality image that will take quite a while to render.
The sample code uses a value of 10 in the interest of reasonable run times while developing and
validating.)

This gives:

![Final scene](https://raytracing.github.io/images/img-1.23-book1-final.jpg)

An interesting thing you might note is the glass balls don’t really have shadows which makes them
look like they are floating. This is not a bug -- you don’t see glass balls much in real life, where
they also look a bit strange, and indeed seem to float on cloudy days. A point on the big sphere
under a glass ball still has lots of light hitting it because the sky is re-ordered rather than
blocked.

### Next Steps
You now have a cool ray tracer! What next?

  1. Lights -- You can do this explicitly, by sending shadow rays to lights, or it can be done
     implicitly by making some objects emit light, biasing scattered rays toward them, and then
     downweighting those rays to cancel out the bias. Both work. I am in the minority in favoring
     the latter approach.

  2. Triangles -- Most cool models are in triangle form. The model I/O is the worst and almost
     everybody tries to get somebody else’s code to do this.

  3. Surface Textures -- This lets you paste images on like wall paper. Pretty easy and a good thing
     to do.

  4. Solid textures -- Ken Perlin has his code online. Andrew Kensler has some very cool info at his
     blog.

  5. Volumes and Media -- Cool stuff and will challenge your software architecture. I favor making
     volumes have the hittable interface and probabilistically have intersections based on density.
     Your rendering code doesn’t even have to know it has volumes with that method.

  6. Parallelism -- Run $N$ copies of your code on $N$ cores with different random seeds. Average
     the $N$ runs. This averaging can also be done hierarchically where $N/2$ pairs can be averaged
     to get $N/4$ images, and pairs of those can be averaged. That method of parallelism should
     extend well into the thousands of cores with very little coding.

Have fun, and please send me your cool images!