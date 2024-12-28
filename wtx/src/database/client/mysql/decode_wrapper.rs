use crate::{database::client::mysql::Ty, misc::Lease};

/// Struct used for decoding elements in MySQL.
#[derive(Debug, PartialEq)]
pub struct DecodeWrapper<'any> {
  bytes: &'any [u8],
  ty: Ty,
}

impl<'any> DecodeWrapper<'any> {
  pub(crate) fn new(bytes: &'any [u8], ty: Ty) -> Self {
    Self { bytes, ty }
  }

  /// Bytes of a column.
  #[inline]
  pub fn bytes(&self) -> &'any [u8] {
    self.bytes
  }

  /// Type of a column.
  #[inline]
  pub fn ty(&self) -> &Ty {
    &self.ty
  }
}

impl Default for DecodeWrapper<'_> {
  #[inline]
  fn default() -> Self {
    Self { bytes: &[], ty: Ty::Null }
  }
}

impl Lease<[u8]> for DecodeWrapper<'_> {
  #[inline]
  fn lease(&self) -> &[u8] {
    self.bytes
  }
}
