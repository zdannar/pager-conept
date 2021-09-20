use std::fmt;

// See: https://www.guru99.com/defect-severity-in-software-testing.html
#[derive(Debug)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug)]
pub enum Severity {
    Critical,
    Major,
    Medium,
    Low,
}

pub struct Alert {
    pub alert_team: String,
    pub priority: Priority,
    pub severity: Severity,
    source: String,
    message: String,
}

impl Alert {
    pub fn new(
        alert_team: String,
        priority: Priority,
        severity: Severity,
        source: String,
        message: String,
    ) -> Self {
        Alert {
            alert_team,
            priority,
            severity,
            source,
            message,
        }
    }
}

impl fmt::Display for Alert {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} : {:?} : {} : {}",
            self.priority, self.severity, self.source, self.message
        )?;
        Ok(())
    }
}
