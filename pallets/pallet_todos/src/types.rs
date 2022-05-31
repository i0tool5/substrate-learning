use frame_support::pallet_prelude::*;

use crate::*;

/// ToDo type is a user's ToDo body
pub(crate) type ToDo<T> = BoundedVec<u8, <T as Config>::MaxToDoLength>;
