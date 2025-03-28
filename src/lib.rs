#![no_std]
pub use commando_macros::*;

#[cfg(feature = "time-machine")]
pub mod time_machine;

/// A trait that represents a command that can be applied to a context.
///
/// Commands are actions that can be executed on a context of type `T`.
/// They can also be undone, allowing the system to revert to its previous state.
pub trait Command<T: Sized> {
    /// Applies the command to the given context.
    ///
    /// This method executes the logic defined by the command, modifying
    /// the state of the context `T`.
    ///
    /// # Parameters
    /// - `ctx`: A mutable reference to the context of type `T` that the command will affect.
    fn execute(&mut self, ctx: &mut T);

    /// Reverts the command's effect on the context.
    ///
    /// This method undoes the changes made by the `execute` method, restoring
    /// the context `T` to its previous state.
    ///
    /// # Parameters
    /// - `ctx`: A mutable reference to the context of type `T` that will be reverted.
    fn undo(&mut self, ctx: &mut T);
}

/// A trait for types that can manage the execution and undoing of commands.
///
/// Types that implement `Commander` are responsible for executing and potentially
/// undoing commands of type `Command<T>`. A `Commander` may represent a system
/// or an entity that can manipulate its state through a series of commands.
pub trait Commander<T = Self>: Sized {
    /// Executes a given command on the commander.
    ///
    /// This method delegates the execution of a command to the `Command` trait,
    /// applying the command to the state of the `Commander` (or the context `T`).
    ///
    /// # Parameters
    /// - `cmd`: The command to be executed. The `Command<T>` trait ensures that
    ///   the command can modify the context `T`.
    fn execute(&mut self, cmd: impl Command<T> + 'static);

    /// Optionally undoes the effect of the previously executed command.
    ///
    /// This method is provided as a default implementation and can be overridden
    /// by types that need custom undo behavior. By default, this method does nothing.
    fn undo(&mut self) {}
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

    #[derive(Clone, Copy)]
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
    fn can_execute() {
        let mut state: State = State::default();
        assert_eq!(state.value(), 0);

        state.execute(Translate(7));
        assert_eq!(state.value(), 7);

        state.execute(Scale::from(2));
        assert_eq!(state.value(), 14);
    }

    #[test]
    fn can_undo() {
        let mut state: State = State(0);

        let mut cmds = (Translate(5), Translate(10), Scale::from(0));

        assert_eq!(state.value(), 0);

        state.execute(cmds.0);
        assert_eq!(state.value(), 5);

        state.execute(cmds.1);
        assert_eq!(state.value(), 15);

        state.execute(cmds.2);
        assert_eq!(state.value(), 0);

        cmds.2.undo(&mut state);
        assert_eq!(state.value(), 15);

        cmds.1.undo(&mut state);
        assert_eq!(state.value(), 5);

        cmds.0.undo(&mut state);
        assert_eq!(state.value(), 0);

        state.undo();
        assert_eq!(state.value(), 0);
    }
}
