//! Ported from dev.isxander.yacl3.config.v2.api.autogen.Boolean by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface Boolean
///
/// An option factory: a regular option with a `BooleanControllerBuilder` controller.
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq)]
pub struct Boolean {
    /// java: Formatter formatter() default Formatter.TRUE_FALSE;
    pub formatter: Formatter,
    /// java: boolean colored() default false;
    pub colored: bool,
}

impl Default for Boolean {
    fn default() -> Self {
        Boolean::of()
    }
}

impl Boolean {
    /// The annotation as written with no elements: every element takes its declared default.
    pub fn of() -> Boolean {
        Boolean { formatter: Formatter::TrueFalse, colored: false }
    }

    /// java: Formatter formatter() default Formatter.TRUE_FALSE;
    pub fn formatter(&self) -> Formatter {
        self.formatter
    }

    /// java: boolean colored() default false;
    pub fn colored(&self) -> bool {
        self.colored
    }
}

/// java: enum Formatter
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Formatter {
    YesNo,
    TrueFalse,
    OnOff,
    /// Uses the translation keys `yacl3.config.$configId.$fieldName.fmt.true` / `.false`.
    Custom,
}

impl Formatter {
    pub const VALUES: [Formatter; 4] = [Formatter::YesNo, Formatter::TrueFalse, Formatter::OnOff, Formatter::Custom];

    /// java: values()
    pub fn values() -> Vec<Formatter> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Formatter::YesNo => "YES_NO",
            Formatter::TrueFalse => "TRUE_FALSE",
            Formatter::OnOff => "ON_OFF",
            Formatter::Custom => "CUSTOM",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Formatter {
        match name {
            "YES_NO" => Formatter::YesNo,
            "TRUE_FALSE" => Formatter::TrueFalse,
            "ON_OFF" => Formatter::OnOff,
            "CUSTOM" => Formatter::Custom,
            _ => panic!("IllegalArgumentException: No enum constant Boolean.Formatter.{name}"),
        }
    }
}

impl std::fmt::Display for Formatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
