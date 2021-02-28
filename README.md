# structlog

[![dependency status](https://deps.rs/repo/github/JonPulfer/structlog/status.svg)](https://deps.rs/repo/github/JonPulfer/structlog)
![docs](https://docs.rs/structlog/badge.svg)
[![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/structlog.svg
[crates.io]: https://crates.io/crates/structlog

If you like to include useful pieces of information in log messages but don't want to use verbose text to join all the
pieces together, this crate might get you some way towards that. It won't help you make your messages useful nor will it
be the fastest way to get messages into _such-and-such_ system. The primary outflow for these messages is as a `JSON`
serialised structure that just includes the keys and values given to the logger.

This is currently being built to use in another of my projects, so it will be updated and extended as needs arise.
Contributions are welcome and encouraged. Please see the [Contribution guide](CONTRIBUTING.md) to find out how you can
help.

## Current usage options

You create an `Event` which implements `fmt::Display` to output JSON for convenience. There are a few useful fields
added when you create the event like a created timestamp and the caller location. Creating an event can either be done
using a `Event::from_str()` like: -

```rust

// By default the event will be created with a level/severity of INFO.
let test_event = Event::from_str("some event");

// To make it another level you can do: -
let mut test_event_error = Event::from_str("something bad happened");
test_event_error.error();
```

or you can use: -

```rust
let mut test_event = Event::new();
test_event.add_field(
String::from("message"),
String::from("some useful message"),
);
```

If you `println!("{}", test_event)` you would see something like: -

```json
{
  "attributes": {
    "message": "Some useful message"
  },
  "created": "2021-02-02T20:04:06.949823Z",
  "level": "INFO",
  "severity": "INFO",
  "caller": "tests/integration_tests.rs:5:18"
}
```