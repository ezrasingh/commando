# Stratagem

Stratagem is a Rust trait library that enables state management and command execution with built-in undo support, compatible with `no_std` environments. Inspired by the [Command design pattern](https://refactoring.guru/design-patterns/command/rust/example), it provides a flexible framework for modifying state through actions while tracking and undoing them efficiently.

This makes Stratagem ideal for use cases requiring undo/redo functionality, such as interactive applications, game state management, and embedded systems where heap allocation is limited.

## Key Features

- **Command Pattern**: Encapsulates actions that modify a state into command objects.
- **Undo Functionality**: Allows you to undo previous commands, reverting the state back to its prior value.
- **History Tracking**: Stores a history of executed commands for potential future undos.
- **Time Travel**: Step back through changes made to a state.

## Getting Started

### Define State

The `Commander` trait defines how the state executes commands and undos (optional).

```rust
use stratagem::Commander;

#[derive(Default)]
struct State {
    value: i32
}

impl Commander for State {
    fn execute(&mut self, mut cmd: impl Command<Self> + 'static) {
        cmd.execute(self);
    }
}

impl State {
    pub fn value(&self) -> &i32 {
        &self.value
    }
}
```

We can achieve the same result by using the `Commander` derive macro.

```rust
use stratagem::Commander;

#[derive(Commander)]
struct State {
    value: i32
}

impl State {
    pub fn value(&self) -> &i32 {
        &self.value
    }
}
```

### Define Commands

The `Command` trait defines how commands interact with the state during execution and undo.

```rust

use stratagem::Command;

// Define a command to translate (add/sub) a value to the state
#[derive(Clone, Copy)]
struct Translate(i32);

impl Translate {
    pub fn new(value: i32) -> Self {
        Self(value)
    }
}

impl Command<State> for Translate {
    fn execute(&mut self, ctx: &mut State) {
        ctx.value = ctx.value().saturating_add(self.0);
    }

    fn undo(&mut self, ctx: &mut State) {
        ctx.value = ctx.value().saturating_sub(self.0);
    }
}

// Define a command to scale (mult/div) a value to the state
#[derive(Clone, Copy)]
struct Scale {
    factor: i32,
    previous_value: Option<i32>,
};

impl Scale {
    pub fn new(value: i32) -> Self {
        Self{ value, previous_value: None }
    }
}

impl Command<State> for Scale {
    fn execute(&mut self, ctx: &mut State) {
        // copy current value and store it
        self.previous_value.replace(ctx.value());
        ctx.value = ctx.value().saturating_mul(self.factor);
    }

    fn undo(&mut self, ctx: &mut State) {
        // ctx.value = ctx.value().saturating_div(self.factor); // could panic
        if let Some(last_value) = self.previous_value.take() {
            ctx.value = last_value;
        };
    }
}
```

### Running Commands

Dynamically dispatch commands at runtime

```rust
use stratagem::*;

let mut state = State::default();

let mut cmd = (
    Translate::new(5),
    Scale::new(0)
);

state.execute(cmd.0);
assert_eq!(state.value(), 5);

state.execute(cmd.1);
assert_eq!(state.value(), 0);

cmd1.undo(&mut state);
assert_eq!(state.value(), 5);
```

### Time Travel

By enabling the `time-machine` feature (requires `std` due to `Vec` and `Box` depencies) we can convert anything implementing the `Commander` trait into a `TimeMachine` which wraps the state and provides a `history` to store past commands. The underlying state is accessible from the `machine` property.

```rust
use stratagem::time_machine::TimeMachine;

let mut state = TimeMachine::<State>::default();;
let mut cmd = Scale::from(10);

state.execute(cmd);

println!("{}", state.machine.value());

state.undo();
println!("{}", state.machine.value());
```

## Contributing

Contributions are welcome! Please see the [contributing guidelines](CONTRIBUTING.md) for more information.

## License

This project is licensed under the [Apache 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) (your choice).
