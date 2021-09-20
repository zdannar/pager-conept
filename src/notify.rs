use crate::user::User;
use anyhow::Result;

pub trait Notify {
    fn notify(&self) -> Result<Vec<User>>;
}
