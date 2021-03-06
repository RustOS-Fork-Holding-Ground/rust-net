//! See http://en.wikipedia.org/wiki/Cyclic_order

#![allow(unstable)]

#[cfg(test)]
#[macro_use]
extern crate quickcheck_macros;
#[cfg(test)]
extern crate quickcheck;


pub mod linked_list;

mod impls;

#[derive(PartialEq, PartialOrd, Eq, Ord,
         Copy, Clone, Hash, Show)]
pub enum CyclicOrdering {
  Clockwise,
  CounterClockwise,
  Degenerate,      // a b and c are not all distinct
}

/// Axioms (from Wikipedia):
///
///  - Cyclicity:    If [a, b, c] then [b, c, a]
///  - Asymmetry:    If [a, b, c] then not [c, b, a]
///  - Transitivity: If [a, b, c] and [a, c, d] then [a, b, d]
pub trait PartialCyclicOrd
{
  fn is_clockwise(&self, them: &Self, other: &Self) -> bool;
}

pub mod partial_axioms {
  //! implementors can test their impls easily with these

  use super::*;

  pub fn cyclicity<T>(a: &T, b: &T, c: &T) -> bool where T: PartialCyclicOrd {
    a.is_clockwise(b, c) == b.is_clockwise(c, a)
  }

  pub fn antisymmetry<T>(a: &T, b: &T, c: &T) -> bool where T: PartialCyclicOrd {
    !( a.is_clockwise(b, c) && c.is_clockwise(b, a) )
  }

  pub fn transitivity<T>(a: &T, b: &T, c: &T, d: &T) -> bool where T: PartialCyclicOrd {
    match (a.is_clockwise(b, c), a.is_clockwise(c, d), a.is_clockwise(b, d)) {
      (true,        true,        trans) => trans,
      _                                 => true,
    }
  }
}


/// Axioms (from Wikipedia):
///
///  - From PartialCyclicOrder:
///     - Cyclicity:    If [a, b, c] then [b, c, a]
///     - Asymmetry:    If [a, b, c] then not [c, b, a]
///     - Transitivity: If [a, b, c] and [a, c, d] then [a, b, d]
///
///  - Totality:     If a, b, and c are distinct, then either [a, b, c] or [c, b, a]
pub trait CyclicOrd: PartialEq + PartialCyclicOrd {
  fn cyclic_cmp(&self, them: &Self, other: &Self) -> CyclicOrdering;
}

pub mod total_axioms {
  //! implementors can test their impls easily with these

  use super::*;
  use super::CyclicOrdering::*;

  pub fn cyclicity<T>(a: &T, b: &T, c: &T) -> bool where T: CyclicOrd {
    a.cyclic_cmp(b, c) == b.cyclic_cmp(c, a)
  }

  pub fn antisymmetry<T>(a: &T, b: &T, c: &T) -> bool where T: CyclicOrd {
    match (a.cyclic_cmp(b, c), c.cyclic_cmp(b, a)) {
      (Clockwise,        Clockwise)        => false,
      (CounterClockwise, CounterClockwise) => false,
      _                                    => true,
    }
  }

  pub fn transitivity<T>(a: &T, b: &T, c: &T, d: &T) -> bool where T: CyclicOrd {
    match (a.cyclic_cmp(b, c), a.cyclic_cmp(c, d), a.cyclic_cmp(b, d)) {
      (Clockwise,        Clockwise,        trans) => trans == Clockwise,
      (CounterClockwise, CounterClockwise, trans) => trans == CounterClockwise,
      _                                           => true,
    }
  }

  pub fn totality<T>(a: &T, b: &T, c: &T) -> bool where T: CyclicOrd {
    match (a.cyclic_cmp(b, c), c.cyclic_cmp(b, a)) {
      (Clockwise,        CounterClockwise) => true,
      (CounterClockwise, Clockwise)        => true,
      (Degenerate,       Degenerate)       => a == b || b == c || c == a,
      _                                    => false,
    }
  }

  pub fn super_trait_cohesion<T>(a: &T, b: &T, c: &T) -> bool where T: CyclicOrd {
    match (a.cyclic_cmp(b, c), a.is_clockwise(b, c)) {
      (Clockwise, true)  => true,
      (_,         false) => true,
      _                  => false,
    }
  }
}
