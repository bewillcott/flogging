# Introduction

`Flogging` provides an easy framework for logging.

Log entries can be sent to the console (stdout or stderr), file, memory log, or a custom handler. They
can be formatted in various layouts: ISO8601, Simple, Unix TimeStamp, or a custom
layout.

Macros and public functions are provided, with the macros being the simplest method
of operation.

There are several levels for logging at:

- SEVERE,
- WARNING,
- INFO,
- CONFIG,
- FINE,
- FINER, and
- FINEST

There are even two special settings:

- ALL, and
- OFF
