use crate::notify::Notify;
use crate::user::User;
use anyhow::Result;

pub struct AlertTeam {
    pub name: String,
    pub members: Vec<Box<dyn Notify>>,
}

impl AlertTeam {
    pub fn new(name: String) -> Self {
        AlertTeam {
            name,
            members: Vec::new(),
        }
    }

    pub fn push(&mut self, u: Box<dyn Notify>) {
        self.members.push(u)
    }

    fn idx_on_call(&self) -> usize {
        return 0;
    }
}

impl Notify for AlertTeam {
    fn notify(&self) -> Result<Vec<User>> {
        let idx = self.idx_on_call();
        let oc = &self.members[idx];
        oc.notify()
    }
}
