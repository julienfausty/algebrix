/// A library for translating mathematical concepts into computable structures.

/// A module for sets
mod set;
pub use set::Set;

/// A module for defining topologies on sets
mod topology;
pub use topology::Topology;

/// A module defining maps between sets
mod map;
pub use map::Map;

/// A module including structs for working with the set of real numbers
mod reals;
pub use reals::Reals;
pub use reals::Interval;
