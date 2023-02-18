use std::{error::Error, fmt::Display};

mod wrap_err;
pub use wrap_err::*;

use crate::report::{self, report_string};

use yansi::Color;

pub fn unicorn<M>(message: M) -> UnicornError
where
    M: ToString,
{
    UnicornError {
        description: message.to_string(),
    }
}

#[derive(Debug)]
pub struct UnicornError {
    description: String,
}

impl Display for UnicornError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.description)
    }
}

impl Error for UnicornError {}

pub fn report_err_string<E>(err: &E) -> String
where
    E: std::error::Error,
{
    report_string(Color::Red, "error", &format!("{}", err))
}

pub fn report_err<E>(err: E)
where
    E: std::error::Error,
{
    let message = report_err_string(&err);

    println!("{}", message);

    match err.source() {
        Some(err_source) => report::report(Color::Magenta, "caused by", &format!("{}", err_source)),
        None => (),
    }
}
