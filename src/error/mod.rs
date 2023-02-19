use std::{error::Error, fmt::Display};

mod wrap_err;
pub use wrap_err::*;

use crate::report::Report;

use yansi::Color;

pub fn unicorn<M>(message: M) -> UnicornError
where
    M: Display,
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

impl Report {
    pub fn report_err_string<E: std::error::Error>(err: &E) -> String {
        Report::report_string(Color::Red, "error", &format!("{}", err))
    }

    pub fn report_err<E: std::error::Error>(err: E) {
        println!("{}", Report::report_err_string(&err));

        match err.source() {
            Some(err_source) => {
                Report::report(Color::Magenta, "caused by", &format!("{}", err_source))
            }
            None => (),
        }
    }
}
