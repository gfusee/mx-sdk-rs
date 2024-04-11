use crate::api::{CallTypeApi, HandleTypeInfo, StorageMapperApi, VMApi};

use super::UncallableApi;

impl<'a> CallTypeApi<'a> for UncallableApi {}

impl<'a> StorageMapperApi<'a> for UncallableApi {}

impl PartialEq for UncallableApi {
    fn eq(&self, _: &Self) -> bool {
        unreachable!()
    }
}

impl Eq for UncallableApi {}

impl<'a> VMApi<'a> for UncallableApi {}

impl HandleTypeInfo for UncallableApi {
    type ManagedBufferHandle = i32;

    type BigIntHandle = i32;

    type BigFloatHandle = i32;

    type EllipticCurveHandle = i32;

    type ManagedMapHandle = i32;
}
