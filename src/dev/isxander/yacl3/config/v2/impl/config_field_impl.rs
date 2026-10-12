//! Ported from dev.isxander.yacl3.config.v2.impl.ConfigFieldImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v2::api::autogen::auto_gen::AutoGen;
use crate::dev::isxander::yacl3::config::v2::api::autogen::auto_gen_field::AutoGenField;
use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::ConfigClassHandler;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::api::serial_entry::SerialEntry;
use crate::dev::isxander::yacl3::config::v2::api::serial_field::SerialField;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::ReflectionFieldAccess;

/// java: public class ConfigFieldImpl<T> implements ConfigField<T>
pub struct ConfigFieldImpl<T> {
    /// java: private ReflectionFieldAccess<T> field;
    pub field: ReflectionFieldAccess<T>,
    /// java: private final ReflectionFieldAccess<T> defaultField;
    pub default_field: ReflectionFieldAccess<T>,
    /// java: private final ConfigClassHandler<?> parent;
    pub parent: compat::Shared<dyn ConfigClassHandler<compat::Object>>,
    /// java: private final Optional<SerialField> serial;
    pub serial: Option<compat::Shared<dyn SerialField>>,
    /// java: private final Optional<AutoGenField> autoGen;
    pub auto_gen: Option<compat::Shared<dyn AutoGenField>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl<T: Clone + 'static> ConfigFieldImpl<T> {
    /// java: public ConfigFieldImpl( ReflectionFieldAccess<T> field, ReflectionFieldAccess<T> defaultField, ConfigClassHandler<?> parent, @Nullable SerialEntry config, @Nullable SerialEntry inheritedConfig, @Nulla
    pub fn new(mut field: ReflectionFieldAccess<T>, mut default_field: ReflectionFieldAccess<T>, mut parent: compat::Shared<dyn ConfigClassHandler<compat::Object>>, mut config: Option<SerialEntry>, mut inherited_config: Option<SerialEntry>, mut auto_gen: Option<AutoGen>) -> compat::Shared<Self> {
        let __init_field = field.clone();
        let __init_default_field = default_field.clone();
        let __init_parent = parent.clone();
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            field: __init_field,
            default_field: __init_default_field,
            parent: __init_parent,
            serial: Default::default(),
            auto_gen: Default::default(),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        if let Some(config) = &config {
            { let __v = Some(SerialFieldImpl::new((if "" == config.value() { field.name() } else { config.value() }), (if "" == config.comment() { None } else { Some(config.comment()) }), config.required(), config.nullable())); compat::write(&__this, |__t| __t.serial = __v); }
        } else {
            if let Some(inherited_config) = &inherited_config {
                { let __v = Some(SerialFieldImpl::new(field.name(), None, inherited_config.required(), inherited_config.nullable())); compat::write(&__this, |__t| __t.serial = __v); }
            } else {
                { let __v = None; compat::write(&__this, |__t| __t.serial = __v); }
            }
        }
        { let __v = (if auto_gen.is_some() { Some(AutoGenFieldImpl::new(auto_gen.as_ref().unwrap().category(), (if "" == auto_gen.as_ref().unwrap().group() { None } else { Some(auto_gen.as_ref().unwrap().group()) }))) } else { None }); compat::write(&__this, |__t| __t.auto_gen = __v); }
        __this
    }

    /// java: public void setFieldAccess(ReflectionFieldAccess<T> field)
    pub fn set_field_access(__this: &compat::Shared<Self>, mut field: ReflectionFieldAccess<T>) {
        { let __v = field.clone(); compat::write(__this, |__t| __t.field = __v); }
    }

}

impl<T: Clone + 'static> ConfigField<T> for ConfigFieldImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public ReflectionFieldAccess<T> access()
    fn access(&self) -> ReflectionFieldAccess<T> {
        return self.field.clone();
    }

    /// java: @Override public ReflectionFieldAccess<T> defaultAccess()
    fn default_access(&self) -> ReflectionFieldAccess<T> {
        return self.default_field.clone();
    }

    /// java: @Override public ConfigClassHandler<?> parent()
    fn parent(&self) -> compat::Shared<dyn ConfigClassHandler<compat::Object>> {
        return self.parent.clone();
    }

    /// java: @Override public Optional<SerialField> serial()
    fn serial(&self) -> Option<compat::Shared<dyn SerialField>> {
        return self.serial.clone();
    }

    /// java: @Override public Optional<AutoGenField> autoGen()
    fn auto_gen(&self) -> Option<compat::Shared<dyn AutoGenField>> {
        return self.auto_gen.clone();
    }

}

/// java: private record SerialFieldImpl(String serialName, Optional<String> comment, boolean required, boolean nullable) implements SerialField
#[derive(Clone, Debug, PartialEq)]
pub struct SerialFieldImpl {
    /// java: String serialName
    pub serial_name: String,
    /// java: Optional<String> comment
    pub comment: Option<String>,
    /// java: boolean required
    pub required: bool,
    /// java: boolean nullable
    pub nullable: bool,
}

impl SerialFieldImpl {
    /// java: the canonical record constructor
    pub fn new(mut serial_name: String, mut comment: Option<String>, mut required: bool, mut nullable: bool) -> Self {
        Self { serial_name, comment, required, nullable }
    }

    /// java: the record accessor `serialName()`
    pub fn serial_name(&self) -> String {
        self.serial_name.clone()
    }

    /// java: the record accessor `comment()`
    pub fn comment(&self) -> Option<String> {
        self.comment.clone()
    }

    /// java: the record accessor `required()`
    pub fn required(&self) -> bool {
        self.required
    }

    /// java: the record accessor `nullable()`
    pub fn nullable(&self) -> bool {
        self.nullable
    }

}

impl SerialField for SerialFieldImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: private record AutoGenFieldImpl<T>(String category, Optional<String> group) implements AutoGenField
#[derive(Clone, Debug, PartialEq)]
pub struct AutoGenFieldImpl<T> {
    /// java: String category
    pub category: String,
    /// java: Optional<String> group
    pub group: Option<String>,
}

impl<T: Clone + 'static> AutoGenFieldImpl<T> {
    /// java: the canonical record constructor
    pub fn new(mut category: String, mut group: Option<String>) -> Self {
        Self { category, group }
    }

    /// java: the record accessor `category()`
    pub fn category(&self) -> String {
        self.category.clone()
    }

    /// java: the record accessor `group()`
    pub fn group(&self) -> Option<String> {
        self.group.clone()
    }

}

impl<T: Clone + 'static> AutoGenField for AutoGenFieldImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}
