use clap::Parser;

/// Zat plugin to fetch the latest stable dependency version for a Scala library
///
/// Examples:
///
/// scala-deps -o org.typelevel -g cats-core -s 2.13
///
/// scala-deps -o org.scala-lang -g scala-library
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

  /// Scala version. One of 2.13 or 3. This can be optional for Java dependencies or libraries that don't have a specific compiler version attached.
  #[arg(short)]
  pub scala_version: Option<String>,
}

pub fn get_cli_args() -> Args {
  Args::parse()
}
