We have used a PDF related to $\cos(\theta)$, and a PDF related to sampling the light. We would like
a PDF that combines these.

### The PDF Class
We've worked with PDFs in quite a lot of code already. I think that now is a good time to figure out
how we want to standardize our usage of PDFs. We already know that we are going to have a PDF for
the surface and a PDF for the light, so let's create a `pdf` base class. So far, we've had a `pdf()`
function that took a direction and returned the PDF's distribution value for that direction. This
value has so far been one of $1/4\pi$, $1/2\pi$, and $\cos(\theta)/\pi$. In a couple of our examples
we generated the random direction using a different distribution than the
distribution of the PDF. We covered this quite a lot in the chapter Playing with Importance
Sampling. In general, if we know the distribution of our random directions, we should use a PDF with
the same distribution. This will lead to the fastest convergence. With that in mind, we'll create a
`pdf` class that is responsible for generating random directions and determining the value of the
PDF.

From all of this, any `pdf` class should be responsible for

  1. returning a random direction weighted by the internal PDF distribution, and
  2. returning the corresponding PDF distribution value in that direction.

The details of how this is done under the hood varies for $\operatorname{pSurface}$ and
  $\operatorname{pLight}$, but that is exactly what class hierarchies were invented for!
It’s never obvious what goes in an abstract class, so my approach is to be greedy and hope a minimal
  interface works, and for `pdf` this implies:

```c++ title="The abstract pdf class"
#ifndef PDF_H
#define PDF_H

#include "rtweekend.h"

#include "onb.h"


class pdf {
  public:
    virtual ~pdf() {}

    virtual double value(const vec3& direction) const = 0;
    virtual vec3 generate() const = 0;
};

#endif
```

We’ll see if we need to add anything else to `pdf` by fleshing out the subclasses. First, we'll
create a uniform density over the unit sphere:

```c++ title="The uniform_pdf class"
class sphere_pdf : public pdf {
  public:
    sphere_pdf() { }

    double value(const vec3& direction) const override {
        return 1/ (4 * pi);
    }

    vec3 generate() const override {
        return random_unit_vector();
    }
};
```

Next, let’s try a cosine density:

```c++ title="The cosine_pdf class"
class cosine_pdf : public pdf {
  public:
    cosine_pdf(const vec3& w) { uvw.build_from_w(w); }

    double value(const vec3& direction) const override {
        auto cosine_theta = dot(unit_vector(direction), uvw.w());
        return fmax(0, cosine_theta/pi);
    }

    vec3 generate() const override {
        return uvw.local(random_cosine_direction());
    }

  private:
    onb uvw;
};
```

We can try this cosine PDF in the `ray_color()` function:

```c++ title="The ray_color function, using cosine pdf" hl_lines="18-19 24-26 31"
class camera {
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
        double pdf_val;
        color color_from_emission = rec.mat->emitted(r, rec, rec.u, rec.v, rec.p);

        if (!rec.mat->scatter(r, rec, attenuation, scattered, pdf_val))
            return color_from_emission;

        cosine_pdf surface_pdf(rec.normal);
        scattered = ray(rec.p, surface_pdf.generate(), r.time());
        pdf_val = surface_pdf.value(scattered.direction());

        double scattering_pdf = rec.mat->scattering_pdf(r, rec, scattered);

        color color_from_scatter =
            (attenuation * scattering_pdf * ray_color(scattered, depth-1, world)) / pdf_val;

        return color_from_emission + color_from_scatter;
    }
};
```

This yields an exactly matching result so all we’ve done so far is move some computation up into the
`cosine_pdf` class:

![Cornell box with a cosine density PDF](https://raytracing.github.io/images/img-3.09-cornell-cos-pdf.jpg)

### Sampling Directions towards a Hittable
Now we can try sampling directions toward a `hittable`, like the light.

```c++ title="The hittable_pdf class"
...
#include "hittable_list.h"
...
class hittable_pdf : public pdf {
  public:
    hittable_pdf(const hittable& _objects, const point3& _origin)
      : objects(_objects), origin(_origin)
    {}

    double value(const vec3& direction) const override {
        return objects.pdf_value(origin, direction);
    }

    vec3 generate() const override {
        return objects.random(origin);
    }

  private:
    const hittable& objects;
    point3 origin;
};
```

If we want to sample the light, we will need `hittable` to answer some queries that it doesn’t yet
have an interface for. The above code assumes the existence of two as-of-yet unimplemented functions
in the `hittable` class: `pdf_value()` and `random()`. We need to add these functions for the
program to compile. We could go through all of the `hittable` subclasses and add these functions,
but that would be a hassle, so we’ll just add two trivial functions to the `hittable` base class.
This breaks our previously pure abstract implementation, but it saves work. Feel free to write these
functions through to subclasses if you want a purely abstract `hittable` interface class.

```c++ title="The hittable class, with two new methods" hl_lines="5-11"
class hittable {
  public:
    ...

    virtual double pdf_value(const point3& o, const vec3& v) const {
        return 0.0;
    }

    virtual vec3 random(const vec3& o) const {
        return vec3(1, 0, 0);
    }
};
```

And then we change `quad` to implement those functions:

```c++ title="quad with pdf" hl_lines="11 17-31 41"
class quad : public hittable {
  public:
    quad(const point3& _Q, const vec3& _u, const vec3& _v, shared_ptr<material> m)
      : Q(_Q), u(_u), v(_v), mat(m)
    {
        auto n = cross(u, v);
        normal = unit_vector(n);
        D = dot(normal, Q);
        w = n / dot(n,n);

        area = n.length();

        set_bounding_box();
    }
    ...

    double pdf_value(const point3& origin, const vec3& v) const override {
        hit_record rec;
        if (!this->hit(ray(origin, v), interval(0.001, infinity), rec))
            return 0;

        auto distance_squared = rec.t * rec.t * v.length_squared();
        auto cosine = fabs(dot(v, rec.normal) / v.length());

        return distance_squared / (cosine * area);
    }

    vec3 random(const point3& origin) const override {
        auto p = plane_origin + (random_double() * axis_A) + (random_double() * axis_B);
        return p - origin;
    }

  private:
    point3 Q;
    vec3 u, v;
    shared_ptr<material> mat;
    aabb bbox;
    vec3 normal;
    double D;
    vec3 w;
    double area;
};
```

We only need to add `pdf_value()` and `random()` to `quad` because we're using this to importance
sample the light, and the only light we have in our scene is a `quad`. if you want other light
geometries, or want to use a PDF with other objects, you'll need to implement the above functions
for the corresponding classes.

Add a `lights` parameter to the camera `render()` function:

```c++ title="ray_color function with light PDF" hl_lines="4 17 42-44 48-49"
class camera {
  public:
    ...
    void render(const hittable& world, const hittable& lights) {
        initialize();

        std::cout << "P3\n" << image_width << ' ' << image_height << "\n255\n";

        int sqrt_spp = int(sqrt(samples_per_pixel));
        for (int j = 0; j < image_height; ++j) {
            std::clog << "\rScanlines remaining: " << (image_height - j) << ' ' << std::flush;
            for (int i = 0; i < image_width; ++i) {
                color pixel_color(0,0,0);
                for (int s_j = 0; s_j < sqrt_spp; ++s_j) {
                    for (int s_i = 0; s_i < sqrt_spp; ++s_i) {
                        ray r = get_ray(i, j, s_i, s_j);
                        pixel_color += ray_color(r, max_depth, world, lights);
                    }
                }
                write_color(std::cout, pixel_color, samples_per_pixel);
            }
        }

        std::clog << "\rDone.                 \n";
    }

    ...
  private:
    ...
    color ray_color(const ray& r, int depth, const hittable& world, const hittable& lights)
    const {
        ...

        ray scattered;
        color attenuation;
        double pdf_val;
        color color_from_emission = rec.mat->emitted(r, rec, rec.u, rec.v, rec.p);

        if (!rec.mat->scatter(r, rec, attenuation, scattered, pdf_val))
            return color_from_emission;

        hittable_pdf light_pdf(light_ptr, rec.p);
        scattered = ray(rec.p, light_pdf.generate(), r.time());
        pdf_val = light_pdf.value(scattered.direction());

        double scattering_pdf = rec.mat->scattering_pdf(r, rec, scattered);

        color sample_color = ray_color(scattered, depth-1, world, lights);
        color color_from_scatter = (attenuation * scattering_pdf * sample_color) / pdf_val;

        return color_from_emission + color_from_scatter;
    }
};
```

Create a light in the middle of the ceiling:

```c++ title="Adding a light to the Cornell box" hl_lines="10-13 18"
int main() {
    ...

    // Box 2
    shared_ptr<hittable> box2 = box(point3(0,0,0), point3(165,165,165), white);
    box2 = make_shared<rotate_y>(box2, -18);
    box2 = make_shared<translate>(box2, vec3(130,0,65));
    world.add(box2);

    // Light Sources
    hittable_list lights;
    auto m = shared_ptr<material>();
    lights.add(make_shared<quad>(point3(343,554,332), vec3(-130,0,0), vec3(0,0,-105), m));

    camera cam;
    ...

    cam.render(world, lights);
}
```

At 10 samples per pixel we get:

![Cornell box, sampling a hittable light, 10 samples per pixel](https://raytracing.github.io/images/img-3.10-hittable-light.jpg)

### The Mixture PDF Class
As was briefly mentioned in the chapter Playing with Importance Sampling, we can create linear
mixtures of any PDFs to form mixture densities that are also PDFs. Any weighted average of PDFs is
also a PDF. As long as the weights are positive and add up to any one, we have a new PDF.

  $$ \operatorname{pMixture}() = w_0 p_0() + w_1 p_1() + w_2 p_2() + \ldots + w_{n-1} p_{n-1}() $$

  $$ 1 = w_0 + w_1 + w_2 + \ldots + w_{n-1} $$

For example, we could just average the two densities:

  $$ \operatorname{pMixture}(\omega_o)
     = \frac{1}{2} \operatorname{pSurface}(\omega_o) + \frac{1}{2} \operatorname{pLight}(\omega_o)
  $$

How would we instrument our code to do that? There is a very important detail that makes this not
quite as easy as one might expect. Generating the random direction for a mixture PDF is simple:

```c++
if (random_double() < 0.5)
    pick direction according to pSurface
else
    pick direction according to pLight
```

But solving for the PDF value of $\operatorname{pMixture}$ is slightly more subtle. We can't just

```c++
if (direction is from pSurface)
    get PDF value of pSurface
else
    get PDF value of pLight
```

For one, figuring out which PDF the random direction came from is probably not trivial. We don't
have any plumbing for `generate()` to tell `value()` what the original `random_double()` was, so we
can't trivially say which PDF the random direction comes from. If we thought that the above was
correct, we would have to solve backwards to figure which PDF the direction could come from. Which
honestly sounds like a nightmare, but fortunately we don't need to do that. There are some
directions that both PDFs could have generated.
For example, a direction toward the light could have been generated by either
  $\operatorname{pLight}$ _or_ $\operatorname{pSurface}$.
It is sufficient for us to solve for the pdf value of $\operatorname{pSurface}$ and of
  $\operatorname{pLight}$ for a random direction and then take the PDF mixture weights to solve for
  the total PDF value for that direction.
The mixture density class is actually pretty straightforward:

```c++ title="The mixture_pdf class"
class mixture_pdf : public pdf {
  public:
    mixture_pdf(shared_ptr<pdf> p0, shared_ptr<pdf> p1) {
        p[0] = p0;
        p[1] = p1;
    }

    double value(const vec3& direction) const override {
        return 0.5 * p[0]->value(direction) + 0.5 *p[1]->value(direction);
    }

    vec3 generate() const override {
        if (random_double() < 0.5)
            return p[0]->generate();
        else
            return p[1]->generate();
    }

  private:
    shared_ptr<pdf> p[2];
};
```

Now we would like to do a mixture density of the cosine sampling and of the light sampling. We can
plug it into `ray_color()`:

```c++ title="The ray_color function, using mixture PDF" hl_lines="12-17"
class camera {
  ...
  private:
    ...
    color ray_color(const ray& r, int depth, const hittable& world, const hittable& lights)
    const {
        ...

        if (!rec.mat->scatter(r, rec, attenuation, scattered, pdf_val))
            return color_from_emission;

        auto p0 = make_shared<hittable_pdf>(light_ptr, rec.p);
        auto p1 = make_shared<cosine_pdf>(rec.normal);
        mixture_pdf mixed_pdf(p0, p1);

        scattered = ray(rec.p, mixed_pdf.generate(), r.time());
        pdf_val = mixed_pdf.value(scattered.direction());

        double scattering_pdf = rec.mat->scattering_pdf(r, rec, scattered);

        color sample_color = ray_color(scattered, depth-1, world, lights);
        color color_from_scatter = (attenuation * scattering_pdf * sample_color) / pdf_val;

        return color_from_emission + color_from_scatter;
    }
```

1000 samples per pixel yields:

![Cornell box, mixture density of cosine and light sampling](https://raytracing.github.io/images/img-3.11-cosine-and-light.jpg)