use core::cell::Cell;
use core::marker;

/// Raw data type
pub trait DataSpec {
    /// Raw register type (`u8`, `u16`, `u32`, ...).
    type Ux: Copy;
    fn reset_value() -> Self::Ux;
    fn to_address() -> u16;
}

/// Trait implemented by readable registers to enable the `read` method.
///
/// Registers marked with `Writable` can be also `modify`'ed.
// pub trait Readable: DataSpec {
//     /// Result from a call to `read` and argument to `modify`.
//     type Reader: From<R<Self>> + core::ops::Deref<Target = R<Self>>;
// }

/// Trait implemented by writeable registers.
///
/// This enables the  `write`, `write_with_zero` and `reset` methods.
///
/// Registers marked with `Readable` can be also `modify`'ed.
// pub trait Writable: DataSpec {
//     /// Writer type argument to `write`, et al.
//     type Writer: From<W<Self>> + core::ops::DerefMut<Target = W<Self>>;
// }

/// Reset value of the register.
///
/// This value is the initial value for the `write` method. It can also be directly written to the
/// register by using the `reset` method.
pub trait Resettable: DataSpec {
    /// Reset value of the register.
    fn reset_value() -> Self::Ux;
}

/// This structure provides volatile access to registers.
#[repr(transparent)]
pub struct Data<D: DataSpec> {
    data: Cell<D::Ux>,
    _marker: marker::PhantomData<D>,
}
