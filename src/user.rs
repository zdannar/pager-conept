use crate::notify::Notify;
use anyhow::Result;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, PartialEq, Eq)]
pub enum UserStatus {
    Active,
    Inactive, // consider Argument for kind of inactive? "Vacation", "Deactivated", etc?
}

#[derive(Clone, PartialEq, Eq)]
pub struct User {
    username: String,
    status: UserStatus,
}

impl User {
    pub fn new(username: String, status: UserStatus) -> Self {
        User { username, status }
    }
}
impl Hash for User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.username.hash(state);
    }
}

impl Notify for User {
    fn notify(&self) -> Result<Vec<User>> {
        Ok(vec![self.clone()])
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.username)
    }
}
