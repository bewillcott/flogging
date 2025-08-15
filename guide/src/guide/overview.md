# Overview

This guide is provided to assist you in customizing `FLogging`.

The available options are:

- Custom Handlers
- Custom Formatters

Each of these are independent of the other. You can use custom handlers with the built-in formatters, and custom formatters with the built-in handlers. Or, if you want to be very fancy, you can use your custom formatter with your custom handler!!! What a great idea, hey?

I have tried to make this process as simple as possible.  Though there are many ways I could have gone with this, I decided that having separate modules/files for each one was the easiest and most efficient option, when it comes down to maintainability.

So, let's get started.
