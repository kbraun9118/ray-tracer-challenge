use std::rc::Rc;

use crate::{shape::Shape, tuple::Tuple};

use super::{Intersection, IntersectionHeap};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Tuple,
    direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Tuple {
        self.origin
    }

    pub fn direction(&self) -> Tuple {
        self.direction
    }

    pub fn position(&self, position: f64) -> Tuple {
        self.origin + (self.direction * position)
    }

    pub fn intersections(&self, shape: Rc<dyn Shape>) -> IntersectionHeap {
        let mut heap = IntersectionHeap::new();
        for i in shape
            .intersects(*self)
            .into_iter()
            .map(|i| Intersection::new(i, shape.clone()))
        {
            heap.push(i);
        }
        heap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(4.0, 5.0, 6.0);
        let r = Ray::new(origin, direction);

        assert_eq!(origin, r.origin());
        assert_eq!(direction, r.direction());
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = Ray::new(Tuple::point(2.0, 3.0, 4.0), Tuple::vector(1.0, 0.0, 0.0));
        assert_eq!(Tuple::point(2.0, 3.0, 4.0), r.position(0.0));
        assert_eq!(Tuple::point(3.0, 3.0, 4.0), r.position(1.0));
        assert_eq!(Tuple::point(1.0, 3.0, 4.0), r.position(-1.0));
        assert_eq!(Tuple::point(4.5, 3.0, 4.0), r.position(2.5));
    }
}
