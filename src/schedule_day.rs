use crate::subject::Subject;
use core::fmt;
use serde_json::Value;
use tbot::markup::{bold, markdown_v2::Formattable};

pub struct ScheduleDay {
    day: String,
    pub date: String,
    subjects: Vec<Subject>,
}

impl ScheduleDay {
    pub fn new(day_of_week: String, subjects_vec: Vec<Subject>) -> Self {
        Self {
            day: day_of_week,
            subjects: subjects_vec.clone(),
            date: String::from("0"),
        }
    }

    pub fn add_subject(&mut self, subject: Value) {
        self.subjects.push(Subject::from_json(subject));
    }

    pub fn set_date(&mut self) {
        self.date = self.subjects[0].date.clone();
    }

    pub fn to_message(&self) -> impl Formattable {
        let Self {
            day,
            subjects,
            date,
        } = self;

        let mut subjects_vec: Vec<Box<dyn Formattable + Send>> = vec![];

        for subject in subjects {
            subjects_vec.push(Box::new(subject.to_message()));
        }

        (bold(day.clone() + " - " + date), "\n", subjects_vec)
    }
}

impl fmt::Display for ScheduleDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            day,
            subjects,
            date,
        } = self;
        writeln!(f, "{day} - ({date})");
        for subject in subjects {
            write!(f, "{subject}");
        }

        Ok(())
    }
}
