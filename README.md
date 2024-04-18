# Scala Deps

![GitHub Release](https://img.shields.io/github/v/release/ssanj/scala-deps-zatp)

[Zat](https://github.com/ssanj/zat) plugin to fetch the latest stable dependency version for a Scala library.

## Installation

### Prerequisites

`scala-deps` uses [Coursier](https://get-coursier.io/) in the background. As such Coursier must be [installed](https://get-coursier.io/docs/cli-installation) for `scala-deps` to work. You do not need to do this if you install `scala-deps` through [Homebrew](https://brew.sh/).


### Homebrew

```
brew tap ssanj/homebrew-scala-deps-zatp
brew install scala-deps
```

Note: Homebrew automatically installs all necessary dependencies for `scala-deps`

### Downloading a Release

Download the latest [release](https://github.com/ssanj/scala-deps/releases) for your operating system (linux or macos).
Make it executable with:

`chmod +x <SCALA_DEPS_EXEC>`

Copy executable to a directory on your path.

### Building from Source

Ensure you have [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) and [Rust](https://www.rust-lang.org/) installed.

Run:

```
cargo build --release
Copy binary file from target/release/scala-deps to a directory on your path.
```

You can use `./release-local` to do the above if your `~/bin` folder is on your PATH.


## Zat Usage

To use `scala-deps` in a Zat repository, define the following properties in your `.variables.zat-prompt`:

```
  {
    "variable_name": "YOUR_VARIABLE_NAME",
    "description": "WHAT YOUR_VARIABLE_NAME IS",
    "prompt": "HOW TO ASK FOR YOUR_VARIABLE_NAME",
    "plugin": {
      "id": "scala-deps",
      "args":[
          "-o",
          "<ORGANIZATION>",
          "-g",
          "<GROUP>",
          "-s",
          "<SCALA_VERSION>"
      ]
    }
  }
```

Here's an example of using `scala-deps` to look up the latest stable version of Scala 3 via the `.variables.zat-prompt` file:

```json
  {
    "variable_name": "scala_3_version",
    "description": "Which version of Scala 3 to use",
    "prompt": "Please enter Scala 3 version to use",
    "plugin": {
      "id": "scala-deps",
      "args":[
          "-o",
          "org.scala-lang",
          "-g",
          "scala3-library",
          "-s",
          "3"
      ]
    }
  }
```

## Stand-alone Usage

`scala-deps -h`:

```
Zat plugin to fetch the latest stable dependency version for a Scala library

Usage: scala-deps [OPTIONS] -o <ORG> -g <GROUP>

Options:
      --verbose           Verbose debug logging
  -o <ORG>                Org String. Eg. org.typelevel
  -g <GROUP>              Group String. Eg. cats-core
  -s <SCALA_VERSION>      Scala version. One of 2.13 or 3. This can be optional for Java dependencies or libraries that don't have a specific compiler version attached
  -h, --help              Print help (see more with '--help')
  -V, --version           Print version
```

Here's how to look up the latest stable version of Scala 3:

```
scala-deps -o org.scala-lang -g scala3-library -s 3
```

which results in:

```json
{
    "success":
    {
        "result": "3.3.1"
    }
}
```

Here's how to look up the latest stable version of Scala 2:

```
scala-deps -o org.scala-lang -g scala-library
```

which results in:

```json
{
    "success":
    {
        "result": "2.13.13"
    }
}
```

## Output

### Success

Looking up a dependency that exists with:

```
scala-deps -o org.typelevel -g cats-core -s 2.13
```

results in:

```.json
{
    "success":
    {
        "result": "2.10.0"
    }
}
```

### Error

Looking up a dependency that does not exist with:

```
scala-deps -o org.typelevel -g cats-snore -s 2.13
```

results in:

```.json
{
    "error":
    {
        "plugin_name": "scala-deps",
        "error": "The 'scala-deps' plugin did not receive any matching results from coursier.",
        "exception": null,
        "fix": "Verify the output returned by courser by running 'cs complete-dep org.typelevel:cats-snore_2.13:'"
    }
}
```
