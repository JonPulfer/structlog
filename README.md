# structlog

If you like to include useful pieces of information in log messages but don't want to use verbose text to join all the pieces
together, this crate might get you some way towards that. It won't help you make your messages useful nor will it be the fastest
way to get messages into _such-and-such_ system. The primary outflow for these messages is as a `JSON` serialised structure that
just includes the keys and values given to the logger.

This is currently being built to use in another of my projects, so it will be updated and extended as needs arise. Contributions 
are welcome and encouraged. Please see the [Contribution guide](CONTRIBUTING.md) to find out how you can help.

## Current usage options

You create an `Event` which implements `fmt::Display` to output the JSON. Creating an event can either be done using
a `parse()` like: -

```rust

// By default the event will be created with a level/severity of INFO.
let test_event: event::Event = "some event".parse().unwrap();

// To make it another level you can do: -
let test_event_error: event::Event = "sonething bad happened".parse<event::Event>().unwrap().error();
```

or you can use: -

```rust
    let mut test_event = event::Event::new();
    test_event.add_field(
        String::from("message"),
        String::from("this is the error message"),
    );
```

If you `println!("{}", test_event)` you would see something like: -

```text
{"attributes":{"message":"Some error message"},"created":"2021-01-30T20:41:08.883084Z","level":"INFO","severity":"INFO"}
```