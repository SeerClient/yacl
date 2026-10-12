//! Ported from dev.isxander.yacl3.impl.XMappedStateManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::state_manager::ResetAction;
use crate::dev::isxander::yacl3::api::state_manager::StateListener;
use crate::dev::isxander::yacl3::api::state_manager::StateManager;

/// java: public class XMappedStateManager<I, O> implements StateManager<O>
#[derive(Clone)]
pub struct XMappedStateManager<I, O> {
    /// java: private final StateManager<I> backing;
    pub backing: compat::Shared<dyn StateManager<I>>,
    /// java: private final Function<I, O> inputToOutput;
    pub input_to_output: Arc<dyn Fn(I) -> O>,
    /// java: private final Function<O, @Nullable I> outputToInput;
    pub output_to_input: Arc<dyn Fn(O) -> Option<I>>,
}

impl<I: Clone + 'static, O: Clone + 'static> XMappedStateManager<I, O> {
    /// java: public XMappedStateManager( StateManager<I> backing, Function<I, O> inputToOutput, Function<O, @Nullable I> outputToInput )
    pub fn new(mut backing: compat::Shared<dyn StateManager<I>>, input_to_output: Arc<dyn Fn(I) -> O>, output_to_input: Arc<dyn Fn(O) -> Option<I>>) -> Self {
        let __init_backing = backing.clone();
        let __init_input_to_output = input_to_output.clone();
        let __init_output_to_input = output_to_input.clone();
        let mut this = Self {
            backing: __init_backing,
            input_to_output: __init_input_to_output,
            output_to_input: __init_output_to_input,
        };
        this
    }

}

impl<I: Clone + 'static, O: Clone + 'static> StateManager<O> for XMappedStateManager<I, O> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void set(O value)
    fn set(&mut self, mut value: O) {
        let mut input = (self.output_to_input)(value.clone());
        if let Some(input) = &input {
            { let (__a0,) = (input.clone(),); self.backing.lock().borrow_mut().set(__a0) };
        }
    }

    /// java: @Override public O get()
    fn get(&self) -> O {
        return (self.input_to_output)(self.backing.lock().borrow().get());
    }

    /// java: @Override public void apply()
    fn apply(&mut self) {
        self.backing.lock().borrow_mut().apply();
    }

    /// java: @Override public void resetToDefault(ResetAction action)
    fn reset_to_default(&mut self, mut action: ResetAction) {
        self.backing.lock().borrow_mut().reset_to_default(action);
    }

    /// java: @Override public void sync()
    fn sync(&mut self) {
        self.backing.lock().borrow_mut().sync();
    }

    /// java: @Override public boolean isSynced()
    fn is_synced(&self) -> bool {
        return self.backing.lock().borrow().is_synced();
    }

    /// java: @Override public boolean isAlwaysSynced()
    fn is_always_synced(&self) -> bool {
        return self.backing.lock().borrow().is_always_synced();
    }

    /// java: @Override public boolean isDefault()
    fn is_default(&self) -> bool {
        return self.backing.lock().borrow().is_default();
    }

    /// java: @Override public void addListener(StateListener<O> stateListener)
    fn add_listener(&mut self, state_listener: &compat::Shared<dyn StateListener<O>>) {
        { let __s0 = (Arc::new(move |old_value, new_value| { let (__a0, __a1,) = (&(self.input_to_output)(old_value.clone()), &(self.input_to_output)(new_value.clone()),); state_listener.lock().borrow().on_state_change(__a0, __a1) })).clone(); self.backing.lock().borrow_mut().add_listener(&__s0) };
    }

}
