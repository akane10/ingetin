use crate::HasId;
use chrono::{DateTime, Duration, Local, TimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type Events = Vec<Event>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub title: String,
    pub kind: String,
    pub description: String,
    pub start_at: DateTime<Local>,
    pub end_at: Option<DateTime<Local>>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl HasId for Event {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

impl Event {
    pub fn new<S: Into<String>>(
        title: S,
        kind: S,
        description: S,
        start_at: DateTime<Local>,
        end_at: Option<DateTime<Local>>,
    ) -> Self {
        let now = Local::now();
        Self {
            id: Uuid::new_v4(),
            title: title.into(),
            kind: kind.into(),
            description: description.into(),
            start_at,
            end_at,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_past(&self) -> bool {
        self.start_at < Local::now()
    }

    pub fn _start_in_minutes(&self, m: i64) -> DateTime<Local> {
        self.start_at
            .checked_add_signed(Duration::minutes(m))
            .unwrap()
    }

    pub fn start_in(&self) -> Duration {
        DateTime::signed_duration_since(self.start_at, Local::now())
    }

    pub fn create_start_at(y: i32, mt: u32, d: u32, h: u32, m: u32, s: u32) -> DateTime<Local> {
        Local.ymd(y, mt, d).and_hms(h, m, s)
    }

    pub fn newest(events: &Events) -> Events {
        let mut xs = events.clone();
        xs.sort_by(|a, b| a.start_at.partial_cmp(&b.start_at).unwrap());
        xs
    }

    pub fn filter_past(events: &Events) -> Events {
        events
            .clone()
            .into_iter()
            .filter(|x| !x.is_past())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_in_test() {
        let now = Local::now();
        let event = Event::new(
            "test",
            "test desc",
            "test",
            now.checked_add_signed(Duration::hours(1)).unwrap(),
            None,
        );

        assert_eq!(event.start_in().num_minutes(), 59);
    }

    #[test]
    fn newest_test() {
        let event = Event::new(
            "test",
            "test desc",
            "test",
            Local::now()
                .checked_sub_signed(Duration::minutes(3))
                .unwrap(),
            None,
        );
        let event1 = Event::new(
            "test",
            "test desc",
            "test",
            Local::now()
                .checked_sub_signed(Duration::minutes(5))
                .unwrap(),
            None,
        );
        let event2 = Event::new(
            "test",
            "test desc",
            "test",
            Local::now()
                .checked_sub_signed(Duration::minutes(2))
                .unwrap(),
            None,
        );

        let mut xs = Vec::new();
        xs.push(event);
        xs.push(event1);
        xs.push(event2);

        let ys = Event::newest(&xs);

        assert!(xs[1].id == ys[0].id);
    }

    #[test]
    fn newest_should_not_mutate() {
        let event = Event::new(
            "test",
            "test desc",
            "test",
            Local::now()
                .checked_sub_signed(Duration::minutes(3))
                .unwrap(),
            None,
        );
        let event1 = Event::new(
            "test",
            "test desc",
            "test",
            Local::now()
                .checked_sub_signed(Duration::minutes(1))
                .unwrap(),
            None,
        );
        let event2 = Event::new(
            "test",
            "test desc",
            "test",
            Local::now()
                .checked_sub_signed(Duration::minutes(2))
                .unwrap(),
            None,
        );

        let mut xs = Vec::new();
        xs.push(event);
        xs.push(event1);
        xs.push(event2);

        let ys = Event::newest(&xs);

        assert!(xs[1].id != ys[1].id);
    }

    #[test]
    fn filter_past_test() {
        let event = Event::new(
            "test",
            "test desc",
            "test",
            Local::now()
                .checked_add_signed(Duration::minutes(3))
                .unwrap(),
            None,
        );
        let event1 = Event::new(
            "test",
            "test desc",
            "test",
            Local::now()
                .checked_add_signed(Duration::minutes(1))
                .unwrap(),
            None,
        );
        let event2 = Event::new(
            "test",
            "test desc",
            "test",
            Local::now()
                .checked_sub_signed(Duration::minutes(2))
                .unwrap(),
            None,
        );

        let mut xs = Vec::new();
        xs.push(event);
        xs.push(event1);
        xs.push(event2);

        let ys = Event::filter_past(&xs);

        assert!(ys.len() == 2);
    }
}
