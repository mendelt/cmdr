# The scope
Cmdr applications can be modal. An application can accept different commands in diferent modes. The mode the application is in is determined by the `scope` it is in. Each scope has commands defined on it that are available to the user when that `scope` is active.

The getting started example had the `GreeterScope` as its single scope. But multiple scopes can be defined. Any type annotated with the `#[cmdr]` macro can act as a scope. The scope can hold state that can be accessed by the commands.

