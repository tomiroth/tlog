use chrono::{
    prelude::{DateTime, Local},
    Duration, TimeDelta,
};

use crate::task::Task;

fn pretty_duration(duration: TimeDelta) -> String {
    // Extract days, hours, and minutes from the Duration
    let days = duration.num_days();
    let hours = (duration - Duration::days(days)).num_hours();
    let minutes = (duration - Duration::days(days) - Duration::hours(hours)).num_minutes();

    let days = if days > 0 {
        (format!("{}:", days), format!("{}:", "Days"))
    } else {
        ("".to_owned(), "".to_owned())
    };

    let hours = if hours > 0 {
        (format!("{}:", hours), format!("{}:", "Hours"))
    } else {
        ("".to_owned(), "".to_owned())
    };

    let time = format!("{}{}{}", days.0, hours.0, minutes);
    let description = format!("{}{}{}", days.1, hours.1, "Mins");

    format!("{} ({})", time, description)
}

pub struct TaskOut;
impl TaskOut {
    pub fn current_task(task: Task) {
        let now = Local::now();
        let duration = now - task.start;

        let ticket_string = match task.ticket_number {
            Some(t) => format!(" <{}>", t),
            _ => "".to_owned(),
        };
        println!("*******************");
        println!("* {}{}", task.name, ticket_string);
        println!("* - Project: {}", task.project);
        println!("* - Time Logged: {}", pretty_duration(duration));
        println!("*******************");
    }
}
