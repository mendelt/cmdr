# cmdr
Library for building an interactive shell in Rust. Inspired by the Python Cmd project

The following features need to be implemented for an intial release

- [X] Parse typed commands
- [X] Execute context commands
- [X] Call context method on empty command
- [X] Call context method on unknown command
- [ ] Macro to auto-generate context command function from available methods
- [ ] Process command results to quit cmdr application
- [ ] Call sub-contexts
- [X] Show configurable prompt
- [ ] Command history
- [ ] Auto complete commands

Other Todo's
- [ ] Write tests
- [ ] Re-think memory/ownership for passing commands
