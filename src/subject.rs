use core::fmt;
use serde_json::Value;
use tbot::markup::{
    markdown_v2::Formattable,
    {bold, italic, underline},
};

#[derive(Clone)]
pub struct Subject {
    name: String,
    start_time: String,
    end_time: String,
    lecturer: String,
    auditorium: String,
    kind: String,
    pub date: String,
}

impl Subject {
    pub fn from_json(json: Value) -> Self {
        Self {
            name: json["discipline"].to_string().replace("\"", ""),
            start_time: json["beginLesson"].to_string().replace("\"", ""),
            end_time: json["endLesson"].to_string().replace("\"", ""),
            lecturer: json["lecturer"].to_string().replace("\"", ""),
            auditorium: json["auditorium"].to_string().replace("\"", ""),
            kind: json["kindOfWork"].to_string().replace("\"", ""),
            date: json["date"].to_string().replace("\"", ""),
        }
    }

    pub fn to_message(&self) -> impl Formattable {
        let Self {
            name,
            start_time,
            end_time,
            auditorium,
            lecturer,
            kind,
            ..
        } = self;

        (
            bold(name.clone()),
            "\n",
            underline(start_time.clone() + " - " + end_time),
            "\n",
            bold(italic(auditorium.clone())),
            "\n",
            italic(kind.clone()),
            "\n",
            italic(lecturer.clone()),
            "\n", "\n"
        )
    }
}

impl fmt::Display for Subject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            name,
            start_time,
            end_time,
            auditorium,
            lecturer,
            kind,
            date,
        } = self;

        if f.alternate() {
            write!(
                f,
                "{name}\n{start_time} - {end_time}\n{auditorium}\n{lecturer}\n{kind}\n{date}\n\n"
            )
        } else {
            write!(
                f,
                "{name}\n{start_time} - {end_time}\n{auditorium}\n{lecturer}\n{kind}\n\n"
            )
        }
    }
}
