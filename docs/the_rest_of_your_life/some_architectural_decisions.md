We won't write any code in this chapter. We’re at a crossroads and we need to make some
architectural decisions.

The mixture-density approach is an alternative to having more traditional shadow rays. These are
rays that check for an unobstructed path from an intersection point to a given light source. Rays
that intersect an object between a point and a given light source indicate that the intersection
point is in the shadow of that particular light source. The mixture-density approach is something
that I personally prefer, because in addition to lights, you can sample windows or bright cracks
under doors or whatever else you think might be bright -- or important. But you'll still see shadow
rays in most professional path tracers. Typically they'll have a predefined number of shadow rays
(_e.g_ 1, 4, 8, 16) where over the course of rendering, at each place where the path tracing ray
intersects, they'll send these terminal shadow rays to random lights in the scene to determine if
the intersection is lit by that random light. The intersection will either be lit by that light, or
completely in shadow, where more shadow rays lead to a more accurate illumination. After all of the
shadow rays terminate (either at a light or at an occluding surface), the inital path tracing ray
continues on and more shadow rays are sent at the next intersection. You can't tell the shadow rays
what is important, you can only tell them what is emissive, so shadow rays work best on simpler
scenes that don't have overly complicated photon distribution. That said, shadow rays terminate at
the first thing they run into and don't bounce around, so one shadow ray is cheaper than one path
tracing ray, which is the reason that you'll typically see a lot more shadow rays than path tracing
rays (_e.g_ 1, 4, 8, 16). You could choose shadow rays over mixture-density in a more restricted
scene; that’s a personal design preference. Shadow rays tend to be cheaper for a crude result than
mixture-density and is becoming increasingly common in realtime.

There are some other issues with the code.

The PDF construction is hard coded in the `ray_color()` function. We should clean that up.

We've accidentally broken the specular rays (glass and metal), and they are no longer supported.
The math would continue to work out if we just made their scattering function a delta function, but
that would lead to all kinds of floating point disasters. We could either make specular reflection
a special case that skips $f()/p()$, or we could set surface roughness to a very small -- but
nonzero -- value and have almost-mirrors that look perfectly smooth but that don’t generate NaNs. I
don’t have an opinion on which way to do it (I have tried both and they both have their advantages),
but we have smooth metal and glass code anyway, so we'll add perfect specular surfaces that just
skip over explicit $f()/p()$ calculations.

We also lack a real background function infrastructure in case we want to add an environment map or
a more interesting functional background. Some environment maps are HDR (the RGB components are
normalized floats rather than 0–255 bytes). Our output has been HDR all along; we’ve just been
truncating it.

Finally, our renderer is RGB. A more physically based one -- like an automobile manufacturer might
use -- would probably need to use spectral colors and maybe even polarization. For a movie
renderer, most studios still get away with RGB. You can make a hybrid renderer that has both modes,
but that is of course harder. I’m going to stick to RGB for now, but I will touch on this at the end
of the book.