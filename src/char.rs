use crate::common::*;

impl X for char {
  type View = Char;

  fn serialize<A: Allocator, C: Continuation<A>>(
    &self,
    mut serializer: <Self::View as View>::Serializer<A, C>,
  ) -> C {
    serializer.serialize_char(*self)
  }
}

impl X for Char {
  type View = Char;

  fn serialize<A: Allocator, C: Continuation<A>>(
    &self,
    mut serializer: <Self::View as View>::Serializer<A, C>,
  ) -> C {
    serializer.serialize_char(char::from_view(self))
  }
}

#[repr(C)]
#[derive(Debug)]
pub struct Char {
  le_bytes: [u8; 3],
}

impl View for Char {
  type Serializer<A: Allocator, C: Continuation<A>> = CharSerializer<A, C>;

  fn check<'value>(suspect: &'value MaybeUninit<Self>, _buffer: &[u8]) -> Result<&'value Self> {
    // Safe: There are no bitpattern validity requirements for Self
    let value = unsafe { suspect.assume_init_ref() };

    let scalar = value.scalar();

    if char::from_u32(scalar).is_none() {
      return Err(Error::Char { value: scalar });
    }

    Ok(value)
  }
}

impl FromView for char {
  fn from_view(view: &Self::View) -> Self {
    char::from_u32(view.scalar()).unwrap()
  }
}

impl Char {
  fn scalar(&self) -> u32 {
    u32::from_le_bytes([self.le_bytes[0], self.le_bytes[1], self.le_bytes[2], 0])
  }
}

pub struct CharSerializer<A: Allocator, C: Continuation<A>> {
  state: State<A, C>,
}

impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for CharSerializer<A, C> {
  fn new(state: State<A, C>) -> Self {
    Self { state }
  }
}

impl<A: Allocator, C: Continuation<A>> CharSerializer<A, C> {
  fn serialize_char(mut self, value: char) -> C {
    let bytes = (value as u32).to_le_bytes();
    self.state.write(&[bytes[0], bytes[1], bytes[2]]);
    self.state.continuation()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic() {
    ok('\0', &[0, 0, 0]);
    ok(1 as char, &[1, 0, 0]);
    ok('𓃩', &[233, 48, 1]);
    ok(char::MAX, &[255, 255, 16]);
  }

  #[test]
  fn error_range() {
    let buffer: &[u8] = &[0xFF, 0xFF, 0xFF];
    assert_eq!(char::view(buffer).unwrap_err(), Error::Char {
      value: 0xFFFFFF,
    });
  }
}
