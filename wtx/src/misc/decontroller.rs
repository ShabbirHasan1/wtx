/// Decode/Encode Controller
pub trait DEController {
  /// Decode wrapper
  type DecodeWrapper<'dw>;
  /// Error
  type Error: From<crate::Error>;
  /// Encode wrapper
  type EncodeWrapper<'ew>;
}

impl DEController for () {
  type DecodeWrapper<'dw> = ();
  type Error = crate::Error;
  type EncodeWrapper<'ew> = ();
}
