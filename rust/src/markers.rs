pub trait ApiMessage {
    fn api_key(&self) -> i16;
    fn version(&self) -> i16;
}

pub trait Header : ApiMessage {}

pub trait Request : ApiMessage {}

pub trait Response : ApiMessage {}

pub trait Data : ApiMessage {}
