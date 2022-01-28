use crate::subject::Subject;
use core::fmt;
use serde_json::Value;

pub struct ScheduleDay {
    day: String,
    date: String,
     subjects: Vec<Subject>,
}

impl ScheduleDay {
    pub fn new(day_of_week: String, subjects_vec: Vec<Subject>) -> Self {
        Self {
            day: day_of_week,
            subjects: subjects_vec,
        }
    }

    pub fn add_subject(&mut self, subject: Value) {
        self.subjects.push(Subject::from_json(subject));
    }
}

impl fmt::Display for ScheduleDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (day, subjects) = (&self.day, &self.subjects);
        writeln!(f, "{day}");
        for subject in subjects {
            write!(f, "{subject}");
        }

        Ok(())
    }
}
