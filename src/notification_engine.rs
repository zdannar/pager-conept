use crate::alert::{Alert, Priority, Severity};
use crate::alerter::Alerter;
use crate::notification_agent::NotificationAgent;
use crate::user::User;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

pub struct NotificationEngine<T> {
    hm: HashMap<String, Box<NotificationAgent<T>>>,
}

impl<T> NotificationEngine<T>
where
    T: Alerter,
{
    pub fn new() -> Self {
        let hm: HashMap<String, Box<NotificationAgent<T>>> = HashMap::new();
        NotificationEngine { hm }
    }

    pub fn register(
        &mut self,
        user: &User,
        priority: &Priority,
        severity: &Severity,
        na: NotificationAgent<T>,
    ) {
        self.hm.insert(
            format!("{}:{:?}:{:?}", user, priority, severity),
            Box::new(na),
        );
    }
    // TODO: Refactor
    // - Needs error
    fn get(&self, u: &User, a: &Alert) -> Option<&Box<NotificationAgent<T>>> {
        // TODO: Generate key again
        let key = format!("{}:{:?}:{:?}", u, a.priority, a.severity);
        self.hm.get(&key)
    }

    pub fn notifiy(&self, u: &User, a: &Alert) -> Result<()> {
        // TODO: Handle missing user alert preference.
        //self.get(u, a).unwrap().send_alert(u, a)?;
        self.get(u, a)
            .ok_or(anyhow!(
                "User `{}` is not configured to handle this alert Priority `{:?}`, Severity `{:?}`",
                &u,
                a.priority,
                a.severity
            ))?
            .send_alert(u, a)?;
        Ok(())
    }
}
