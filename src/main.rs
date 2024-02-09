// use args::cli;
use std::process::Command;
use std::format as s;

use version_compare::Version;
use error::{ErrorTypes, PResult};

mod args;
mod error;

fn main() {
  // let args = cli::get_cli_args();
  // TODO: Add parameters for depenency prefix
  let coursier_result = run_coursier("org.scalatest:scalatest_2.13:");

  let zat_result =
    match coursier_result {
      Ok(result) => encodeSuccess(result),
      Err(error) => encodeError(error),
    };

  match zat_result {
    Ok(success) => println!("{}", success),
    Err(error) => println!("{}", error),
  }
}

fn encodeError(error: ErrorTypes) -> Result<String, serde_json::Error> {
    match error {
      ErrorTypes::FailedToExecuteCoursier(_, _, _, _) => todo!(),
      ErrorTypes::InvalidResponseEncoding(_, _, _, _) => todo!(),
      ErrorTypes::NoResults(_, _, _) => todo!(),
      ErrorTypes::InvalidStatusCode(_, _, _) => todo!(),
    }
}

fn encodeSuccess(result: String) -> serde_json::Result<String> {
  todo!()
    //serde_json::to_string(result)
}

fn run_coursier(dependency_prefix: &str) -> PResult<String> {
  let mut command = Command::new("cs");

  command
    .arg("complete-dep")
    .arg(dependency_prefix);

  let command_str = get_program(&command);

  let output = command.output().map_err(|e| ErrorTypes::failedToExecuteCoursier(&command_str, e.to_string()))?;
  let status = &output.status;

  if status.success() {
    let response =
      std::str::from_utf8(&output.stdout)
        .map_err(|e| ErrorTypes::invalidResponseEncoding(&command_str, "output", e.to_string()))?;

    let mut response_versions =
      response
        .lines()
        .filter(|l| !l.contains("-"))
        .filter_map(|l| Version::from(l))
        .collect::<Vec<_>>();

    response_versions
      .sort_by(|x, y| x.compare(y).opposite().ord().expect(&s!("Could not compare order of {} and {}", x, y)));

    let responses =
      response_versions
        .into_iter()
        .map(|l| l.to_string())
        .collect::<Vec<_>>();

    let latest =
      responses
      .first()
      .ok_or_else(|| ErrorTypes::noResults(&command_str))?;

    Ok(latest.to_owned())
  } else {
    Err(ErrorTypes::invalidStatusCode(&command_str, *status))
  }
}

fn get_program(command: &Command) -> String {
  let program = command.get_program().to_string_lossy().to_string();
  let args =
    command
      .get_args()
      .into_iter()
      .map(|arg| arg.to_string_lossy().to_string())
      .collect::<Vec<_>>()
      .join(" ");
  s!("{} {}", program, args)
}
