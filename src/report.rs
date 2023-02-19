use yansi::Color;

use std::fmt::Display;

pub struct Report;

impl Report {
    pub fn report<M: Display>(status: Color, left_message: M, right_message: M) {
        println!(
            "{}",
            Report::report_string(status, left_message, right_message)
        )
    }

    pub fn report_string<M: Display>(status: Color, left_message: M, right_message: M) -> String {
        format!(
            "{}: {}",
            status.style().bold().paint(left_message),
            right_message
        )
    }

    pub fn lp_report<M: Display>(status: Color, left_message: M, right_message: M) {
        println!(
            "{}",
            Report::lp_report_string(status, left_message, right_message)
        );
    }

    pub fn lp_report_string<M: Display>(
        status: Color,
        left_message: M,
        right_message: M,
    ) -> String {
        format!(
            "{:>12} {}",
            status.style().bold().paint(left_message),
            right_message
        )
    }
}
