// core
pub(crate) use core::{
  borrow::Borrow,
  convert::TryInto,
  marker::PhantomData,
  mem::{self, MaybeUninit},
  ops::Range,
  slice,
};

// dependencies
pub(crate) use static_assertions::const_assert;

// traits
pub(crate) use crate::{
  allocator::Allocator, continuation::Continuation, is::Is, range_ext::RangeExt,
  serializer::Serializer, to_i64::ToI64, to_u64::ToU64, view::View, x::X,
};

// structs and enums
pub(crate) use crate::{
  anonymous_serializer::AnonymousSerializer,
  done::Done,
  error::Error,
  integer::{I64Serializer, U64Serializer, I64, U64},
  offset::Offset,
  slice_allocator::SliceAllocator,
  state::State,
  usize::Usize,
};

// type aliases
pub(crate) use crate::Result;

#[cfg(feature = "alloc")]
mod alloc {
  // dependencies
  pub(crate) use ::alloc::{collections::TryReserveError, vec::Vec};

  // traits
  pub(crate) use crate::vec_ext::VecExt;

  // structs and enums
  pub(crate) use crate::vec_allocator::VecAllocator;

  #[cfg(test)]
  pub(crate) use ::alloc::vec;
}

#[cfg(feature = "alloc")]
pub(crate) use self::alloc::*;

#[cfg(feature = "std")]
mod std {
  pub(crate) use std::io::{self, Seek, SeekFrom, Write};
}

#[cfg(feature = "std")]
pub(crate) use self::std::*;

#[cfg(test)]
mod test {
  pub(crate) use core::fmt::Debug;

  #[allow(unused)]
  pub(crate) use crate::test::{err, ok};
}

#[cfg(test)]
pub(crate) use self::test::*;
