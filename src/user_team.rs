use crate::notify::Notify;
use crate::user::User;
use anyhow::Result;

// Can have users or other teams?  KISS, just users for the minute
pub struct UserTeam {
    name: String,
    members: Vec<User>,
}

impl UserTeam {
    pub fn new(name: String, members: Vec<User>) -> Self {
        UserTeam { name, members }
    }

    pub fn push(&mut self, u: User) {
        self.members.push(u)
    }

    pub fn append(&mut self, users: &mut Vec<User>) {
        self.members.append(users)
    }

    pub fn oncall(&self) -> Result<Vec<User>> {
        // TODO: This needs to be smarter
        let users = self.members[0..=0].to_vec();
        Ok(users)
    }
}

impl Notify for UserTeam {
    fn notify(&self) -> Result<Vec<User>> {
        let users = self.oncall();
        users
    }
}
