# Firebase secrets cli tool 


## Description:

A powerful command-line tool built with Rust, allowing users to easily set multiple secrets at once from a file containing key-value pairs. This eliminates the need for manual, time-consuming configuration of individual secrets, streamlining the process and simplifying the management of sensitive data in Firebase projects.

## Steps: 
- Verify that you have the Rust toolchain installed on your system.
- Clone the project to your local machine by running `git clone https://github.com/ippsav/firebase-secrets-cli.git`.
- In the root directory of the cloned project, run `cargo install --path .` and ensure that the `~/.cargo/bin` directory is included in your system's $PATH environment variable.

Note: You can install `firebase-secrets-cli` directly by running `cargo install --git https://github.com/ippsav/firebase-secrets-cli.git`

## Usage:
- To set a single key: `firebase-secrets-cli -a dev -s MYSECRET=MYVALUE`
- To set multiple keys from a file: `firebase-secrets-cli -a dev -p path/to/secrets`
- For more information, run `firebase-secrets-cli --help`

Note: The `-a` flag specifies the Firebase project for which the secrets will be set, and the `-s` flag is used for setting individual secrets, while the `-p` flag is used for setting multiple secrets from a file.
