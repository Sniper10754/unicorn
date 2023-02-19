use yansi::Color;

use std::process::exit;

use crate::report::Report;

use super::unicorn;

macro_rules! here {
    () => {
        concat!("at ", file!(), ":", line!(), ":", column!())
    };
}

pub trait WrapErr<T> {
    /// Inspired version of `unwrap` method that
    /// exits in case of `Err` or `None`,
    /// giving a pretty error message.
    fn wrap_err(self, note: Option<&str>) -> T;
}

impl<T> WrapErr<T> for Option<T> {
    fn wrap_err(self, note: Option<&str>) -> T {
        match self {
            Some(x) => x,
            None => {
                let mut err_message =
                    format!("tried to call `unwrap` on an option containing `None`");

                match note {
                    Some(x) => {
                        err_message.push_str(&format!("\n{}: {}", Color::Magenta.paint("note"), x))
                    }
                    None => (),
                }

                Report::report_err(unicorn(format!("{err_message} at {}", here!())));
                exit(1);
            }
        }
    }
}

impl<T, E: std::error::Error> WrapErr<T> for Result<T, E> {
    fn wrap_err(self, note: Option<&str>) -> T {
        match self {
            Ok(x) => x,
            Err(err) => {
                let mut err_message = format!("{err} at {}", here!());

                match note {
                    Some(x) => {
                        err_message.push_str(&format!("\n{}: {}", Color::Magenta.paint("note"), x))
                    }
                    None => (),
                }

                Report::report_err(unicorn(err_message));
                exit(1);
            }
        }
    }
}
