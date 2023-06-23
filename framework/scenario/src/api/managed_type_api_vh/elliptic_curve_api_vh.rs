use multiversx_sc::{api::EllipticCurveApiImpl, types::BoxedBytes};

use crate::api::{VMHooksApi, VMHooksBackendType};

impl<const BACKEND_TYPE: VMHooksBackendType> EllipticCurveApiImpl for VMHooksApi<BACKEND_TYPE> {
    fn ec_create_from_name_bytes(&self, _name: &[u8]) -> Self::ManagedBufferHandle {
        todo!()
    }

    fn ec_create_from_name_mb(
        &self,
        _name_handle: Self::ManagedBufferHandle,
    ) -> Self::ManagedBufferHandle {
        todo!()
    }

    fn ec_get_values(
        &self,
        _ec_handle: Self::EllipticCurveHandle,
        _field_order_handle: Self::BigIntHandle,
        _base_point_order_handle: Self::BigIntHandle,
        _eq_constant_handle: Self::BigIntHandle,
        _x_base_point_handle: Self::BigIntHandle,
        _y_base_point_handle: Self::BigIntHandle,
    ) {
        todo!()
    }

    fn ec_curve_length(&self, _ec_handle: Self::EllipticCurveHandle) -> u32 {
        todo!()
    }

    fn ec_private_key_byte_length(&self, _ec_handle: Self::EllipticCurveHandle) -> u32 {
        todo!()
    }

    fn ec_add(
        &self,
        _x_result_handle: Self::BigIntHandle,
        _y_result_handle: Self::BigIntHandle,
        _ec_handle: Self::EllipticCurveHandle,
        _x_first_point: Self::BigIntHandle,
        _y_first_point: Self::BigIntHandle,
        _x_second_point: Self::BigIntHandle,
        _y_second_point: Self::BigIntHandle,
    ) {
        todo!()
    }

    fn ec_double(
        &self,
        _x_result_handle: Self::BigIntHandle,
        _y_result_handle: Self::BigIntHandle,
        _ec_handle: Self::EllipticCurveHandle,
        _x_point_handle: Self::BigIntHandle,
        _y_point_handle: Self::BigIntHandle,
    ) {
        todo!()
    }

    fn ec_is_on_curve(
        &self,
        _ec_handle: Self::EllipticCurveHandle,
        _x_point_handle: Self::BigIntHandle,
        _y_point_handle: Self::BigIntHandle,
    ) -> bool {
        todo!()
    }

    fn ec_scalar_mult_legacy(
        &self,
        _x_result_handle: Self::BigIntHandle,
        _y_result_handle: Self::BigIntHandle,
        _ec_handle: Self::EllipticCurveHandle,
        _x_point_handle: Self::BigIntHandle,
        _y_point_handle: Self::BigIntHandle,
        _data: &[u8],
    ) {
        todo!()
    }

    fn ec_scalar_mult(
        &self,
        _x_result_handle: Self::BigIntHandle,
        _y_result_handle: Self::BigIntHandle,
        _ec_handle: Self::EllipticCurveHandle,
        _x_point_handle: Self::BigIntHandle,
        _y_point_handle: Self::BigIntHandle,
        _data_handle: Self::ManagedBufferHandle,
    ) {
        todo!()
    }

    fn ec_scalar_base_mult_legacy(
        &self,
        _x_result_handle: Self::BigIntHandle,
        _y_result_handle: Self::BigIntHandle,
        _ec_handle: Self::EllipticCurveHandle,
        _data: &[u8],
    ) {
        todo!()
    }

    fn ec_scalar_base_mult(
        &self,
        _x_result_handle: Self::BigIntHandle,
        _y_result_handle: Self::BigIntHandle,
        _ec_handle: Self::EllipticCurveHandle,
        _data_handle: Self::ManagedBufferHandle,
    ) {
        todo!()
    }

    fn ec_marshal_legacy(
        &self,
        _ec_handle: Self::EllipticCurveHandle,
        _x_pair_handle: Self::BigIntHandle,
        _y_pair_handle: Self::BigIntHandle,
    ) -> BoxedBytes {
        todo!()
    }

    fn ec_marshal(
        &self,
        _ec_handle: Self::EllipticCurveHandle,
        _x_pair_handle: Self::BigIntHandle,
        _y_pair_handle: Self::BigIntHandle,
        _result_handle: Self::ManagedBufferHandle,
    ) {
        todo!()
    }

    fn ec_marshal_compressed_legacy(
        &self,
        _ec_handle: Self::EllipticCurveHandle,
        _x_pair_handle: Self::BigIntHandle,
        _y_pair_handle: Self::BigIntHandle,
    ) -> BoxedBytes {
        todo!()
    }

    fn ec_marshal_compressed(
        &self,
        _ec_handle: Self::EllipticCurveHandle,
        _x_pair_handle: Self::BigIntHandle,
        _y_pair_handle: Self::BigIntHandle,
        _result_handle: Self::ManagedBufferHandle,
    ) {
        todo!()
    }

    fn ec_unmarshal_legacy(
        &self,
        _x_result_handle: Self::BigIntHandle,
        _y_result_handle: Self::BigIntHandle,
        _ec_handle: Self::EllipticCurveHandle,
        _data: &[u8],
    ) {
        todo!()
    }

    fn ec_unmarshal(
        &self,
        _x_result_handle: Self::BigIntHandle,
        _y_result_handle: Self::BigIntHandle,
        _ec_handle: Self::EllipticCurveHandle,
        _data_handle: Self::ManagedBufferHandle,
    ) {
        todo!()
    }

    fn ec_unmarshal_compressed_legacy(
        &self,
        _x_result_handle: Self::BigIntHandle,
        _y_result_handle: Self::BigIntHandle,
        _ec_handle: Self::EllipticCurveHandle,
        _data: &[u8],
    ) {
        todo!()
    }

    fn ec_unmarshal_compressed(
        &self,
        _x_result_handle: Self::BigIntHandle,
        _y_result_handle: Self::BigIntHandle,
        _ec_handle: Self::EllipticCurveHandle,
        _data_handle: Self::ManagedBufferHandle,
    ) {
        todo!()
    }

    fn ec_generate_key_legacy(
        &self,
        _x_pub_key_handle: Self::BigIntHandle,
        _y_pub_key_handle: Self::BigIntHandle,
        _ec_handle: Self::EllipticCurveHandle,
    ) -> BoxedBytes {
        todo!()
    }

    fn ec_generate_key(
        &self,
        _x_pub_key_handle: Self::BigIntHandle,
        _y_pub_key_handle: Self::BigIntHandle,
        _ec_handle: Self::EllipticCurveHandle,
        _result_handle: Self::ManagedBufferHandle,
    ) {
        todo!()
    }
}
