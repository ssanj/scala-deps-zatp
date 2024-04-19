use clap::Parser;

/// Zat plugin to fetch the latest stable dependency version for a Scala library
///
/// Examples:
///   - `scala-deps -o org.typelevel -g cats-core -s 2.13`    (cats-core for Scala 2.13)
///   - `scala-deps -o org.typelevel -g cats-core -s 3`       (cats-core for Scala 3)
///   - `scala-deps -o org.scala-lang -g scala-library`       (Scala 2 library)
///   - `scala-deps -o org.scala-lang -g scala3-library -s 3` (Scala 3 library)
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, verbatim_doc_comment)]
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

  /// This can be optional for Java dependencies or libraries that don't have a specific compiler version attached. For specific Scala version use one of 2.13 or 3.
  #[arg(short)]
  pub scala_version: Option<String>,
}

pub fn get_cli_args() -> Args {
  Args::parse()
}
