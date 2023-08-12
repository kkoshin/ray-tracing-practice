In Ray Tracing in One Weekend, you built a simple brute force path tracer. In this installment we’ll
add textures, volumes (like fog), rectangles, instances, lights, and support for lots of objects
using a BVH. When done, you’ll have a “real” ray tracer.

A heuristic in ray tracing that many people--including me--believe, is that most optimizations
complicate the code without delivering much speedup. What I will do in this mini-book is go with the
simplest approach in each design decision I make. Check https://in1weekend.blogspot.com/ for
readings and references to a more sophisticated approach. However, I strongly encourage you to do no
premature optimization; if it doesn’t show up high in the execution time profile, it doesn’t need
optimization until all the features are supported!

The two hardest parts of this book are the BVH and the Perlin textures. This is why the title
suggests you take a week rather than a weekend for this endeavor. But you can save those for last if
you want a weekend project. Order is not very important for the concepts presented in this book, and
without BVH and Perlin texture you will still get a Cornell Box!

Thanks to everyone who lent a hand on this project. You can find them in the acknowledgments section
at the end of this book.