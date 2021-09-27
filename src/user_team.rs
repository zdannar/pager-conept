use crate::notify::Notify;
use crate::schedule::Schedule;
use crate::user::User;
use anyhow::Result;

// Can have users or other teams?  KISS, just users for the minute
pub struct UserTeam {
    name: String,
    members: Vec<User>,
    schedule: Box<dyn Schedule>,
}

impl UserTeam {
    pub fn new(name: String, members: Vec<User>, schedule: Box<dyn Schedule>) -> Self {
        Self {
            name,
            members,
            schedule,
        }
    }

    pub fn push(&mut self, u: User) {
        self.members.push(u)
    }

    pub fn append(&mut self, users: &mut Vec<User>) {
        self.members.append(users)
    }
}

impl Notify for UserTeam {
    fn notify(&self) -> Result<Vec<User>> {
        self.schedule.schedule(&self.members)
    }
}
