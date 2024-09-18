/// An API message.
pub trait ApiMessage {
    /// The API key of this API message.
    fn api_key(&self) -> i16;

    /// The version of this API message.
    fn version(&self) -> i16;
}

/// A header.
pub trait Header : ApiMessage {}

/// A request message.
pub trait Request : ApiMessage {}

/// A response message.
pub trait Response : ApiMessage {}

/// A data message.
pub trait Data : ApiMessage {}
