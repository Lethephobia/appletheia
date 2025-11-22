use super::RequestContext;

pub trait RequestContextAccess {
    fn request_context(&self) -> &RequestContext;
}
