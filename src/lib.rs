#![forbid(unsafe_code)]
// #![warn(missing_docs)]

use core::{any::type_name, fmt};

use serde::{Deserialize, Serialize};
/// ```
/// use redact::Secret;
/// let encryption_key: Secret<&str> = Secret::new("hello world");
/// assert_eq!("[REDACTED &str]", format!("{encryption_key:?}"))
/// ```
/// ```
/// use redact::Secret;
/// use redact::ExposeSecret;
/// let encryption_key: Secret<&str> = Secret::new("hello world");
/// assert_eq!("hello world", *encryption_key.expose_secret())
/// ```
/// ```
/// use redact::Secret;
/// let encryption_key: Secret<&str, false> = Secret::new("hello world");
/// assert_eq!("[REDACTED]", format!("{encryption_key:?}"))
/// ```
#[derive(Serialize, Deserialize, Default, Copy, Clone, Eq, PartialEq)]
#[serde(transparent)]
pub struct Secret<T, const DISPLAY_TYPE_NAME: bool = true>(T);

impl<T, const DISPLAY_TYPE_NAME: bool> Secret<T, DISPLAY_TYPE_NAME> {
    #[inline]
    pub fn new(secret: T) -> Self {
        Self(secret)
    }
}

impl<T, const DISPLAY_TYPE_NAME: bool> From<T> for Secret<T, DISPLAY_TYPE_NAME> {
    #[inline]
    fn from(secret: T) -> Self {
        Self::new(secret)
    }
}

impl<T> fmt::Debug for Secret<T, false> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED]")
    }
}

impl<T> fmt::Debug for Secret<T, true> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED {}]", type_name::<T>())
    }
}

pub trait ExposeSecret: Sealed {
    type Secret;
    fn expose_secret(&self) -> &Self::Secret;
}

use private::Sealed;
mod private {
    pub trait Sealed {}
    impl<T, const DISPLAY_TYPE_NAME: bool> Sealed for super::Secret<T, DISPLAY_TYPE_NAME> {}
}

impl<T, const DISPLAY_TYPE_NAME: bool> ExposeSecret for Secret<T, DISPLAY_TYPE_NAME> {
    type Secret = T;
    #[inline]
    fn expose_secret(&self) -> &Self::Secret {
        &self.0
    }
}
