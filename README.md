# rust-password-generator

Rust Password Generator is a simple and lightweight password generator written in Rust.

## Features

- Generate a random password with options to include or exclude numbers and symbols in password generation to increase password complexity.
- Generate a random PIN number with the option to configure the PIN number length.
- Analyze the security and crack time estimations of the generated password or pin number.

## Getting Started

You will need to have Rust and Cargo installed.

```bash
# Build the project
$ cargo build --release

# Run all tests
$ cargo test
```

## Usage

```bash
$ cargo run -- -h
A simple and lightweight password generator written in Rust

Usage: rust-password-generator.exe [OPTIONS] <COMMAND>

Commands:
  normal  Generate a random password with specified complexity
  pin     Generate a random PIN number
  help    Print this message or the help of the given subcommand(s)

Options:
      --analyze
      --copy
      --seed <SEED>
  -h, --help         Print help
  -V, --version      Print version
```

### Generate random password

```bash
$ cargo run -- normal -h
Generate a random password with specified complexity

Usage: rust-password-generator.exe normal [OPTIONS]

Options:
  -l, --length <LENGTH>  [default: 12]
  -n, --numbers
  -s, --symbols
  -h, --help             Print help (see more with '--help')
```

```bash
# Generate password with default settings
$ cargo run -- normal
JyXAmuqSKiyf

# Generate password with options
$ cargo run -- normal -n -s -l 20
q39rCtO{2gYm13w]4Sxl

# Generate password and analyze
$ cargo run --analyze normal -n -s -l 20
┌──────────────────────┐
│ Generated Password   │
├──────────────────────┤
│ ~A:lRpB1Q/_9H-z=@J^% │
└──────────────────────┘
┌──────────┬────────────────────┐
│ Password Security Analysis    │
├──────────┼────────────────────┤
│ Strength │ very strong        │
├──────────┼────────────────────┤
│ Guesses  │ 19.265919722494797 │
└──────────┴────────────────────┘
┌───────────────────────┬───────────┐
│ Password Crack Time Estimations   │
├───────────────────────┼───────────┤
│ 100 attempts/hour     │ centuries │
├───────────────────────┼───────────┤
│ 10 attempts/second    │ centuries │
├───────────────────────┼───────────┤
│ 10^4 attempts/second  │ centuries │
├───────────────────────┼───────────┤
│ 10^10 attempts/second │ 57 years  │
└───────────────────────┴───────────┘
```

### Generate random PIN number

```bash
$ cargo run -- pin -h
Generate a random PIN number

Usage: rust-password-generator.exe pin [OPTIONS]

Options:
  -l, --length <LENGTH>  [default: 4]
  -h, --help             Print help (see more with '--help')
```

```bash
# Generate PIN number with default settings
$ cargo run -- pin
2045

# Generate PIN number with options
$ cargo run -- pin -l 12
088573096367

# Generate PIN number and analyze
$ cargo run -- --analyze pin -l 12
┌────────────────────┐
│ Generated Password │
├────────────────────┤
│ 088573096367       │
└────────────────────┘
┌──────────┬───────────────────┐
│ Password Security Analysis   │
├──────────┼───────────────────┤
│ Strength │ very strong       │
├──────────┼───────────────────┤
│ Guesses  │ 11.17638069224327 │
└──────────┴───────────────────┘
┌───────────────────────┬────────────┐
│ Password Crack Time Estimations    │
├───────────────────────┼────────────┤
│ 100 attempts/hour     │ centuries  │
├───────────────────────┼────────────┤
│ 10 attempts/second    │ centuries  │
├───────────────────────┼────────────┤
│ 10^4 attempts/second  │ 5 months   │
├───────────────────────┼────────────┤
│ 10^10 attempts/second │ 15 seconds │
└───────────────────────┴────────────┘
```

### Extra options

You can also set the seed number for deterministic password generation (for testing purposes).

```bash
$ cargo run -- --seed 12345 normal
WCGSImLDEapZ
```

## Additional Notes

- The normal password generator mode uses a weighted distribution method, in which the weights depend on the settings enabled:

  - 60% alphabets, 20% numbers, 20% symbols
  - 80% alphabets, 20% numbers
  - 80% alphabets, 20% symbols
  - 100% alphabets

  Note that in rare cases, due to the weighted distribution method, it will generate a password without numbers/symbols even though users have already enabled the enable numbers and symbols option.

- The password security analysis and password crack time estimations uses the [zxcvbn](https://github.com/shssoichiro/zxcvbn-rs) library, which is a password strength estimator through pattern matching and conservative estimation, based on Dropbox's zxcvbn library.

## License

rust-password-generator is distributed under the MIT license
