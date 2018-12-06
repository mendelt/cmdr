# cmdr
Library for building interactive shells in Rust. Inspired by the Python Cmd project

The following features need to be implemented for an intial release

- [X] Parse typed commands
- [X] Execute context commands
- [X] Call context method on empty command
- [X] Call context method on unknown command
- [ ] Macro to auto-generate context command function from available methods
- [ ] Process command results to quit cmdr application
- [ ] Nested or sub-contexts
- [X] Show configurable prompt
- [ ] Command history
- [ ] Auto complete commands
- [ ] Help function

Write more examples
- [ ] A small mud
- [ ] Show nested contexts when implemented
- [ ] Show auto-completion when implemented

Other Todo's
- [ ] Write tests
- [ ] Re-think memory/ownership for passing commands
- [ ] Split off input provider

