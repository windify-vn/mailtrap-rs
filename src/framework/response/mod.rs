mod api_fail;

pub use api_fail::*;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

pub type ApiResponse<ResultType> = Result<ResultType, ApiFailure>;

pub trait JsonResult: DeserializeOwned + Debug {}

pub trait ApiResponseType: Sized {
    fn from_response(bytes: &bytes::Bytes) -> Result<Self, ApiFailure>;
}
impl<T> ApiResponseType for T
where
    T: JsonResult,
{
    fn from_response(bytes: &bytes::Bytes) -> Result<Self, ApiFailure> {
        serde_json::from_slice(bytes).map_err(ApiFailure::Decoding)
    }
}

impl ApiResponseType for String {
    fn from_response(bytes: &bytes::Bytes) -> Result<Self, ApiFailure> {
        let text = String::from_utf8_lossy(bytes);

        Ok(text.into_owned())
    }
}

impl ApiResponseType for Vec<u8> {
    fn from_response(bytes: &bytes::Bytes) -> Result<Self, ApiFailure> {
        Ok(bytes.to_vec())
    }
}

impl ApiResponseType for () {
    fn from_response(_: &bytes::Bytes) -> Result<Self, ApiFailure> {
        Ok(())
    }
}
