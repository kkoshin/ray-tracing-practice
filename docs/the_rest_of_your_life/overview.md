In _Ray Tracing in One Weekend_ and _Ray Tracing: the Next Week_, you built a “real” ray tracer.

If you are motivated, you can take the source and information contained in those books to implement
any visual effect you want. The source provides a meaningful and robust foundation upon which to
build out a raytracer for a small hobby project. Most of the visual effects found in commercial ray
tracers rely on the techniques described in these first two books. However, your capacity to add
increasingly complicated visual effects like subsurface scattering or nested dielectrics will be
severely limited by a missing mathematical foundation. In this volume, I assume that you are either
a highly interested student, or are someone who is pursuing a career related to ray tracing. We will
be diving into the math of creating a very serious ray tracer. When you are done, you should be
well equipped to use and modify the various commercial ray tracers found in many popular domains,
such as the movie, television, product design, and architecture industries.

There are many many things I do not cover in this short volume. For example, there are many ways of
writing Monte Carlo rendering programs--I dive into only one of them. I don’t cover shadow rays
(deciding instead to make rays more likely to go toward lights), nor do I cover bidirectional
methods, Metropolis methods, or photon mapping. You'll find many of these techniques in the
so-called "serious ray tracers", but they are not covered here because it is more important to cover
the concepts, math, and terms of the field. I think of this book as a deep exposure that should be
your first of many, and it will equip you with some of the concepts, math, and terms that you'll
need in order to study these and other interesting techniques.

I hope that you find the math as fascinating as I do.

As before, https://in1weekend.blogspot.com/ will have further readings and references.

Thanks to everyone who lent a hand on this project. You can find them in the acknowledgments section
at the end of this book.