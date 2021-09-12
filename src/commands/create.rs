use crate::{db::DB, error::Error, event::Event};
use chrono::{Datelike, Local, Timelike};
use dialoguer::Input;
use std::num::ParseIntError;

fn ask_input(
    s: &str,
    val: Option<String>,
    f: impl Fn(String) -> Result<(), String>,
) -> Result<String, Error> {
    let inital_text = match val {
        Some(v) => v,
        _ => String::new(),
    };
    let input: String = Input::new()
        .with_prompt(s)
        .with_initial_text(&inital_text)
        .default(inital_text)
        .show_default(false)
        .allow_empty(false)
        .validate_with(|ss: &String| f(ss.clone()))
        .interact_text()?;

    Ok(input)
}

fn validate_date(n: u32) -> impl Fn(String) -> Result<(), String> {
    let validator = move |s: String| -> Result<(), String> {
        let x: Result<u32, ParseIntError> = s.parse();
        match x {
            Ok(val) => {
                if val < n + 1 {
                    Ok(())
                } else {
                    let msg: String = format!("can't be more than {}", n);
                    Err(msg)
                }
            }
            Err(e) => Err(e.to_string()),
        }
    };

    validator
}

fn print_info(e: &Event) {
    println!("Event created");
    println!("title: {}", e.title);
    println!("description: {}", e.description);
    println!("hour: {}", e.start_at.hour());
    println!("minute: {}", e.start_at.minute());
    println!(
        "in {} hour {} minute",
        e.start_in().num_hours(),
        e.start_in().num_minutes()
    );
}

pub fn create() -> Result<(), Error> {
    let now = Local::now();
    let year = now.year();
    let month = now.month();
    let day = now.day();
    let hour = now.hour();
    let minute = now.minute();
    let validate_nothing = |_: _| Ok(());

    let title = ask_input("title", None, validate_nothing)?;
    let body = ask_input("body", None, validate_nothing)?;
    let desc = ask_input("description", None, validate_nothing)?;

    let mt = ask_input("month", Some(month.to_string()), validate_date(12))?;
    // TODO: make day match with month
    let d = ask_input("day", Some(day.to_string()), validate_date(31))?;
    let h = ask_input("hour", Some(hour.to_string()), validate_date(23))?;
    let m = ask_input("minute", Some(minute.to_string()), validate_date(59))?;

    let mt_n: u32 = mt.parse()?;
    let d_n: u32 = d.parse()?;
    let h_n: u32 = h.parse()?;
    let m_n: u32 = m.parse()?;

    let start_at_event = Event::create_start_at(year, mt_n, d_n, h_n, m_n, 0);
    let event = Event::new(
        title,
        body,
        String::from("prompt"),
        desc,
        start_at_event,
        None,
    );

    print_info(&event);

    DB::write(&event)?;
    Ok(())
}
