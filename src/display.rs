extern crate yansi;

use cobweb_core::Config;
use cobweb_core::{Issue, IssuePriority};

use chrono::{Local, TimeZone};

use self::yansi::{Color, Style};

pub fn issue_short(issue: &Issue) {
    // styles
    let sty_begin = Style::new(Color::Yellow);
    let sty_property = Style::new(Color::White).bold();

    println!(
        "{} {} {} {} {}",
        sty_begin.paint(">"),
        sty_property.paint("H:"),
        issue.hash(),
        sty_property.paint("T:"),
        issue.title()
    );
}

pub fn issue_long(issue: &Issue) {
    // styles
    let sty_begin = Style::new(Color::Yellow);
    let sty_property = Style::new(Color::Default).bold();
    let sty_low = Style::new(Color::Green);
    let sty_medium = Style::new(Color::Yellow);
    let sty_high = Style::new(Color::Red);

    /* Note: Properties in bold
       >
       Title: Title of the issue
       Type: improvement                Creation date: 2018-04-23 10:23
       Hash: deadbeefdeadbeef           Parent: -
       Author: Pegasus                  Reviser: Zeus
       Start date: 2018-04-23 10:23     Due date: 2018-04-23 10:28
       Status: in progress    Priority: moderate         Progress: 045%
       Description:
       Lore ipsum
    */

    // seperator
    println!("{}", sty_begin.paint(">"));
    // title
    println!("{} {}", sty_property.paint("Title:"), issue.title());
    // type | creation date
    let dt = Local.timestamp(issue.creation_date(), 0);
    let dt = dt.format("%Y-%m-%d %H:%M").to_string();
    let typ = issue.typ().to_string();
    println!(
        "{} {:<26} {} {}",
        sty_property.paint("Type:"),
        typ,
        sty_property.paint("Creation date:"),
        dt
    );
    // hash | parent
    let hash = issue.hash().to_string();
    let parent = match *issue.parent() {
        Some(ref v) => v.to_string(),
        None => "-".to_string(),
    };
    println!(
        "{} {:<26} {} {}",
        sty_property.paint("Hash:"),
        hash,
        sty_property.paint("Parent:"),
        parent
    );
    // author | reviser
    let reviser = issue.assigned_to().to_owned().unwrap_or(String::from("-"));
    println!(
        "{} {:<24} {} {}",
        sty_property.paint("Author:"),
        issue.author(),
        sty_property.paint("Reviser:"),
        &reviser
    );
    // start date | due date
    let start_dt = Local.timestamp(issue.start_date(), 0);
    let start_dt = start_dt.format("%Y-%m-%d %H:%M").to_string();
    let due_dt = match *issue.due_date() {
        Some(v) => {
            let dt = Local.timestamp(v, 0);
            dt.format("%Y-%m-%d %H:%M").to_string()
        }
        None => "-".to_string(),
    };
    println!(
        "{} {:<20} {} {}",
        sty_property.paint("Start date:"),
        start_dt,
        sty_property.paint("Due date:"),
        due_dt
    );
    // status | priority | progress
    let status = issue.status().to_string();
    let sty_priority = match *issue.priority() {
        IssuePriority::Low => sty_low,
        IssuePriority::Medium => sty_medium,
        IssuePriority::High => sty_high,
    };
    let priority = issue.priority().to_string();
    println!(
        "{} {:<14} {} {:<16} {} {:>3}%",
        sty_property.paint("Status:"),
        status,
        sty_property.paint("Priority:"),
        sty_priority.paint(priority),
        sty_property.paint("Progress:"),
        issue.progress()
    );
    // description
    println!("{}", sty_property.paint("Description:"));
    if let Some(ref v) = *issue.description() {
        println!("{}", v);
    } else {
        println!("-");
    }
    println!();
}

pub fn config(config: &Config) {
    // TODO: use coloring from above
    if let Some(user) = config.user() {
        println!("User: {}", user);
    } else {
        println!("User: -");
    }
}
