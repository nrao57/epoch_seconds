# epoch_seconds

## Overview
`epoch_seconds` is a utility CLI program written in Rust to get the epoch seconds of an ISO8601 DateTime String

## Building the Rust Binary
Run the following command `cargo build --release`

After, there should be a binary located in `./target/release` named `epoch_seconds`


## To Get the HMAC SHA256 Signature of a JSON file

Run the following command
```
./target/release/epoch_seconds ISO8601_DATETIME_STRING
```

## Validation
To check the binary is built correctly, run the command
```
 ./target/release/epoch_seconds 2021-10-25T10:59:59-07:00
```

The command should result in
```
1635184799
```
