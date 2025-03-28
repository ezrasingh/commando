use commando::*;

// Define an alias for the type representing a value.
type Value<T = i32> = T;

// Define a `State` struct that will serve as the context for commands.
#[derive(Default, Commander)] // Derives `Commander` trait for `State`
struct State(Value); // The state holds a single value of type `Value` (i32)

// Implement methods for the `State` struct.
impl State {
    // Returns a reference to the value inside the state.
    pub fn value(&self) -> &Value {
        &self.0
    }
}

// Define a `Translate` struct which will represent a command that modifies the state.
struct Translate(Value); // `Translate` holds a single value, which will be added or subtracted.

impl Command<State> for Translate {
    // Implement the `execute` method to apply the command to the context (State).
    fn execute(&mut self, ctx: &mut State) {
        // Adds the value of `Translate` to the state's value, using `saturating_add` to avoid overflow.
        ctx.0 = ctx.value().saturating_add(self.0);
    }

    // Implement the `undo` method to revert the changes made by `execute`.
    fn undo(&mut self, ctx: &mut State) {
        // Reverts the state's value by subtracting the same amount.
        ctx.0 = ctx.value().saturating_sub(self.0);
    }
}

// Define a `Scale` struct which will represent a command that scales the value in the state.
struct Scale(Value, Option<Value>); // `Scale` holds a value for scaling and an optional previous value.

impl From<Value> for Scale {
    // Implement `From<Value>` to convert a `Value` into a `Scale` command.
    fn from(value: Value) -> Self {
        // Initialize the `Scale` command with the given value and set the previous value to `None`.
        Self(value, None)
    }
}

impl Command<State> for Scale {
    fn execute(&mut self, ctx: &mut State) {
        self.1.replace(*ctx.value());
        ctx.0 = ctx.value().saturating_mul(self.0);
    }
    fn undo(&mut self, ctx: &mut State) {
        if let Some(prev_state) = self.1.take() {
            ctx.0 = prev_state;
        }
    }
}

fn main() {
    use commando::time_machine::TimeMachine;

    let mut machine: TimeMachine<State> = State::default().into();

    machine.execute(Translate(10));

    machine.execute(Scale::from(1));
}
