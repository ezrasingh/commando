extern crate std;
use std::{boxed::Box, vec::Vec};

use crate::{Command, Commander};

/// A type alias for a vector of commands, representing the history of commands executed.
///
/// The `History<T>` type is a container that holds all previously executed commands
/// of type `Command<T>` in a dynamic form (`Box<dyn Command<T>>`). This allows the
/// `TimeMachine` to track and store commands for potential undo actions.
pub type History<T> = Vec<Box<dyn Command<T>>>;

/// A structure that represents a time machine capable of executing and undoing commands from a linear history.
///
/// The `TimeMachine` holds a `Commander` (of type `T`), which is the context where commands are
/// applied, along with a history of all executed commands. This allows the time machine to replay
/// commands (via `execute`) and revert actions (via `undo`).
///
/// The `TimeMachine` itself acts as a `Commander`, meaning it can execute and undo commands directly
/// and also manage a history of past commands for future undos.
pub struct TimeMachine<T>
where
    T: Sized + Commander, // T must implement Commander to be able to execute/undo commands.
{
    /// The machine or context that the time machine operates on.
    pub machine: T,

    /// The history of commands that have been executed, stored for potential undo actions.
    history: History<T>,
}

impl<T> TimeMachine<T>
where
    T: Sized + Commander, // T must implement Commander to be used in TimeMachine.
{
    /// Returns a reference to the history of commands executed in the time machine.
    ///
    /// This allows users to inspect the list of commands that have been executed so far.
    /// The history is stored as a vector of boxed commands (`Box<dyn Command<T>>`).
    pub fn history(&self) -> &History<T> {
        self.history.as_ref() // Returns a reference to the history vector.
    }
}

impl<T> From<T> for TimeMachine<T>
where
    T: Sized + Commander, // T must implement Commander to be used in TimeMachine.
{
    /// Creates a new `TimeMachine` from an existing context (`T`).
    ///
    /// This converts an instance of `T` into a `TimeMachine<T>`. The `TimeMachine` starts with an
    /// empty history of commands and operates on the provided `T` as the context (or "machine").
    ///
    /// # Parameters
    /// - `machine`: The context of type `T` that will be managed by the `TimeMachine`.
    ///
    /// # Returns
    /// - A `TimeMachine` that wraps the provided context and initializes an empty history.
    fn from(machine: T) -> Self {
        Self {
            machine,
            history: Vec::default(), // Initializes an empty history.
        }
    }
}

impl<T> Default for TimeMachine<T>
where
    T: Default + Sized + Commander,
{
    /// Creates an empty `TimeMachine` from a default context (`T`).
    ///
    /// This creates a default instance of `T` inside a `TimeMachine<T>`. The `TimeMachine` starts with an
    /// empty history of commands and operates on the provided `T` as the context (or "machine").
    ///
    /// # Returns
    /// - A `TimeMachine` that wraps the default context and initializes an empty history.
    fn default() -> Self {
        Self {
            machine: T::default(),
            history: Vec::default(), // Initializes an empty history.
        }
    }
}

impl<T> Commander<T> for TimeMachine<T>
where
    T: Sized + Commander, // T must implement Commander to be used as the context.
{
    /// Executes a command and pushes it onto the history stack.
    ///
    /// This method delegates the execution of the command to the context (`machine`),
    /// and then adds the executed command to the history for potential future undos.
    ///
    /// # Parameters
    /// - `cmd`: The command to be executed on the `machine`.
    ///
    /// The command is wrapped in a `Box` and stored in the history to keep track of it.
    fn execute(&mut self, mut cmd: impl Command<T> + 'static) {
        cmd.execute(&mut self.machine); // Executes the command on the `machine`.
        self.history.push(Box::new(cmd)); // Adds the executed command to history.
    }

    /// Undoes the most recently executed command.
    ///
    /// This method pops the last command from the history stack and calls its `undo` method to revert
    /// the changes made by that command on the `machine`.
    ///
    /// If there are no commands in the history, this method does nothing.
    fn undo(&mut self) {
        if let Some(mut cmd) = self.history.pop() {
            cmd.undo(&mut self.machine); // Reverts the most recent command's effect on the `machine`.
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;

    #[derive(Default, Commander)]
    struct State(i32);

    impl State {
        pub fn value(&self) -> i32 {
            self.0
        }
    }

    struct Translate(i32);

    impl Command<State> for Translate {
        fn execute(&mut self, ctx: &mut State) {
            ctx.0 = ctx.value().saturating_add(self.0);
        }

        fn undo(&mut self, ctx: &mut State) {
            ctx.0 = ctx.value().saturating_sub(self.0);
        }
    }

    struct Scale(i32, Option<i32>);

    impl From<i32> for Scale {
        fn from(value: i32) -> Self {
            Self(value, None)
        }
    }

    impl Command<State> for Scale {
        fn execute(&mut self, ctx: &mut State) {
            self.1.replace(ctx.value());
            ctx.0 = ctx.value().saturating_mul(self.0);
        }

        fn undo(&mut self, ctx: &mut State) {
            if let Some(prev_state) = self.1.take() {
                ctx.0 = prev_state;
            };
        }
    }

    #[test]
    fn can_time_travel() {
        let mut state = TimeMachine::<State>::default();
        assert_eq!(state.machine.value(), State::default().value());

        state.execute(Translate(5));
        assert_eq!(state.machine.value(), 5);

        state.execute(Translate(10));
        assert_eq!(state.machine.value(), 15);

        state.execute(Scale::from(0));
        assert_eq!(state.machine.value(), 0);

        state.undo();
        assert_eq!(state.machine.value(), 15);

        state.undo();
        assert_eq!(state.machine.value(), 5);

        state.undo();
        assert_eq!(state.machine.value(), 0);

        state.undo();
        state.undo();
        assert_eq!(state.machine.value(), 0);
        assert_eq!(state.history().len(), 0);
    }
}
