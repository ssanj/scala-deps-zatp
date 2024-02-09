use clap::Parser;

/// Zat plugin to fetch scala dependency versions
#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct Args {
   /// Verbose debug logging
   #[arg(long)]
   pub verbose: bool,

   /// Org String. Eg. org.typelevel
   #[arg(short)]
   pub org: String,

   /// Group String. Eg. cats-core
   #[arg(short)]
   pub group: String,

  /// Scala version. One of 2.13 or 3.
  #[arg(short)]
  pub scala_version: String,
}

pub fn get_cli_args() -> Args {
  Args::parse()
}
