# Demo example for showing iframe switching not working

Run the server by running `cargo run` from inside the `server` directory.

Run `geckodriver` in another terminal.

Yet again in another terminal, run the minimal reproduction program from the project root with `cargo run`.

You should observe that the minimal reproduction program fails with the following error:
```
thread 'tokio-runtime-worker' panicked at 'not yet implemented', /home/<NAME>/.cargo/git/checkouts/fantoccini-bbdc2bd83aa60452/0715a02/src/session.rs:487:18
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
Error: Lost(Custom { kind: BrokenPipe, error: "WebDriver session was closed while waiting" })
```

And the geckodriver cli program doesn't exit, but it will keep the browser window open and will refuse to start a 
new session if you run the error reproduction program again (Session is already started).