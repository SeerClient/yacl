//! Ported from dev.isxander.yacl3.api.OptionEventListener.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;

/// java: @FunctionalInterface public interface OptionEventListener<T>
///
/// Any `Fn(&dyn Option<T>, Event)` is one. The option is handed as the reference the call borrows (Java's `this`
/// of the option that fires); keep its `Arc` from where it was built if the listener must hold on to it.
pub trait OptionEventListener<T: Clone + 'static> {
    /// java: void onEvent(Option<T> option, Event event);
    fn on_event(&self, option: &dyn YOption<T>, event: Event);
}

impl<T: Clone + 'static, F: Fn(&dyn YOption<T>, Event)> OptionEventListener<T> for F {
    fn on_event(&self, option: &dyn YOption<T>, event: Event) {
        self(option, event)
    }
}

/// java: enum Event
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Event {
    Initial,
    StateChange,
    AvailabilityChange,
    Other,
}

impl Event {
    pub const VALUES: [Event; 4] = [Event::Initial, Event::StateChange, Event::AvailabilityChange, Event::Other];

    /// java: values()
    pub fn values() -> Vec<Event> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Event::Initial => "INITIAL",
            Event::StateChange => "STATE_CHANGE",
            Event::AvailabilityChange => "AVAILABILITY_CHANGE",
            Event::Other => "OTHER",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Event {
        match name {
            "INITIAL" => Event::Initial,
            "STATE_CHANGE" => Event::StateChange,
            "AVAILABILITY_CHANGE" => Event::AvailabilityChange,
            "OTHER" => Event::Other,
            _ => panic!("IllegalArgumentException: No enum constant OptionEventListener.Event.{name}"),
        }
    }
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
