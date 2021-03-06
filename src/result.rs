use crate::common::*;

impl<T: X, E: X> X for core::result::Result<T, E> {
  type View = self::Result<T::View, E::View>;

  fn serialize<A: Allocator, C: Continuation<A>>(
    &self,
    mut serializer: <Self::View as View>::Serializer<A, C>,
  ) -> C {
    match self {
      Ok(t) => serializer.ok(t),
      Err(e) => serializer.err(e),
    }
  }
}

const OK_DISCRIMINANT: u8 = 0;
const ERR_DISCRIMINANT: u8 = 1;

#[repr(u8)]
#[derive(Debug)]
pub enum Result<T: View, E: View> {
  Ok(T) = OK_DISCRIMINANT,
  Err(E) = ERR_DISCRIMINANT,
}

impl<T: View, E: View> View for self::Result<T, E> {
  type Serializer<A: Allocator, C: Continuation<A>> = ResultSerializer<A, C, T, E>;

  fn check<'value>(
    suspect: &'value MaybeUninit<Self>,
    buffer: &[u8],
  ) -> crate::Result<&'value Self> {
    let pointer = suspect.as_ptr() as *const u8;
    let payload = unsafe { pointer.add(1) };

    let discriminant = unsafe { *pointer };

    match discriminant {
      OK_DISCRIMINANT => {
        let payload = payload as *const MaybeUninit<T>;
        View::check(unsafe { &*payload }, buffer)?;
        Ok(unsafe { suspect.assume_init_ref() })
      },
      ERR_DISCRIMINANT => {
        let payload = payload as *const MaybeUninit<E>;
        View::check(unsafe { &*payload }, buffer)?;
        Ok(unsafe { suspect.assume_init_ref() })
      },
      value => Err(Error::Discriminant {
        maximum: ERR_DISCRIMINANT,
        ty: "Result",
        value,
      }),
    }
  }
}

pub struct ResultSerializer<A: Allocator, C: Continuation<A>, T: View, E: View> {
  state: State<A, C>,
  t:     PhantomData<T>,
  e:     PhantomData<E>,
}

impl<A: Allocator, C: Continuation<A>, T: View, E: View> ResultSerializer<A, C, T, E> {
  fn ok<N: X<View = T>>(mut self, ok: &N) -> C {
    self.state.write(&[OK_DISCRIMINANT]);
    <N::View as View>::Serializer::new(self.state.identity::<PaddingSerializer<A, C, T, E>>())
      .serialize(ok)
      .serialize_padding()
  }

  fn err<N: X<View = E>>(mut self, err: &N) -> C {
    self.state.write(&[ERR_DISCRIMINANT]);
    <N::View as View>::Serializer::new(self.state.identity::<PaddingSerializer<A, C, E, T>>())
      .serialize(err)
      .serialize_padding()
  }
}

impl<A: Allocator, C: Continuation<A>, T: View, E: View> Serializer<A, C>
  for ResultSerializer<A, C, T, E>
{
  fn new(state: State<A, C>) -> Self {
    Self {
      t: PhantomData,
      e: PhantomData,
      state,
    }
  }
}

impl<T: FromView, E: FromView> FromView for core::result::Result<T, E> {
  fn from_view(view: &Self::View) -> Self {
    match view {
      self::Result::Ok(t) => core::result::Result::Ok(T::from_view(t)),
      self::Result::Err(e) => core::result::Result::Err(E::from_view(e)),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic() {
    ok(core::result::Result::<char, u32>::Ok('a'), &[
      0, 97, 0, 0, 0,
    ]);
    ok(core::result::Result::<char, u32>::Err(67305985), &[
      1, 1, 2, 3, 4,
    ]);
    ok(core::result::Result::<u32, char>::Ok(67305985), &[
      0, 1, 2, 3, 4,
    ]);
    ok(core::result::Result::<u32, char>::Err('a'), &[
      1, 97, 0, 0, 0,
    ]);
  }

  #[test]
  fn invalid_discriminant() {
    assert_eq!(
      core::result::Result::<u8, u8>::view(&[2, 0]).unwrap_err(),
      Error::Discriminant {
        value:   2,
        maximum: ERR_DISCRIMINANT,
        ty:      "Result",
      }
    );
  }

  #[test]
  fn invalid_ok() {
    assert_eq!(
      core::result::Result::<char, u8>::view(&[OK_DISCRIMINANT, 0xFF, 0xFF, 0xFF]).unwrap_err(),
      Error::Char { value: 0xFFFFFF }
    );
  }

  #[test]
  fn invalid_err() {
    assert_eq!(
      core::result::Result::<u8, char>::view(&[ERR_DISCRIMINANT, 0xFF, 0xFF, 0xFF]).unwrap_err(),
      Error::Char { value: 0xFFFFFF }
    );
  }
}
