use nalgebra::{Point3, Vector3};

struct Ray<'a> {
    origin: &'a Point3<f64>,
    direction: &'a Vector3<f64>,
}

impl<'a, 'b> Ray<'a>
where
    'b: 'a,
{
    fn new(origin: &'b Point3<f64>, direction: &'b Vector3<f64>) -> Self {
        Self { origin, direction }
    }

    fn at(&self, t: f64) -> Point3<f64> {
        self.origin + t * self.direction
    }
}
