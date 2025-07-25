use crate::framework::response::JsonResult;
use serde::Serialize;
use std::borrow::Cow;
use url::Url;

pub use http::Method;

pub(crate) use spec::EndpointSpec;

pub enum RequestBody<'a> {
    Json(String),
    Raw(Vec<u8>),
    MultiPart(&'a dyn MultipartBody),
}

pub enum MultipartPart {
    Text(String),
    Bytes(Vec<u8>),
}

pub enum ApiType {
    Base,
    Send,
    Bulk,
    Sandbox,
}

/// Helper trait for endpoints that require a multipart body.
///
/// Mainly exists to allow for client-agnostic multipart body implementations, until reqwest has a
/// conversion between blocking::multipart::Form/Part and async_impl::multipart::Form/Part.
pub trait MultipartBody {
    /// Returns a list of parts to be included in a multipart request.
    /// Each part is a tuple of the part name and the part data.
    //
    // Client-agnostic implementation, because of the non-interoperability
    // between reqwest's blocking::multipart::Form/Part and async_impl::multipart::Form/Part.
    // Refactor this when reqwest has some sort of conversion between the two.
    fn parts(&self) -> Vec<(String, MultipartPart)>;
}

pub mod spec {
    use super::*;
    use crate::framework::Environment;
    use crate::framework::response::ApiResponseType;

    /// Represents a specification for an API call that can be built into an HTTP request and sent.
    /// New endpoints should implement this trait.
    ///
    /// If the request succeeds, the call will resolve to a `ResultType`.
    pub trait EndpointSpec {
        /// The JSON response type for this endpoint, if any.
        type ResponseType: ApiResponseType;

        /// The HTTP Method used for this endpoint (e.g. GET, PATCH, DELETE)
        fn method(&self) -> Method;

        /// The relative URL path for this endpoint
        fn path(&self) -> String;

        /// The url-encoded query string associated with this endpoint. Defaults to `None`.
        ///
        /// Implementors should inline this.
        #[inline]
        fn query(&self) -> Option<String> {
            None
        }

        /// The HTTP body associated with this endpoint. If not implemented, defaults to `None`.
        ///
        /// Implementors should inline this.
        #[inline]
        fn body(&self) -> Option<RequestBody> {
            None
        }

        /// Builds and returns a formatted full URL, including query, for the endpoint.
        ///
        /// Implementors should generally not override this.
        fn url(&self, environment: &Environment) -> Url {
            let base = url::Url::from(environment);

            let base = match self.api_type() {
                ApiType::Base => base,
                _ => {
                    let domain = base.domain().unwrap();
                    let path = base.path();

                    let prefix = match self.api_type() {
                        ApiType::Send => "send",
                        ApiType::Bulk => "bulk",
                        ApiType::Sandbox => "sandbox",
                        _ => unreachable!(),
                    };

                    Url::parse(&format!("https://{prefix}.api.{domain}"))
                        .unwrap()
                        .join(path)
                        .unwrap()
                }
            };

            let mut url = base.join(&self.path()).unwrap();
            url.set_query(self.query().as_deref());

            url
        }

        #[inline]
        fn api_type(&self) -> ApiType {
            ApiType::Base
        }

        //noinspection RsConstantConditionIf
        /// If `body` is populated, indicates the body MIME type (defaults to JSON).
        ///
        /// Implementors generally do not need to override this.
        fn content_type(&self) -> Option<Cow<'static, str>> {
            match Self::body(self) {
                Some(RequestBody::Json(_)) => Some(Cow::Borrowed("application/json")),
                Some(RequestBody::Raw(_)) => Some(Cow::Borrowed("application/octet-stream")),
                Some(RequestBody::MultiPart(_)) => Some(Cow::Borrowed("multipart/form-data")),
                None => None,
            }
        }
    }
}
// Auto-implement the public Endpoint trait for EndpointInternal implementors.
impl<T: JsonResult, U: EndpointSpec> Endpoint<T> for U {}

/// An API call that can be built into an HTTP request and sent.
///
/// If the request succeeds, the call will resolve to a `ResultType`.
pub trait Endpoint<ResultType: JsonResult>: EndpointSpec {}

/// A utility function for serializing parameters into a URL query string.
#[inline]
pub fn serialize_query<Q: Serialize>(q: &Q) -> Option<String> {
    serde_urlencoded::to_string(q).ok()
}
