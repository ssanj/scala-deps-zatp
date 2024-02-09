use std::{format as s, process::ExitStatus};

use serde::Serialize;

pub enum ErrorTypes {
  FailedToExecuteCoursier(String, String, String, String),
  InvalidResponseEncoding(String, String, String, String),
  NoResults(String, String, String),
  InvalidStatusCode(String, String, String),
}

impl ErrorTypes {
  pub fn failedToExecuteCoursier(command: &str, error: String) -> ErrorTypes {
    ErrorTypes::FailedToExecuteCoursier(
      "Plugin Error".to_string(),
      s!("The 'scala-deps' plugin could not successfully execute coursier: '{}'.", command),
      error,
      s!("Verify the output of running '{}' is valid.", command)
      )
  }

  pub fn invalidResponseEncoding(command: &str, error: String) -> ErrorTypes {
    ErrorTypes::InvalidResponseEncoding(
      "Plugin Error".to_string(),
      "The 'scala-deps' plugin could not decode the output it received from coursier.".to_owned(),
      error,
      s!("Verify the output of running '{}' is valid.", command)
      )
  }

  pub fn noResults(command_str: &str) -> ErrorTypes {
    ErrorTypes::NoResults(
      "Plugin Error".to_string(),
      "The 'scala-deps' plugin did not receive any matching results coursier.".to_owned(),
      s!("Verify the output returned by courser by running '{}'", command_str)
    )
  }

  pub fn invalidStatusCode(command_str: &str, status: ExitStatus) -> ErrorTypes {
    ErrorTypes::InvalidStatusCode(
      "Plugin Error".to_string(),
      s!("The 'scala-deps' plugin received an error status code from coursier: '{}'.", status.code().map_or_else(||"Unknown".to_owned(), |ec| ec.to_string())),
      s!("Verify the output returned by courser by running '{}'", command_str)
    )
  }
}

#[derive(Debug, Clone, Serialize)]
pub struct PluginError {
  header: String,
  error: String,
  exception: Option<String>,
  fix: String
}

impl PluginError {
  pub fn new(header: String, error: String, exception: String, fix: String) -> Self {
    Self {
      header,
      error,
      exception: Some(exception),
      fix
    }
  }

  pub fn without_exception(header: String, error: String, fix: String) -> Self {
    Self {
      header,
      error,
      exception: None,
      fix
    }
  }
}

#[derive(Debug, Clone, Serialize)]
pub struct PluginSuccess {
  result: String,
  display_result: String
}

impl PluginSuccess {
  pub fn new(org: &str, group: &str, version: String) -> Self {
    let result = s!("\"{}\" %% \"{}\" % \"{}\"", org, group, version);
    let display_result = version;
    Self {
      result,
      display_result
    }
  }
}

pub type PResult<A> = Result<A, ErrorTypes>;
