In this chapter we won't actually program anything. We'll just be setting up for a big lighting
change in the next chapter. Our ray tracing program from the first two books scatters a ray when it
interacts with a surface or a volume. Ray scattering is the most commonly used model for simulating
light propagation through a scene. This can naturally be modeled probabilistically. There are many
things to consider when modeling the probabilistic scattering of rays.

### Albedo
First, is the light absorbed?

Probability of light being scattered: $A$

Probability of light being absorbed: $1-A$

Where here $A$ stands for _albedo_, which is latin for _whiteness_. Albedo is a precise technical
term in some disciplines, but in all cases it is used to define some form of
_fractional reflectance_. This _fractional reflectance_ (or albedo) will vary with color and (as we
implemented for our glass material) can also vary with incident direction (the direction of the
incoming ray). It can help to stop and remember that when we simulate light propagation, all we're
doing is simulating the movement of photons through a space. If you remember your high school
Physics then you should recall that every photon has a unique energy and wavelength associated by
the Planck constant:

    $$ E = \frac{hc}{\lambda} $$

Each individual photon has a _tiny_ amount of energy, but when you add enough of them up you get all
of the illumination in your rendering. The absorption or scattering of a photon with a surface or a
volume (or really anything that a photon can interact with) is probabilistically determined by the
albedo of the object. Albedo can depend on color because some objects are more likely to absorb
some wavelengths.

In most physically based renderers, we would use a predefined set of specific wavelengths for the
light color rather than RGB. As an example, we would replace our _tristimulus_ RGB renderer with
something that specifically samples at 300nm, 350nm, 400nm, ..., 700nm. We can extend our intuition
by thinking of R, G, and B as specific algebraic mixtures of wavelengths where R is _mostly_ red
wavelengths, G is _mostly_ green wavelengths, and B is _mostly_ blue wavelengths. This is an
approximation of the human visual system which has 3 unique sets of color receptors, called _cones_,
that are each sensitive to different algebraic mixtures of wavelengths, roughly RGB, but are
referred to as long, medium, and short cones (the names are in reference to the wavelengths that
each cone is sensitive to, not the length of the cone). Just as colors can be represented by their
strength in the RGB color space, colors can also be represented by how excited each set of cones is
in the _LMS color space_ (long, medium, short).

### Scattering
If the light does scatter, it will have a directional distribution that we can describe as a PDF
over solid angle.
I will refer to this as its _scattering PDF_: $\operatorname{pScatter}()$.
The scattering PDF will vary with outgoing direction: $\operatorname{pScatter}(\omega_o)$.
The scattering PDF can also vary with _incident direction_:
  $\operatorname{pScatter}(\omega_i, \omega_o)$.
You can see this varying with incident direction when you look at reflections off a road -- they
  become mirror-like as your viewing angle (incident angle) approaches grazing.
The scattering PDF can vary with the wavelength of the light:
  $\operatorname{pScatter}(\omega_i, \omega_o, \lambda)$.
A good example of this is a prism refracting white light into a rainbow.
Lastly, the scattering PDF can also depend on the scattering position:
  $\operatorname{pScatter}(\mathbf{x}, \omega_i, \omega_o, \lambda)$.
The $\mathbf{x}$ is just math notation for the scattering position:
  $\mathbf{x} = (x, y, z)$.
The albedo of an object can also depend on these quantities:
  $A(\mathbf{x}, \omega_i, \omega_o, \lambda)$.

The color of a surface is found by integrating these terms over the unit hemisphere by the incident
direction:

    $$ \operatorname{Color}_o(\mathbf{x}, \omega_o, \lambda) = \int_{\omega_i}
        A(\mathbf{x}, \omega_i, \omega_o, \lambda) \cdot
        \operatorname{pScatter}(\mathbf{x}, \omega_i, \omega_o, \lambda) \cdot
        \operatorname{Color}_i(\mathbf{x}, \omega_i, \lambda) $$

We've added a $\operatorname{Color}_i$ term.
The scattering PDF and the albedo at the surface of an object are acting as filters to the light
  that is shining on that point.
So we need to solve for the light that is shining on that point.
This is a recursive algorithm, and is the reason our `ray_color` function returns the color of the
  current object multiplied by the color of the next ray.

### The Scattering PDF
If we apply the Monte Carlo basic formula we get the following statistical estimate:

    $$ \operatorname{Color}_o(\mathbf{x}, \omega_o, \lambda) \approx \sum
        \frac{A(\, \ldots \,) \cdot
        \operatorname{pScatter}(\, \ldots \,) \cdot
        \operatorname{Color}_i(\, \ldots \,)}
        {p(\mathbf{x}, \omega_i, \omega_o, \lambda)} $$

where $p(\mathbf{x}, \omega_i, \omega_o, \lambda)$ is the PDF of whatever outgoing direction we
randomly generate.

For a Lambertian surface we already implicitly implemented this formula for the special case where
  $pScatter(\, \ldots \,)$ is a cosine density.
The $\operatorname{pScatter}(\, \ldots \,)$ of a Lambertian surface is proportional to
  $\cos(\theta_o)$, where $\theta_o$ is the angle relative to the surface normal.
Let's solve for $C$ once more:

    $$ \operatorname{pScatter}(\mathbf{x}, \omega_i, \omega_o, \lambda) = C \cdot \cos(\theta_o) $$

All two dimensional PDFs need to integrate to one over the whole surface (remember that
  $\operatorname{pScatter}$ is a PDF).
We set $\operatorname{pScatter}(\theta_o < 0) = 0$ so that we don't scatter below the horizon.

    $$ 1 = \int_{0}^{2 \pi} \int_{0}^{\pi / 2} C \cdot cos(\theta) dA $$

To integrate over the hemisphere, remember that in spherical coordinates:

    $$ dA = \sin(\theta) d\theta d\phi $$

So:

    $$ 1 = C \cdot \int_{0}^{2 \pi} \int_{0}^{\pi / 2} cos(\theta) sin(\theta) d\theta d\phi $$
    $$ 1 = C \cdot 2 \pi \frac{1}{2} $$
    $$ 1 = C \cdot \pi $$
    $$ C = \frac{1}{\pi} $$

The integral of $\cos(\theta_o)$ over the hemisphere is $\pi$, so we need to we need to normalize
  by $\frac{1}{\pi}$.
The PDF $\operatorname{pScatter}$ is only dependent on outgoing direction ($\omega_o$), so we'll
  simplify its representation to just $\operatorname{pScatter}(\omega_o)$.
Put all of this together and you get the scattering PDF for a Lambertian surface:

    $$ \operatorname{pScatter}(\omega_o) = \frac{\cos(\theta_o)}{\pi} $$

We'll assume that the $p(\mathbf{x}, \omega_i, \omega_o, \lambda)$ is equal to the scattering PDF:

    $$ p(\omega_o) = \operatorname{pScatter}(\omega_o) = \frac{\cos(\theta_o)}{\pi} $$

The numerator and denominator cancel out, and we get:

    $$ \operatorname{Color}_o(\mathbf{x}, \omega_o, \lambda) \approx \sum
        A(\, \ldots \,) \cdot
        \operatorname{Color}_i(\, \ldots \,) $$

This is exactly what we had in our original `ray_color()` function!

```c++
return attenuation * ray_color(scattered, depth-1, world);
```

The treatment above is slightly non-standard because I want the same math to work for surfaces and
volumes. If you read the literature, you’ll see reflection defined by the
_Bidirectional Reflectance Distribution Function_ (BRDF). It relates pretty simply to our terms:

    $$ BRDF(\omega_i, \omega_o, \lambda) = \frac{A(\mathbf{x}, \omega_i, \omega_o, \lambda) \cdot
        \operatorname{pScatter}(\mathbf{x}, \omega_i, \omega_o, \lambda)}{\cos(\theta_o)} $$

So for a Lambertian surface for example, $BRDF = A / \pi$. Translation between our terms and BRDF is
easy. For participating media (volumes), our albedo is usually called the _scattering albedo_, and
our scattering PDF is usually called the _phase function_.

All that we've done here is outline the PDF for the Lambertian scattering of a material. However,
we'll need to generalize so that we can send extra rays in important directions, such as toward the
lights.
