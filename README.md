# MINIMAL ASYNC RUNTIME

- This project does a minimal single threaded async runtime.
- It sleeps for a given duration that uses a spawn function and returns a joinHandle().
- It runs a top-level async function via block_on() that drives the future to completion and terminates.
- 

## 📦 Installation

```bash
git clone https://github.com/your-username/project-name.git
cd project-name
cargo build
