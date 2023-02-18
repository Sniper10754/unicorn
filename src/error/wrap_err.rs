use yansi::Color;

use std::process::exit;

use super::{unicorn, report_err};

macro_rules! here {
    () => {
        concat!("at ", file!(), ":", line!(), ":", column!())
    };
}

pub trait WrapErr<T> {
    fn wrap_err(self, note: Option<&str>) -> T;
}

impl<T: Clone> WrapErr<T> for Option<T> {
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

                report_err(unicorn(format!("{err_message} at {}", here!())));
                exit(1);
            }
        }
    }
}

impl<T: Clone, E: std::error::Error> WrapErr<T> for Result<T, E> {
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

                report_err(unicorn(err_message));
                exit(1);
            }
        }
    }
}
