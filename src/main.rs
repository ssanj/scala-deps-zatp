use args::cli;
use std::process::Command;
use std::format as s;

use version_compare::Version;
use error::{ErrorTypes, PResult, PluginError, PluginSuccess};

mod args;
mod error;

fn main() {
  let args = cli::get_cli_args();
  let org = &args.org;
  let group = &args.group;
  let scala_version = args.scala_version;
  let coursier_result = run_coursier(&s!("{}:{}_{}:", org, group, scala_version));

  let zat_result =
    match coursier_result {
      Ok(result) => encode_success(org, group, result),
      Err(error) => encode_error(error),
    };

  match zat_result {
    Ok(success) => println!("{}", success),
    Err(error) => println!("{}", error),
  }
}

fn encode_error(error: ErrorTypes) -> Result<String, serde_json::Error> {
  let plugin_error = match error {
    ErrorTypes::FailedToExecuteCoursier(header, error, exception, fix) => PluginError::new(header, error, exception, fix),
    ErrorTypes::InvalidResponseEncoding(header, error, exception, fix) => PluginError::new(header, error, exception, fix),
    ErrorTypes::NoResults(header, error, fix) => PluginError::without_exception(header, error, fix),
    ErrorTypes::InvalidStatusCode(header, error, fix) => PluginError::without_exception(header, error, fix),
  };

  serde_json::to_string(&plugin_error)
}

fn encode_success(org: &str, group: &str, result: String) -> serde_json::Result<String> {
  let plugin_success = PluginSuccess::new(org, group, result);
  serde_json::to_string(&plugin_success)
}

fn run_coursier(dependency_prefix: &str) -> PResult<String> {
  let mut command = Command::new("cs");

  command
    .arg("complete-dep")
    .arg(dependency_prefix);

  let command_str = get_program(&command);

  let output =
    command
      .output()
      .map_err(|e| ErrorTypes::failedToExecuteCoursier(&command_str, e.to_string()))?;

  let status = &output.status;

  if status.success() {
    let response =
      std::str::from_utf8(&output.stdout)
        .map_err(|e| ErrorTypes::invalidResponseEncoding(&command_str, e.to_string()))?;

    let mut response_versions =
      response
        .lines()
        .filter(|l| !l.contains("-")) // Remove, pre-release, milestones, RCs etc.
        .filter_map(|l| Version::from(l))
        .collect::<Vec<_>>();

    response_versions
      .sort_by(|x, y| x.compare(y).opposite().ord().expect(&s!("Could not compare order of {} and {}", x, y))); // There doesn't seem to be a way to do this safely.

    let responses =
      response_versions
        .into_iter()
        .map(|l| l.to_string()) // Convert them from Version back to Strings.
        .collect::<Vec<_>>();

    let latest =
      responses
      .first() // Get the first/latest version
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
