use crate::misc::{Lease, LeaseMut, SuffixWriterFbvm};

/// Struct used for encoding elements in PostgreSQL.
#[derive(Debug)]
pub struct EncodeWrapper<'buffer, 'tmp> {
  sw: &'tmp mut SuffixWriterFbvm<'buffer>,
}

impl<'buffer, 'tmp> EncodeWrapper<'buffer, 'tmp> {
  #[inline]
  pub(crate) fn new(sw: &'tmp mut SuffixWriterFbvm<'buffer>) -> Self {
    Self { sw }
  }

  /// See [`FilledBufferWriter`].
  #[inline]
  pub fn sw(&mut self) -> &mut SuffixWriterFbvm<'buffer> {
    self.sw
  }
}

impl<'buffer> Lease<SuffixWriterFbvm<'buffer>> for EncodeWrapper<'buffer, '_> {
  #[inline]
  fn lease(&self) -> &SuffixWriterFbvm<'buffer> {
    self.sw
  }
}

impl<'buffer> LeaseMut<SuffixWriterFbvm<'buffer>> for EncodeWrapper<'buffer, '_> {
  #[inline]
  fn lease_mut(&mut self) -> &mut SuffixWriterFbvm<'buffer> {
    self.sw
  }
}
