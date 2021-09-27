mod alert;
mod alert_team;
mod alerter;
mod notification_agent;
mod notification_engine;
mod notify;
mod schedule;
mod user;
mod user_team;

use crate::notify::Notify;
use alert::{Alert, Priority, Severity};
use alert_team::AlertTeam;
use alerter::Alerter;
use anyhow::Result;
use notification_agent::{EmailAgent, TerminalAgent};
use notification_engine::NotificationEngine;
use schedule::{CallAllSchedule, RandomSchedule};
use user::{User, UserStatus};
use user_team::UserTeam;

fn main() -> Result<()> {
    let ut = UserTeam::new(
        "The Wookies".to_string(),
        vec![
            User::new("john.doe".to_string(), UserStatus::Active),
            User::new("jane.doe".to_string(), UserStatus::Active),
            User::new("jim.bob".to_string(), UserStatus::Active),
            User::new("jimmy.dean".to_string(), UserStatus::Active),
        ],
        Box::new(RandomSchedule::new()),
    );
    let mut at = AlertTeam::new(
        "Wookies and MGMT".to_string(),
        Box::new(CallAllSchedule::new()),
    );
    at.push(Box::new(User::new(
        "El Hefe".to_string(),
        UserStatus::Active,
    )));

    at.push(Box::new(ut));

    let mut ne: NotificationEngine<Box<dyn Alerter>> = NotificationEngine::new();

    ne.register(
        &User::new("john.doe".to_string(), UserStatus::Active),
        &Priority::High,
        &Severity::Critical,
        TerminalAgent::new().into(),
    );

    ne.register(
        &User::new("john.doe".to_string(), UserStatus::Active),
        &Priority::High,
        &Severity::Low,
        EmailAgent::new().into(),
    );

    ne.register(
        &User::new("El Hefe".to_string(), UserStatus::Active),
        &Priority::High,
        &Severity::Low,
        EmailAgent::new().into(),
    );

    // Make an alert
    let a = Alert::new(
        "Owen and SRE".to_string(),
        Priority::High,
        Severity::Critical,
        "Checkly".to_string(),
        "Rocky is down!".to_string(),
    );

    let a2 = Alert::new(
        "Owen and SRE".to_string(),
        Priority::High,
        Severity::Low,
        "Checkly ".to_string(),
        "Drago is down!".to_string(),
    );

    // Need a map of AlertTeams to map specific alerts to.
    println!("Alert #1");
    let notif_users = at.notify().expect("Damn it!");
    let mut notif_errors: Vec<Result<()>> =
        notif_users.iter().map(|u| ne.notifiy(&u, &a)).collect();
    println!("{:?}", notif_errors);

    println!("Alert #2");
    notif_errors = notif_users.iter().map(|u| ne.notifiy(&u, &a2)).collect();
    println!("{:?}", notif_errors);

    Ok(())
}
