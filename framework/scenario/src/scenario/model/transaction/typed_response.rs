use super::{TxResponse, TxResponseStatus};
use multiversx_sc::codec::{PanicErrorHandler, TopDecodeMulti};

pub struct TypedResponse<T>
where
    T: TopDecodeMulti,
{
    pub result: Result<T, TxResponseStatus>,
    pub new_issued_token_identifier: Option<String>,
    pub gas: u64,
    pub refund: u64,
}

impl<T> TypedResponse<T>
where
    T: TopDecodeMulti,
{
    pub fn from_raw(raw_response: &TxResponse) -> Self {
        let result: Result<T, TxResponseStatus> = if raw_response.tx_error.is_success() {
            let mut result_raw = raw_response.out.clone();
            let Ok(decoded) = T::multi_decode_or_handle_err(&mut result_raw, PanicErrorHandler);
            Ok(decoded)
        } else {
            Err(raw_response.tx_error.clone())
        };

        TypedResponse {
            result,
            new_issued_token_identifier: raw_response.new_issued_token_identifier.clone(),
            gas: raw_response.gas,
            refund: raw_response.refund,
        }
    }
}
