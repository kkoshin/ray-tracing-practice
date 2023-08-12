One thing it’s nice to add to a ray tracer is smoke/fog/mist. These are sometimes called _volumes_
or _participating media_. Another feature that is nice to add is subsurface scattering, which is
sort of like dense fog inside an object. This usually adds software architectural mayhem because
volumes are a different animal than surfaces, but a cute technique is to make a volume a random
surface. A bunch of smoke can be replaced with a surface that probabilistically might or might not
be there at every point in the volume. This will make more sense when you see the code.

### Constant Density Mediums
First, let’s start with a volume of constant density. A ray going through there can either scatter
inside the volume, or it can make it all the way through like the middle ray in the figure. More
thin transparent volumes, like a light fog, are more likely to have rays like the middle one. How
far the ray has to travel through the volume also determines how likely it is for the ray to make it
through.

![Ray-volume interaction](https://raytracing.github.io/images/fig-2.10-ray-vol.jpg)

As the ray passes through the volume, it may scatter at any point. The denser the volume, the more
likely that is. The probability that the ray scatters in any small distance $\Delta L$ is:

  $$ \mathit{probability} = C \cdot \Delta L $$

where $C$ is proportional to the optical density of the volume. If you go through all the
differential equations, for a random number you get a distance where the scattering occurs. If that
distance is outside the volume, then there is no “hit”. For a constant volume we just need the
density $C$ and the boundary. I’ll use another hittable for the boundary.

The resulting class is:

```c++ title="Constant medium class"
#ifndef CONSTANT_MEDIUM_H
#define CONSTANT_MEDIUM_H

#include "rtweekend.h"

#include "hittable.h"
#include "material.h"
#include "texture.h"

class constant_medium : public hittable {
  public:
    constant_medium(shared_ptr<hittable> b, double d, shared_ptr<texture> a)
      : boundary(b), neg_inv_density(-1/d), phase_function(make_shared<isotropic>(a))
    {}

    constant_medium(shared_ptr<hittable> b, double d, color c)
      : boundary(b), neg_inv_density(-1/d), phase_function(make_shared<isotropic>(c))
    {}

    bool hit(const ray& r, interval ray_t, hit_record& rec) const override {
        // Print occasional samples when debugging. To enable, set enableDebug true.
        const bool enableDebug = false;
        const bool debugging = enableDebug && random_double() < 0.00001;

        hit_record rec1, rec2;

        if (!boundary->hit(r, interval::universe, rec1))
            return false;

        if (!boundary->hit(r, interval(rec1.t+0.0001, infinity), rec2))
            return false;

        if (debugging) std::clog << "\nray_tmin=" << rec1.t << ", ray_tmax=" << rec2.t << '\n';

        if (rec1.t < ray_t.min) rec1.t = ray_t.min;
        if (rec2.t > ray_t.max) rec2.t = ray_t.max;

        if (rec1.t >= rec2.t)
            return false;

        if (rec1.t < 0)
            rec1.t = 0;

        auto ray_length = r.direction().length();
        auto distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        auto hit_distance = neg_inv_density * log(random_double());

        if (hit_distance > distance_inside_boundary)
            return false;

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        if (debugging) {
            std::clog << "hit_distance = " <<  hit_distance << '\n'
                      << "rec.t = " <<  rec.t << '\n'
                      << "rec.p = " <<  rec.p << '\n';
        }

        rec.normal = vec3(1,0,0);  // arbitrary
        rec.front_face = true;     // also arbitrary
        rec.mat = phase_function;

        return true;
    }

    aabb bounding_box() const override { return boundary->bounding_box(); }

  private:
    shared_ptr<hittable> boundary;
    double neg_inv_density;
    shared_ptr<material> phase_function;
};

#endif
```

The scattering function of isotropic picks a uniform random direction:

```c++ title="The isotropic class"
class isotropic : public material {
  public:
    isotropic(color c) : albedo(make_shared<solid_color>(c)) {}
    isotropic(shared_ptr<texture> a) : albedo(a) {}

    bool scatter(const ray& r_in, const hit_record& rec, color& attenuation, ray& scattered)
    const override {
        scattered = ray(rec.p, random_unit_vector(), r_in.time());
        attenuation = albedo->value(rec.u, rec.v, rec.p);
        return true;
    }

  private:
    shared_ptr<texture> albedo;
};
```

The reason we have to be so careful about the logic around the boundary is we need to make sure this
works for ray origins inside the volume. In clouds, things bounce around a lot so that is a common
case.

In addition, the above code assumes that once a ray exits the constant medium boundary, it will
continue forever outside the boundary. Put another way, it assumes that the boundary shape is
convex. So this particular implementation will work for boundaries like boxes or spheres, but will
not work with toruses or shapes that contain voids. It's possible to write an implementation that
handles arbitrary shapes, but we'll leave that as an exercise for the reader.

### Rendering a Cornell Box with Smoke and Fog Boxes
If we replace the two blocks with smoke and fog (dark and light particles), and make the light
bigger (and dimmer so it doesn’t blow out the scene) for faster convergence:

```c++ title="Cornell box, with smoke" hl_lines="1 3-45 48 56"
#include "constant_medium.h"
...
void cornell_smoke() {
    hittable_list world;

    auto red   = make_shared<lambertian>(color(.65, .05, .05));
    auto white = make_shared<lambertian>(color(.73, .73, .73));
    auto green = make_shared<lambertian>(color(.12, .45, .15));
    auto light = make_shared<diffuse_light>(color(7, 7, 7));

    world.add(make_shared<quad>(point3(555,0,0), vec3(0,555,0), vec3(0,0,555), green));
    world.add(make_shared<quad>(point3(0,0,0), vec3(0,555,0), vec3(0,0,555), red));
    world.add(make_shared<quad>(point3(113,554,127), vec3(330,0,0), vec3(0,0,305), light));
    world.add(make_shared<quad>(point3(0,555,0), vec3(555,0,0), vec3(0,0,555), white));
    world.add(make_shared<quad>(point3(0,0,0), vec3(555,0,0), vec3(0,0,555), white));
    world.add(make_shared<quad>(point3(0,0,555), vec3(555,0,0), vec3(0,555,0), white));

    shared_ptr<hittable> box1 = box(point3(0,0,0), point3(165,330,165), white);
    box1 = make_shared<rotate_y>(box1, 15);
    box1 = make_shared<translate>(box1, vec3(265,0,295));

    shared_ptr<hittable> box2 = box(point3(0,0,0), point3(165,165,165), white);
    box2 = make_shared<rotate_y>(box2, -18);
    box2 = make_shared<translate>(box2, vec3(130,0,65));

    world.add(make_shared<constant_medium>(box1, 0.01, color(0,0,0)));
    world.add(make_shared<constant_medium>(box2, 0.01, color(1,1,1)));

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
    switch (8) {
        case 1:  random_spheres();     break;
        case 2:  two_spheres();        break;
        case 3:  earth();              break;
        case 4:  two_perlin_spheres(); break;
        case 5:  quads();              break;
        case 6:  simple_light();       break;
        case 7:  cornell_box();        break;
        case 8:  cornell_smoke();      break;
    }
}
```

We get:

![Cornell box with blocks of smoke](https://raytracing.github.io/images/img-2.22-cornell-smoke.png)