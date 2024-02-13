use std::{format as s, process::ExitStatus};

use serde::Serialize;
use crate::PLUGIN_NAME;

pub enum ErrorTypes {
  FailedToExecuteCoursier(String, String, String),
  InvalidResponseEncoding(String, String, String),
  NoResults(String, String),
  InvalidStatusCode(String, String),
}

impl ErrorTypes {
  pub fn failed_to_execute_coursier(command: &str, error: String) -> ErrorTypes {
    ErrorTypes::FailedToExecuteCoursier(
      s!("The 'scala-deps' plugin could not successfully execute coursier: '{}'.", command),
      error,
      s!("Verify the output of running '{}' is valid.", command)
      )
  }

  pub fn invalid_response_encoding(command: &str, error: String) -> ErrorTypes {
    ErrorTypes::InvalidResponseEncoding(
      "The 'scala-deps' plugin could not decode the output it received from coursier.".to_owned(),
      error,
      s!("Verify the output of running '{}' is valid.", command)
      )
  }

  pub fn no_results(command_str: &str) -> ErrorTypes {
    ErrorTypes::NoResults(
      "The 'scala-deps' plugin did not receive any matching results from coursier.".to_owned(),
      s!("Verify the output returned by courser by running '{}'", command_str)
    )
  }

  pub fn invalid_status_code(command_str: &str, status: ExitStatus) -> ErrorTypes {
    ErrorTypes::InvalidStatusCode(
      s!("The 'scala-deps' plugin received an error status code from coursier: '{}'.", status.code().map_or_else(||"Unknown".to_owned(), |ec| ec.to_string())),
      s!("Verify the output returned by courser by running '{}'", command_str)
    )
  }
}

#[derive(Debug, Clone, Serialize)]
pub enum PluginResult {
  Success(PluginSuccess),
  Error(PluginError)
}

#[derive(Debug, Clone, Serialize)]
pub struct PluginError {
  plugin_name: String,
  error: String,
  exception: Option<String>,
  fix: String
}

impl PluginError {
  pub fn new(error: String, exception: String, fix: String) -> Self {
    Self {
      plugin_name: PLUGIN_NAME.to_owned(),
      error,
      exception: Some(exception),
      fix
    }
  }

  pub fn without_exception(error: String, fix: String) -> Self {
    Self {
      plugin_name: PLUGIN_NAME.to_owned(),
      error,
      exception: None,
      fix
    }
  }
}

#[derive(Debug, Clone, Serialize)]
pub struct PluginSuccess {
  result: String,
}

impl PluginSuccess {
  pub fn new(result: String) -> Self {
    Self {
      result,
    }
  }
}

pub type PResult<A> = Result<A, ErrorTypes>;
