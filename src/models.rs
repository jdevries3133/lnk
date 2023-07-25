use ammonia::clean;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

pub trait Sanitize {
    /// Return a sanitized copy of `Self`
    fn sanitize(&self) -> Self;
}

impl Sanitize for User {
    fn sanitize(&self) -> Self {
        User {
            id: self.id,
            username: clean(&self.username),
            email: clean(&self.email),
        }
    }
}

impl Sanitize for String {
    fn sanitize(&self) -> Self {
        clean(self)
    }
}

/// Generic container for insert queries RETURNING id
pub struct SqlId {
    pub id: i32,
}
