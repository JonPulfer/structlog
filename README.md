# structlog

If you like to include useful pieces of information in log messages but don't want to use verbose text to join all the pieces
together, this crate might get you some way towards that. It won't help you make your messages useful nor will it be the fastest
way to get messages into _such-and-such_ system. The primary outflow for these messages is as a `JSON` serialised structure that
just includes the keys and values given to the logger.

This is currently being built to use in a work project, so it will be updated and extended as needs arise. Contributions are welcome
and encouraged. Please see the [Contribution guide](CONTRIBUTING.md) to find out how you can help.

## Using as middleware

I am building this to use as middleware in web services using the Iron framework and will add that functionality here as
modules.