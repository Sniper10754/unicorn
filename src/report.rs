use yansi::Color;

use std::fmt::Display;

pub fn report_string<M>(status: Color, left_message: M, right_message: M) -> String
where
    M: Display,
{
    let left_message = status.style().bold().paint(left_message);

    format!("{}: {}", left_message, right_message)
}

pub fn report<M>(status: Color, left_message: M, right_message: M)
where
    M: Display,
{
    let message = report_string(status, left_message, right_message);

    println!("{message}")
}

pub fn lp_report<M>(status: Color, left_message: M, right_message: M)
where
    M: Display,
{
    let message = lp_report_string(status, left_message, right_message);

    println!("{message}");
}

pub fn lp_report_string<M>(status: Color, left_message: M, right_message: M) -> String
where
    M: Display,
{
    //
    let left_message = status.style().bold().paint(left_message);

    format!("{:>12} {}", left_message, right_message)
}
