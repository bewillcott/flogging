# Step 1

## Why?

Firstly, you need to know why you want/need to develop a custom handler.

For our example, we are going to develop a simple handler that combines outputting to both the 'stdout' console, and a log file. Note, that this would, at best, be a convenience handler, as it would replace two built-in handlers.

## How?

Now you need to know how you would process each log entry, so that you could either send or store it some where, and how you would get it there.

For our example, we need to do two separate things:

1. `println!("{}", self.con_fmt.format(log_entry));`

2. ```rust, no_run, noplayground
   let mut buf = self.file_fmt.format(log_entry);
   buf.push('\n');
   self.file.as_mut().unwrap().write_all(buf.as_bytes()).expect("writeln!() failed");
   ```
