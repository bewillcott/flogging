# Overview

This guide is provided to assist you in both using and customizing `FLogging`.

For those interested, I have uploaded the complete example project, [`my_project`], to Github. It contains all of the code developed throughout this guide. Please note, that each example is stored under a different 'branch' of the repository.

## Usage

To use `FLogging` you have two options:

- macros\
  For simple straight-forward logging.

  They are very simple and easy to use, with the minimum of coding required.

- functions/methods\
    For when you require more flexibility in your logging regimen.

    As an extreme example, you could have separate log tracking for each function/method in your project. For example: separate log files. Though why such separation would be of use I have no idea.

    Another example would be, if you only require logging in certain functions, with different requirements, then this could be achieved.

## Customization

The available options are:

- Custom Handlers
- Custom Formatters

Each of these are independent of the other. You can use custom handlers with the built-in formatters, and custom formatters with the built-in handlers. Or, if you want to be very fancy, you can use your custom formatter with your custom handler!!! What a great idea, hey?

I have tried to make this process as simple as possible.  Though there are many ways I could have gone with this, I decided that having separate modules/files for each one was the easiest and most efficient option, when it comes down to maintainability.

So, let's get started.

[`my_project`]: https://github.com/bewillcott/my_project
