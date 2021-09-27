use crate::notify::Notify;
use crate::schedule::Schedule;
use crate::user::User;
use anyhow::Result;

pub struct AlertTeam {
    pub name: String,
    pub members: Vec<Box<dyn Notify>>,
    pub schedule: Box<dyn Schedule>,
}

impl AlertTeam {
    pub fn new(name: String, schedule: Box<dyn Schedule>) -> Self {
        AlertTeam {
            name,
            members: Vec::new(),
            schedule,
        }
    }

    pub fn push(&mut self, u: Box<dyn Notify>) {
        self.members.push(u)
    }
}

impl Notify for AlertTeam {
    fn notify(&self) -> Result<Vec<User>> {
        let mut members: Vec<User> = Vec::new();

        for member in &self.members {
            // TODO: Fix unwrap
            members.extend(member.notify().unwrap());
        }

        self.schedule.schedule(&members)
    }
}
