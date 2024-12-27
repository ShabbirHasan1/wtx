use crate::database::{
  client::postgres::{DecodeValue, EncodeValue, Postgres, Ty},
  Decode, Encode, Typed,
};
use uuid::Uuid;

impl<'de, E> Decode<'de, Postgres<E>> for Uuid
where
  E: From<crate::Error>,
{
  #[inline]
  fn decode(input: &DecodeValue<'de>) -> Result<Self, E> {
    let elem = Uuid::from_slice(input.bytes()).map_err(Into::into)?;
    Ok(elem)
  }
}

impl<E> Encode<Postgres<E>> for Uuid
where
  E: From<crate::Error>,
{
  #[inline]
  fn encode(&self, ev: &mut EncodeValue<'_, '_>) -> Result<(), E> {
    ev.sw().extend_from_slice(self.as_bytes()).map_err(Into::into)?;
    Ok(())
  }
}

impl<E> Typed<Postgres<E>> for Uuid
where
  E: From<crate::Error>,
{
  const TY: Ty = Ty::Uuid;
}

test!(uuid, Uuid, Uuid::max());
