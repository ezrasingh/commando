use stratagem::*;

// The state structure, containing a single value
#[derive(Default, Commander)]
struct State(i32);

impl State {
    pub fn value(&self) -> &i32 {
        &self.0
    }
}

// Define a command to translate (add) a value to the state
#[derive(Clone, Copy)]
struct Translate(i32);

impl Command<State> for Translate {
    fn execute(&mut self, ctx: &mut State) {
        ctx.0 = ctx.value().saturating_add(self.0);
    }

    fn undo(&mut self, ctx: &mut State) {
        ctx.0 = ctx.value().saturating_sub(self.0);
    }
}

fn main() {
    // Create a time machine that tracks the state and its history
    let mut machine: State = State::default();
    let mut cmd = Translate(10);
    // Execute commands
    machine.execute(cmd); // Adds 10 to the state
    cmd.undo(&mut machine);
}
