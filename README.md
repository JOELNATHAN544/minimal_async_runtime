# MINIMAL ASYNC RUNTIME

- This project does a minimal single threaded async runtime.
- It sleeps for a given duration that uses a spawn function and returns a joinHandle().
- It runs a top-level async function via block_on() that drives the future to completion and terminates.
- It uses a macro to simplify configuration.
- Has a cooperative yielding (yield_now().await)
- A join_all! macro is implemented to await multiple JoinHandles

##  Installation

```bash
git clone https://github.com/your-username/project-name.git
cd project-name
cargo build

## Dependencies

```bash
tokio = { version = "1.36", features = ["time"] }
futures = "0.3"

