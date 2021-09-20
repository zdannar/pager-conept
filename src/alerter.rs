use crate::alert::Alert;
use crate::user::User;
use anyhow::Result;

pub trait Alerter {
    fn send_alert(&self, u: &User, a: &Alert) -> Result<()>;
}

impl Alerter for Box<dyn Alerter> {
    fn send_alert(&self, u: &User, a: &Alert) -> Result<()> {
        self.as_ref().send_alert(u, a)
    }
}
