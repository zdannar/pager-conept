use crate::alert::Alert;
use crate::alerter::Alerter;
use crate::user::User;
use anyhow::Result;
use std::convert::From;

pub struct NotificationAgent<T> {
    pub agent: T,
}

impl<T> NotificationAgent<T>
where
    T: Alerter,
{
    pub fn new(agent: T) -> Self {
        NotificationAgent { agent }
    }

    pub fn send_alert(&self, user: &User, alert: &Alert) -> Result<()> {
        self.agent.send_alert(user, alert)
    }
}

// TODO: Not sure... doesn't work
impl From<Box<dyn Alerter>> for NotificationAgent<Box<dyn Alerter>> {
    fn from(a: Box<dyn Alerter>) -> Self {
        NotificationAgent::new(a)
    }
}

//the trait `From<Box<TerminalAgent>>` is not implemented for `NotificationAgent<Box<dyn Alerter>>
impl From<Box<TerminalAgent>> for NotificationAgent<Box<dyn Alerter>> {
    fn from(a: Box<TerminalAgent>) -> NotificationAgent<Box<dyn Alerter>> {
        NotificationAgent { agent: a }
    }
}

impl From<TerminalAgent> for NotificationAgent<Box<dyn Alerter>> {
    fn from(a: TerminalAgent) -> NotificationAgent<Box<dyn Alerter>> {
        NotificationAgent { agent: Box::new(a) }
    }
}

impl From<EmailAgent> for NotificationAgent<Box<dyn Alerter>> {
    fn from(a: EmailAgent) -> NotificationAgent<Box<dyn Alerter>> {
        NotificationAgent { agent: Box::new(a) }
    }
}

pub struct TerminalAgent {
    // Some configuration stuff in here
}

impl TerminalAgent {
    pub fn new() -> Self {
        TerminalAgent {}
    }
}

impl Alerter for TerminalAgent {
    fn send_alert(&self, user: &User, alert: &Alert) -> Result<()> {
        println!("Terminal notification to {} : {}", user, alert);
        Ok(())
    }
}

pub struct EmailAgent {
    // Some configuration stuff in here
}

impl EmailAgent {
    pub fn new() -> Self {
        EmailAgent {}
    }
}

impl Alerter for EmailAgent {
    fn send_alert(&self, user: &User, alert: &Alert) -> Result<()> {
        println!("Email notification to {} : {}", user, alert);
        Ok(())
    }
}
