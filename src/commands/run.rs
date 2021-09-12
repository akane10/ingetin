use crate::{
    db::DB,
    error::Error,
    event::{Event, Events},
    show_notif,
};
use tokio::time;

fn get_events() -> Result<Events, Error> {
    let events = DB::get_all()?;
    let newest = Event::filter_past(&Event::newest(&events));
    Ok(newest)
}

fn get_next_event() -> (Option<Event>, usize) {
    let events = get_events().unwrap_or(Vec::new());
    if events.len() > 0 {
        (Some(events[0].clone()), events.len())
    } else {
        (None, 0)
    }
}

fn show_info(event: &Event, total_event: usize) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let start_in = event.start_in().num_seconds();
    let h = start_in / 3600;
    let m = (start_in % 3600) / 60;
    let s = start_in % 60;
    println!("you have {} event in coming", total_event);
    println!("next event: {}", event.title);
    println!(
        "start at: {}",
        event.start_at.format("%d-%m-%Y %H:%M").to_string()
    );
    println!("in {} hours {} minutes {} seconds", h, m, s);
}

pub async fn run() -> Result<(), Error> {
    let mut in_ = 10;
    let mut show = true;
    let mut interval = time::interval(time::Duration::from_secs(1));

    loop {
        interval.tick().await;
        let (next_event, total_event) = get_next_event();

        match next_event {
            Some(ref event) => {
                let seconds = event.start_in().num_seconds();
                let m = (seconds % 3600) / 60;
                let title = &event.title;
                show_info(event, total_event);

                if seconds == 0 {
                    let body = format!("{} is now!!", title);
                    show_notif(title, &body)?;
                    in_ = 10;
                    show = true;
                } else if m <= in_ && show && m > 0 {
                    let body = format!("start in {} minutes", m);
                    show_notif(title, &body)?;
                    in_ -= 5;
                    show = false;
                }
            }
            _ => break,
        }
    }

    show_notif("No More Event", "Shuting down, no event no more")?;
    Ok(())
}
