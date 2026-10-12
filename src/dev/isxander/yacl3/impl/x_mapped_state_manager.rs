//! Ported from dev.isxander.yacl3.impl.XMappedStateManager.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::state_manager::{ResetAction, StateListener, StateManager};

/// java: public class XMappedStateManager<I, O> implements StateManager<O>
pub struct XMappedStateManager<I: Clone + 'static, O: Clone + 'static> {
    /// java: private final StateManager<I> backing;
    pub backing: Arc<dyn StateManager<I>>,
    /// java: private final Function<I, O> inputToOutput;
    pub input_to_output: Arc<dyn Fn(I) -> O>,
    /// java: private final Function<O, @Nullable I> outputToInput;
    pub output_to_input: Arc<dyn Fn(O) -> std::option::Option<I>>,
}

impl<I: Clone + 'static, O: Clone + 'static> XMappedStateManager<I, O> {
    /// java: public XMappedStateManager(StateManager<I> backing, Function<I, O> inputToOutput, Function<O, @Nullable I> outputToInput)
    pub fn new(
        backing: Arc<dyn StateManager<I>>,
        input_to_output: Arc<dyn Fn(I) -> O>,
        output_to_input: Arc<dyn Fn(O) -> std::option::Option<I>>,
    ) -> Self {
        Self { backing, input_to_output, output_to_input }
    }
}

impl<I: Clone + 'static, O: Clone + 'static> StateManager<O> for XMappedStateManager<I, O> {
    /// java: @Override public void set(O value)
    fn set(&self, value: O) {
        let input = (self.output_to_input)(value);
        if let Some(input) = input {
            self.backing.set(input);
        }
    }

    /// java: @Override public O get()
    fn get(&self) -> O {
        (self.input_to_output)(self.backing.get())
    }

    /// java: @Override public void apply()
    fn apply(&self) {
        self.backing.apply();
    }

    /// java: @Override public void resetToDefault(ResetAction action)
    fn reset_to_default(&self, action: ResetAction) {
        self.backing.reset_to_default(action);
    }

    /// java: @Override public void sync()
    fn sync(&self) {
        self.backing.sync();
    }

    /// java: @Override public boolean isSynced()
    fn is_synced(&self) -> bool {
        self.backing.is_synced()
    }

    /// java: @Override public boolean isAlwaysSynced()
    fn is_always_synced(&self) -> bool {
        self.backing.is_always_synced()
    }

    /// java: @Override public boolean isDefault()
    fn is_default(&self) -> bool {
        self.backing.is_default()
    }

    /// java: @Override public void addListener(StateListener<O> stateListener)
    fn add_listener(&self, state_listener: Arc<dyn StateListener<O>>) {
        let input_to_output = self.input_to_output.clone();
        self.backing.add_listener(Arc::new(move |old_value: I, new_value: I| {
            state_listener.on_state_change(input_to_output(old_value), input_to_output(new_value))
        }));
    }
}
