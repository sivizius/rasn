use alloc::vec::Vec;
use core::ops;

/// A global unique identifier that identifies an concept, such as a
/// organisation, or encoding rules.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ObjectIdentifier(Vec<u32>);

impl ObjectIdentifier {
    /// Creates a new object identifier from `vec`.
    ///
    /// # Panics
    /// If `vec` contains less than two components or the first component is
    /// greater than 1.
    pub fn new(vec: Vec<u32>) -> Option<Self> {
        if vec.len() >= 2 && vec[0] < 2 {
            Some(Self(vec))
        } else {
            None
        }
    }
}

impl AsRef<[u32]> for ObjectIdentifier {
    fn as_ref(&self) -> &[u32] {
        self.0.as_ref()
    }
}

impl ops::Deref for ObjectIdentifier {
    type Target = Vec<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for ObjectIdentifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
