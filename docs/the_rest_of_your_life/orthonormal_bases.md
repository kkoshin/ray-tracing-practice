In the last chapter we developed methods to generate random directions relative to the $z$ axis. If
we want to be able to produce reflections off of any surface, we are going to need to make this more
general: Not all normals are going to be perfectly aligned with the $z$ axis. So in this chapter
we are going to generalize our methods so that they support arbitrary surface normal vectors.

### Relative Coordinates
An _orthonormal basis_ (ONB) is a collection of three mutually orthogonal unit vectors. It is a
strict subtype of coordinate system. The Cartesian $xyz$ axes are one example of an orthonormal
basis. All of our renders are the result of the relative positions and orientations of the objects
in a scene projected onto the image plane of the camera. The camera and objects must be described in
the same coordinate system, so that the projection onto the image plane is logically defined,
otherwise the camera has no definitive means of correctly rendering the objects. Either the camera
must be redefined in the objects' coordinate system, or the objects must be redefined in the
camera's coordinate system. It's best to start with both in the same coordinate system, so no
redefinition is necessary. So long as the camera and scene are described in the same coordinate
system, all is well. The orthonormal basis defines how distances and orientations are represented in
the space, but an orthonormal basis alone is not enough. The objects and the camera need to
described by their displacement from a mutually defined location. This is just the origin
$\mathbf{O}$ of the scene; it represents the center of the universe for everything to displace from.

Suppose we have an origin $\mathbf{O}$ and Cartesian unit vectors $\mathbf{x}$, $\mathbf{y}$, and
$\mathbf{z}$. When we say a location is (3,-2,7), we really are saying:

    $$ \text{Location is } \mathbf{O} + 3\mathbf{x} - 2\mathbf{y} + 7\mathbf{z} $$

If we want to measure coordinates in another coordinate system with origin $\mathbf{O}'$ and basis
vectors $\mathbf{u}$, $\mathbf{v}$, and $\mathbf{w}$, we can just find the numbers $(u,v,w)$ such
that:

    $$ \text{Location is } \mathbf{O}' + u\mathbf{u} + v\mathbf{v} + w\mathbf{w} $$

### Generating an Orthonormal Basis
If you take an intro to graphics course, there will be a lot of time spent on coordinate systems and
4×4 coordinate transformation matrices. Pay attention, it’s really important stuff! But we won’t be
needing it for this book and we'll make do without it. What we do need is to generate random
directions with a set distribution relative to the surface normal vector $\mathbf{n}$. We won’t be
needing an origin for this because a direction is relative and has no specific origin. To start off
with, we need two cotangent vectors that are each perpendicular to $\mathbf{n}$ and that are also
perpendicular to each other.

Some 3D object models will come with one or more cotangent vectors for each vertex. If our model has
only one cotangent vector, then the process of making an ONB is a nontrivial one. Suppose we have
any vector $\mathbf{a}$ that is of nonzero length and nonparallel with $\mathbf{n}$. We can get
vectors $\mathbf{s}$ and $\mathbf{t}$ perpendicular to $\mathbf{n}$ by using the property of the
cross product that $\mathbf{n} \times \mathbf{a}$ is perpendicular to both $\mathbf{n}$ and
$\mathbf{a}$:

    $$ \mathbf{s} = \operatorname{unit_vector}(\mathbf{n} \times \mathbf{a}) $$

    $$ \mathbf{t} = \mathbf{n} \times \mathbf{s} $$

This is all well and good, but the catch is that we may not be given an $\mathbf{a}$ when we load a
model, and our current program doesn't have a way to generate one. If we went ahead and picked an
arbitrary $\mathbf{a}$ to use as an initial vector we may get an $\mathbf{a}$ that is parallel to
$\mathbf{n}$. So a common method is to pick an arbitrary axis and check to see if it's parallel to
$\mathbf{n}$ (which we assume to be of unit length), if it is, just use another axis:

```c++
if (fabs(n.x()) > 0.9)
    a = vec3(0, 1, 0)
else
    a = vec3(1, 0, 0)
```

We then take the cross product to get $\mathbf{s}$ and $\mathbf{t}$

```c++
vec3 s = unit_vector(cross(n, a));
vec3 t = cross(n, s);
```

Note that we don't need to take the unit vector for $\mathbf{t}$. Since $\mathbf{n}$ and
$\mathbf{s}$ are both unit vectors, their cross product $\mathbf{t}$ will be also. Once we have an
ONB of $\mathbf{s}$, $\mathbf{t}$, and $\mathbf{n}$, and we have a random $(x,y,z)$ relative to the
$z$ axis, we can get the vector relative to $\mathbf{n}$ with:

    $$ \mathit{Random vector} = x \mathbf{s} + y \mathbf{t} + z \mathbf{n} $$

If you remember, we used similar math to produce rays from a camera. You can think of that as a
change to the camera’s natural coordinate system.

### The ONB Class
Should we make a class for ONBs, or are utility functions enough? I’m not sure, but let’s make a
class because it won't really be more complicated than utility functions:

```c++ title="Orthonormal basis class"
#ifndef ONB_H
#define ONB_H

#include "rtweekend.h"

class onb {
  public:
    onb() {}

    vec3 operator[](int i) const { return axis[i]; }
    vec3& operator[](int i) { return axis[i]; }

    vec3 u() const { return axis[0]; }
    vec3 v() const { return axis[1]; }
    vec3 w() const { return axis[2]; }

    vec3 local(double a, double b, double c) const {
        return a*u() + b*v() + c*w();
    }

    vec3 local(const vec3& a) const {
        return a.x()*u() + a.y()*v() + a.z()*w();
    }

    void build_from_w(const vec3& w) {
        vec3 unit_w = unit_vector(w);
        vec3 a = (fabs(unit_w.x()) > 0.9) ? vec3(0,1,0) : vec3(1,0,0);
        vec3 v = unit_vector(cross(unit_w, a));
        vec3 u = cross(unit_w, v);
        axis[0] = u;
        axis[1] = v;
        axis[2] = unit_w;
    }

  public:
    vec3 axis[3];
};


#endif
```

We can rewrite our Lambertian material using this to get:

```c++ title="Scatter function, with orthonormal basis" hl_lines="8-10 13"
class lambertian : public material {
  public:
    ...

    bool scatter(
        const ray& r_in, const hit_record& rec, color& alb, ray& scattered, double& pdf
    ) const override {
        onb uvw;
        uvw.build_from_w(rec.normal);
        auto scatter_direction = uvw.local(random_cosine_direction());
        scattered = ray(rec.p, unit_vector(scatter_direction), r_in.time());
        alb = albedo->value(rec.u, rec.v, rec.p);
        pdf = dot(uvw.w(), scattered.direction()) / pi;
        return true;
    }

    ...
};

```

Which produces:

![Cornell box, with orthonormal basis scatter function](https://raytracing.github.io/images/img-3.06-cornell-ortho.jpg)

Let’s get rid of some of that noise.

But first, let's quickly update the `isotropic` material:

```c++ title="Isotropic material, modified for importance sampling" hl_lines="6-7 11 15-18"
class isotropic : public material {
  public:
    isotropic(color c) : albedo(make_shared<solid_color>(c)) {}
    isotropic(shared_ptr<texture> a) : albedo(a) {}

    bool scatter(
        const ray& r_in, const hit_record& rec, color& alb, ray& scattered, double& pdf
    ) const override {
        scattered = ray(rec.p, random_unit_vector(), r_in.time());
        attenuation = albedo->value(rec.u, rec.v, rec.p);
        pdf = 1 / (4 * pi);
        return true;
    }

    double scattering_pdf(const ray& r_in, const hit_record& rec, const ray& scattered)
    const override {
        return 1 / (4 * pi);
    }

  private:
    shared_ptr<texture> albedo;
};
```