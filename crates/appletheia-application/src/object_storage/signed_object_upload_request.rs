use serde::{Deserialize, Serialize};

use super::{
    ObjectUploadExpiresIn, ObjectUploadHeaders, ObjectUploadMethod, SignedObjectUploadUrl,
};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SignedObjectUploadRequest {
    method: ObjectUploadMethod,
    url: SignedObjectUploadUrl,
    expires_in: ObjectUploadExpiresIn,
    headers: ObjectUploadHeaders,
}

impl SignedObjectUploadRequest {
    pub fn new(
        method: ObjectUploadMethod,
        url: SignedObjectUploadUrl,
        expires_in: ObjectUploadExpiresIn,
        headers: ObjectUploadHeaders,
    ) -> Self {
        Self {
            method,
            url,
            expires_in,
            headers,
        }
    }

    pub fn method(&self) -> ObjectUploadMethod {
        self.method
    }

    pub fn url(&self) -> &SignedObjectUploadUrl {
        &self.url
    }

    pub fn expires_in(&self) -> ObjectUploadExpiresIn {
        self.expires_in
    }

    pub fn headers(&self) -> &ObjectUploadHeaders {
        &self.headers
    }
}
