use crate::request_handler::get_json;
use crate::schedule_day::ScheduleDay;
use core::fmt;
use core::fmt::Display;
use serde_json::Value;

pub struct Schedule {
    days: Vec<ScheduleDay>,
}

impl Schedule {
    pub async fn new(group: String) -> Self {
        Self {
            days: Schedule::parse_to_days(group).await,
        }
    }
    async fn parse_to_days(group: String) -> Vec<ScheduleDay> {
        let src: Value =
            serde_json::from_str(&get_json(group).await.text().await.unwrap()).unwrap();
        let src = src.as_array().unwrap().to_owned();

        let mut days: Vec<ScheduleDay> = Vec::with_capacity(6);
        days.push(ScheduleDay::new("Понедельник".to_string(), vec![]));
        days.push(ScheduleDay::new("Вторник".to_string(), vec![]));
        days.push(ScheduleDay::new("Среда".to_string(), vec![]));
        days.push(ScheduleDay::new("Четверг".to_string(), vec![]));
        days.push(ScheduleDay::new("Пятница".to_string(), vec![]));
        days.push(ScheduleDay::new("Суббота".to_string(), vec![]));

        for subject in src {
            days[(subject["dayOfWeek"].as_u64().unwrap() - 1) as usize].add_subject(subject);
        }

        days
    }
}

impl fmt::Display for Schedule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for day in &self.days {
            Display::fmt(day, f)?;
        }

        Ok(())
    }
}
