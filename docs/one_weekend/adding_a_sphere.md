Let’s add a single object to our ray tracer. People often use spheres in ray tracers because
calculating whether a ray hits a sphere is relatively simple.

### Ray-Sphere Intersection
The equation for a sphere of radius $r$ that is centered at the origin is an important mathematical
equation:

    $$ x^2 + y^2 + z^2 = r^2 $$

You can also think of this as saying that if a given point $(x,y,z)$ is on
the sphere, then $x^2 + y^2 + z^2 = r^2$. If a given point $(x,y,z)$ is _inside_ the sphere, then
$x^2 + y^2 + z^2 < r^2$, and if a given point $(x,y,z)$ is _outside_ the sphere, then
$x^2 + y^2 + z^2 > r^2$.

If we want to allow the sphere center to be at an arbitrary point $(C_x, C_y, C_z)$, then the
equation becomes a lot less nice:

  $$ (x - C_x)^2 + (y - C_y)^2 + (z - C_z)^2 = r^2 $$

In graphics, you almost always want your formulas to be in terms of vectors so that all the
$x$/$y$/$z$ stuff can be simply represented using a `vec3` class. You might note that the vector
from center $\mathbf{C} = (C_x, C_y, C_z)$ to point $\mathbf{P} = (x,y,z)$ is
$(\mathbf{P} - \mathbf{C})$. If we use the definition of the dot product:

  $$ (\mathbf{P} - \mathbf{C}) \cdot (\mathbf{P} - \mathbf{C})
     = (x - C_x)^2 + (y - C_y)^2 + (z - C_z)^2
  $$

Then we can rewrite the equation of the sphere in vector form as:

  $$ (\mathbf{P} - \mathbf{C}) \cdot (\mathbf{P} - \mathbf{C}) = r^2 $$

We can read this as “any point $\mathbf{P}$ that satisfies this equation is on the sphere”. We want
to know if our ray $\mathbf{P}(t) = \mathbf{A} + t\mathbf{b}$ ever hits the sphere anywhere. If it
does hit the sphere, there is some $t$ for which $\mathbf{P}(t)$ satisfies the sphere equation. So
we are looking for any $t$ where this is true:

  $$ (\mathbf{P}(t) - \mathbf{C}) \cdot (\mathbf{P}(t) - \mathbf{C}) = r^2 $$

which can be found by replacing $\mathbf{P}(t)$ with its expanded form:

  $$ ((\mathbf{A} + t \mathbf{b}) - \mathbf{C})
      \cdot ((\mathbf{A} + t \mathbf{b}) - \mathbf{C}) = r^2 $$

We have three vectors on the left dotted by three vectors on the right. If we solved for the full
dot product we would get nine vectors. You can definitely go through and write everything out, but
we don't need to work that hard. If you remember, we want to solve for $t$, so we'll separate the
terms based on whether there is a $t$ or not:

  $$ (t \mathbf{b} + (\mathbf{A} - \mathbf{C}))
      \cdot (t \mathbf{b} + (\mathbf{A} - \mathbf{C})) = r^2 $$

And now we follow the rules of vector algebra to distribute the dot product:

  $$ t^2 \mathbf{b} \cdot \mathbf{b}
     + 2t \mathbf{b} \cdot (\mathbf{A}-\mathbf{C})
     + (\mathbf{A}-\mathbf{C}) \cdot (\mathbf{A}-\mathbf{C}) = r^2
  $$

Move the square of the radius over to the left hand side:

  $$ t^2 \mathbf{b} \cdot \mathbf{b}
     + 2t \mathbf{b} \cdot (\mathbf{A}-\mathbf{C})
     + (\mathbf{A}-\mathbf{C}) \cdot (\mathbf{A}-\mathbf{C}) - r^2 = 0
  $$

It's hard to make out what exactly this equation is, but the vectors and $r$ in that equation are
all constant and known. Furthermore, the only vectors that we have are reduced to scalars by dot
product. The only unknown is $t$, and we have a $t^2$, which means that this equation is quadratic.
You can solve for a quadratic equation by using the quadratic formula:

  $$ \frac{-b \pm \sqrt{b^2 - 4ac}}{2a} $$

Where for ray-sphere intersection the $a$/$b$/$c$ values are:

  $$ a = \mathbf{b} \cdot \mathbf{b} $$
  $$ b = 2 \mathbf{b} \cdot (\mathbf{A}-\mathbf{C}) $$
  $$ c = (\mathbf{A}-\mathbf{C}) \cdot (\mathbf{A}-\mathbf{C}) - r^2 $$

Using all of the above you can solve for $t$, but there is a square root part that can be either
positive (meaning two real solutions), negative (meaning no real solutions), or zero (meaning one
real solution). In graphics, the algebra almost always relates very directly to the geometry. What
we have is:

![Ray-sphere intersection results](https://raytracing.github.io/images/fig-1.05-ray-sphere.jpg)

### Creating Our First Raytraced Image
If we take that math and hard-code it into our program, we can test our code by placing a small
sphere at -1 on the z-axis and then coloring red any pixel that intersects it.

```c++ hl_lines="1-8 11-12" title="Rendering a red sphere"
bool hit_sphere(const point3& center, double radius, const ray& r) {
    vec3 oc = r.origin() - center;
    auto a = dot(r.direction(), r.direction());
    auto b = 2.0 * dot(oc, r.direction());
    auto c = dot(oc, oc) - radius*radius;
    auto discriminant = b*b - 4*a*c;
    return (discriminant >= 0);
}


color ray_color(const ray& r) {
    if (hit_sphere(point3(0,0,-1), 0.5, r))
        return color(1, 0, 0);

    vec3 unit_direction = unit_vector(r.direction());
    auto a = 0.5*(unit_direction.y() + 1.0);
    return (1.0-a)*color(1.0, 1.0, 1.0) + a*color(0.5, 0.7, 1.0);
}
```

What we get is this:

![A simple red sphere](https://raytracing.github.io/images/img-1.03-red-sphere.png)

Now this lacks all sorts of things -- like shading, reflection rays, and more than one object --
but we are closer to halfway done than we are to our start! One thing to be aware of is that we
are testing to see if a ray intersects with the sphere by solving the quadratic equation and seeing
if a solution exists, but solutions with negative values of $t$ work just fine. If you change your
sphere center to $z = +1$ you will get exactly the same picture because this solution doesn't
distinguish between objects _in front of the camera_ and objects _behind the camera_. This is not a
feature! We’ll fix those issues next.