use crate::{
    db::DB,
    error::Error,
    event::{Event, Events},
    HasId,
};
use chrono::{Datelike, Timelike};
use dialoguer::MultiSelect;

pub fn delete() -> Result<(), Error> {
    let events: Events = Event::newest(&DB::get_all()?);
    let items: Vec<String> = events
        .iter()
        .map(|x| {
            let title = &x.title;
            let start_at = x.start_at;
            let when = match x.is_past() {
                true => format!("past event"),
                false => format!("in comming event"),
            };
            let time = format!(
                "{}-{}-{} {}:{} {}",
                start_at.day(),
                start_at.month(),
                start_at.year(),
                start_at.hour(),
                start_at.minute(),
                when
            );

            format!("{} ({})", title, time)
        })
        .collect();

    if items.len() < 1 {
        println!("Nothing to be deleted");
        Ok(())
    } else {
        let chosen: Vec<usize> = MultiSelect::new().items(&items).interact()?;
        let mut chosen_event_ids = Vec::new();

        for i in chosen {
            chosen_event_ids.push(events[i].get_id())
        }
        DB::delete_many(&chosen_event_ids)?;
        let info = format!("{} event deleted", chosen_event_ids.len());
        println!("{}", info);
        Ok(())
    }
}
