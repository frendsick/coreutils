// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::thread;
use std::time::Duration;

use uucore::{
    error::{UResult, USimpleError, UUsageError},
    format_usage,
    parser::parse_time,
    show_error,
};

use clap::{Arg, ArgAction, Command};

use uucore::locale::get_message;

mod options {
    pub const NUMBER: &str = "NUMBER";
}

#[uucore::main]
pub fn uumain(args: impl uucore::Args) -> UResult<()> {
    let matches = uu_app().try_get_matches_from(args)?;

    let numbers = matches
        .get_many::<String>(options::NUMBER)
        .ok_or_else(|| {
            USimpleError::new(
                1,
                format!(
                    "missing operand\nTry '{} --help' for more information.",
                    uucore::execution_phrase()
                ),
            )
        })?
        .map(|s| s.as_str())
        .collect::<Vec<_>>();

    sleep(&numbers)
}

pub fn uu_app() -> Command {
    Command::new(uucore::util_name())
        .version(uucore::crate_version!())
        .about(get_message("sleep-about"))
        .after_help(get_message("sleep-after-help"))
        .override_usage(format_usage(&get_message("sleep-usage")))
        .infer_long_args(true)
        .arg(
            Arg::new(options::NUMBER)
                .help("pause for NUMBER seconds")
                .value_name(options::NUMBER)
                .action(ArgAction::Append),
        )
}

fn sleep(args: &[&str]) -> UResult<()> {
    let mut arg_error = false;

    let sleep_dur = args
        .iter()
        .filter_map(|input| match parse_time::from_str(input, true) {
            Ok(duration) => Some(duration),
            Err(error) => {
                arg_error = true;
                show_error!("{error}");
                None
            }
        })
        .fold(Duration::ZERO, |acc, n| acc.saturating_add(n));

    if arg_error {
        return Err(UUsageError::new(1, ""));
    };
    thread::sleep(sleep_dur);
    Ok(())
}
