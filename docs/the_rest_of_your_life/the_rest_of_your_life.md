The purpose of this book was to walk through all of the little details (dotting all the i's and
crossing all of the t's) necessary when organizing a physically based renderer’s sampling approach.
You should now be able to take all of this detail and explore a lot of different potential paths.

If you want to explore Monte Carlo methods, look into bidirectional and path spaced approaches such
as Metropolis. Your probability space won't be over solid angle, but will instead be over path
space, where a path is a multidimensional point in a high-dimensional space. Don’t let that scare
you -- if you can describe an object with an array of numbers, mathematicians call it a point in the
space of all possible arrays of such points. That’s not just for show. Once you get a clean
abstraction like that, your code can get clean too. Clean abstractions are what programming is all
about!

If you want to do movie renderers, look at the papers out of studios and Solid Angle. They are
surprisingly open about their craft.

If you want to do high-performance ray tracing, look first at papers from Intel and NVIDIA. They are
also surprisingly open.

If you want to do hard-core physically based renderers, convert your renderer from RGB to spectral.
I am a big fan of each ray having a random wavelength and almost all the RGBs in your program
turning into floats. It sounds inefficient, but it isn’t!

Regardless of what direction you take, add a glossy BRDF model. There are many to choose from, and
each has its advantages.

Have fun!

[Peter Shirley][]<br>
Salt Lake City, March, 2016