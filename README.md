# Workspacer
Demo for creating multiple libraries and binaries inside the same project

## To test
- Go to respective rust sub-projects
  - Build `safe_app` - `cargo test --release`
  - Build `safe_authenticator` - `cargo test --release`
- Go to respective c sub-projects and make sure to do `chmod +x run`
  - Run `safe_app` - `./run`
  - Run `safe_authenticator` - `./run`
