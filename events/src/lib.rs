use chrono::Duration;
use colored::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

use std::fmt;

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({:?}, {}, {})",
            self.position,
            self.size,
            self.content.truecolor(self.color.0, self.color.1, self.color.2)
        )
    }
}

use Event::*;

impl<'a> Event<'a> {
    pub fn notify(&self) -> Notification {
        match self {
            Event::Holiday =>
                Notification {
                    size: 25,
                    color: (0, 255, 0),
                    position: Position::Top,
                    content: "Enjoy your holiday".to_string(),
                },
            Event::Registration(duration) =>
                Notification {
                    size: 30,
                    color: (255, 2, 22),
                    position: Position::Top,
                    content: format!("You have {} left before the registration ends", from_duration_to_ft(duration)),
                },
            Event::Appointment(appointment) =>
                Notification {
                    size: 100,
                    color: (200, 200, 3),
                    position: Position::Center,
                    content: appointment.to_string(),
                },
            Event::Remainder(remainder) =>
                Notification {
                    size: 50,
                    color: (50, 50, 50),
                    position: Position::Bottom,
                    content: remainder.to_string(),
                },
        }
    }
}

pub fn from_duration_to_ft(duration: &Duration) -> String {
    let total_seconds = duration.num_seconds();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{}H:{}M:{}S", hours, minutes, seconds)
}
