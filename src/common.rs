pub(crate) use core::{borrow::Borrow, marker::PhantomData};

pub(crate) use crate::{
  allocator::Allocator, continuation::Continuation, into_allocator::IntoAllocator,
  serializer::Serializer, x::X,
};

pub(crate) use crate::{done::Done, slice_allocator::SliceAllocator};
