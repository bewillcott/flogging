# Wrap Up

The use of the methods directly, requires a lot more coding in general, hence maintenance,
and the inclusion of boilerplating of the 'logger' setup.

The primary benefit, is the ability to provide different 'logger' configurations for each
function, as needed.

As mentioned in the API, the methods are the backbone of the `FLogging` crate. The macros
are simply convenient wrappers for them. Thereby removing the need for you to track and maintain
that code.
