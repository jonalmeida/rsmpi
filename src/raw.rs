//! Bridge between rust types and raw values

use ffi::{MPI_Comm, MPI_Group, MPI_Datatype, MPI_Request, MPI_Op};

/// Rust C bridge traits
pub mod traits {
    pub use super::{AsRaw, AsRawMut, RawCommunicator, RawGroup, RawDatatype, RawRequest,
        RawOperation};
}

/// A rust type than can identify as a raw value understood by the MPI C API.
pub trait AsRaw {
    /// The raw MPI C API type
    type Raw;
    /// The raw value
    unsafe fn as_raw(&self) -> Self::Raw;
}

impl<'a, T: 'a + AsRaw> AsRaw for &'a T {
    type Raw = <T as AsRaw>::Raw;
    unsafe fn as_raw(&self) -> Self::Raw { (*self).as_raw() }
}

/// A rust type than can provide a mutable pointer to a raw value understood by the MPI C API.
pub trait AsRawMut: AsRaw {
    /// A mutable pointer to the raw value
    unsafe fn as_raw_mut(&mut self) -> *mut <Self as AsRaw>::Raw;
}

/// A type that can identify as an `MPI_Comm`
pub trait RawCommunicator: AsRaw<Raw = MPI_Comm> { }
impl<T> RawCommunicator for T where T: AsRaw<Raw = MPI_Comm> { }

/// A type that can identify as an `MPI_Group`
pub trait RawGroup: AsRaw<Raw = MPI_Group> { }
impl<T> RawGroup for T where T: AsRaw<Raw = MPI_Group> { }

/// A type that can identify as an `MPI_Datatype`
pub trait RawDatatype: AsRaw<Raw = MPI_Datatype> { }
impl<T> RawDatatype for T where T: AsRaw<Raw = MPI_Datatype> { }

/// A type that can identify as an `MPI_Request`
pub trait RawRequest: AsRaw<Raw = MPI_Request> + AsRawMut { }
impl<T> RawRequest for T where T: AsRaw<Raw = MPI_Request> + AsRawMut { }

/// A type that can identify as an `MPI_Op`
pub trait RawOperation: AsRaw<Raw = MPI_Op> { }
impl<T> RawOperation for T where T: AsRaw<Raw = MPI_Op> { }
