use crate::Set;

use num::traits::real::Real;

#[derive(PartialEq, Eq, Debug)]
pub enum SizeT {
    Finite(usize),
    Infinity,
}

/// A structure representing an interval on the real line
pub struct Interval<RealType: Real> {
    begin: RealType,
    end: RealType,
}

impl<RealType: Real> Interval<RealType> {
    pub fn new(start: RealType, stop: RealType) -> Interval<RealType> {
        if start > stop {
            panic!("Starting value for interval is larger than stopping value")
        }
        if start == stop {
            panic!("Starting value for interval is equal stopping value")
        }
        Interval {
            begin: start,
            end: stop,
        }
    }

    pub fn start(&self) -> RealType {
        self.begin
    }

    pub fn stop(&self) -> RealType {
        self.end
    }
}

impl<RealType: Real> Set<RealType, RealType, SizeT> for Interval<RealType> {
    type SubsetType = Interval<RealType>;

    fn get_cardinality(&self) -> SizeT {
        SizeT::Infinity
    }

    /// Clamp anything outside the interval
    fn get_element(&self, key: RealType) -> RealType {
        if key < self.begin {
            return self.begin;
        }
        if key > self.end {
            return self.end;
        }
        key
    }

    /// Clamp anything outside the interval to only return true subsets
    fn get_subset(&self, edges: &[RealType]) -> Interval<RealType> {
        if !(edges.len() == 2) {
            panic!("Edges passed to get interval don't have exactly 2 values")
        }
        Interval::new(self.get_element(edges[0]), self.get_element(edges[1]))
    }
}

/// A structure representing the real line
pub struct Reals {}

impl Reals {
    pub fn new() -> Reals {
        Reals {}
    }
}

impl<RealType: Real> Set<RealType, RealType, SizeT> for Reals {
    type SubsetType = Interval<RealType>;

    fn get_cardinality(&self) -> SizeT {
        SizeT::Infinity
    }

    fn get_element(&self, key: RealType) -> RealType {
        key
    }

    fn get_subset(&self, edges: &[RealType]) -> Interval<RealType> {
        if !(edges.len() == 2) {
            panic!("Edges passed to get interval don't have exactly 2 values")
        }
        Interval::new(edges[0], edges[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_construction() {
        let interval = Interval::new(0.0, 1.0);
        assert_eq!(interval.start(), 0.0);
        assert_eq!(interval.stop(), 1.0);
        let interval = Interval::new(-1.0, 2.0);
        assert_eq!(interval.start(), -1.0);
        assert_eq!(interval.stop(), 2.0);
        assert_eq!(interval.get_cardinality(), SizeT::Infinity);
        assert_eq!(interval.get_element(-2.0), interval.start());
        assert_eq!(interval.get_element(10.0), interval.stop());
        assert_eq!(interval.get_element(0.0001), 0.0001);
    }

    #[test]
    fn test_real_construction() {
        let reals = Reals::new();
        assert_eq!(reals.get_element(0.0), 0.0);
        assert_eq!(reals.get_element(1.0), 1.0);
        assert_eq!(reals.get_element(-1.0), -1.0);
    }
}
