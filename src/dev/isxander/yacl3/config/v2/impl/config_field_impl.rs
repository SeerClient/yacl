//! Ported from dev.isxander.yacl3.config.v2.impl.ConfigFieldImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v2::api::autogen::auto_gen::AutoGen;
use crate::dev::isxander::yacl3::config::v2::api::autogen::auto_gen_field::AutoGenField;
use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::ConfigClassHandler;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::api::field_access::FieldAccess;
use crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess;
use crate::dev::isxander::yacl3::config::v2::api::serial_entry::SerialEntry;
use crate::dev::isxander::yacl3::config::v2::api::serial_field::SerialField;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::ReflectionFieldAccess;

/// java: public class ConfigFieldImpl<T> implements ConfigField<T>
pub struct ConfigFieldImpl<T: 'static> {
    /// java: private ReflectionFieldAccess<T> field; (replaced by `setFieldAccess` when a load succeeds)
    pub field: std::cell::RefCell<Arc<ReflectionFieldAccess<T>>>,
    /// java: private final ReflectionFieldAccess<T> defaultField;
    pub default_field: Arc<ReflectionFieldAccess<T>>,
    /// java: private final ConfigClassHandler<?> parent;
    pub parent: Arc<dyn ConfigClassHandler<compat::Object>>,
    /// java: private final Optional<SerialField> serial;
    pub serial: Option<Arc<dyn SerialField>>,
    /// java: private final Optional<AutoGenField> autoGen;
    pub auto_gen: Option<Arc<dyn AutoGenField>>,
}

impl<T: Clone + Send + Sync + 'static> ConfigFieldImpl<T> {
    /// java: public ConfigFieldImpl(ReflectionFieldAccess<T> field, ReflectionFieldAccess<T> defaultField, ConfigClassHandler<?> parent, @Nullable SerialEntry config, @Nullable SerialEntry inheritedConfig, @Nullable AutoGen autoGen)
    pub fn new(
        field: ReflectionFieldAccess<T>,
        default_field: ReflectionFieldAccess<T>,
        parent: Arc<dyn ConfigClassHandler<compat::Object>>,
        config: Option<SerialEntry>,
        inherited_config: Option<SerialEntry>,
        auto_gen: Option<AutoGen>,
    ) -> Arc<Self> {
        let serial: Option<Arc<dyn SerialField>> = if let Some(config) = &config {
            Some(Arc::new(SerialFieldImpl::new(
                if "" == config.value() { field.name() } else { config.value() },
                if "" == config.comment() { None } else { Some(config.comment()) },
                config.required(),
                config.nullable(),
            )))
        } else if let Some(inherited_config) = &inherited_config {
            Some(Arc::new(SerialFieldImpl::new(field.name(), None, inherited_config.required(), inherited_config.nullable())))
        } else {
            None
        };
        let auto_gen: Option<Arc<dyn AutoGenField>> = match &auto_gen {
            Some(auto_gen) => Some(Arc::new(AutoGenFieldImpl::<T>::new(
                auto_gen.category(),
                if "" == auto_gen.group() { None } else { Some(auto_gen.group()) },
            ))),
            None => None,
        };
        Arc::new(Self {
            field: std::cell::RefCell::new(Arc::new(field)),
            default_field: Arc::new(default_field),
            parent,
            serial,
            auto_gen,
        })
    }

    /// java: @Override public ReflectionFieldAccess<T> access() - the covariant return (`ConfigClassHandlerImpl.load`
    /// reads `field.access().field()`)
    pub fn reflection_access(&self) -> Arc<ReflectionFieldAccess<T>> {
        self.field.borrow().clone()
    }

    /// java: @Override public ReflectionFieldAccess<T> defaultAccess() - the covariant return
    pub fn reflection_default_access(&self) -> Arc<ReflectionFieldAccess<T>> {
        self.default_field.clone()
    }

    /// java: public void setFieldAccess(ReflectionFieldAccess<T> field)
    pub fn set_field_access(&self, field: Arc<ReflectionFieldAccess<T>>) {
        *self.field.borrow_mut() = field;
    }
}

impl<T: Clone + Send + Sync + 'static> ConfigField<T> for ConfigFieldImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public ReflectionFieldAccess<T> access()
    fn access(&self) -> Arc<dyn FieldAccess<T>> {
        self.reflection_access()
    }

    /// java: @Override public ReflectionFieldAccess<T> defaultAccess()
    fn default_access(&self) -> Arc<dyn ReadOnlyFieldAccess<T>> {
        self.default_field.clone()
    }

    /// java: @Override public ConfigClassHandler<?> parent()
    fn parent(&self) -> Arc<dyn ConfigClassHandler<compat::Object>> {
        self.parent.clone()
    }

    /// java: @Override public Optional<SerialField> serial()
    fn serial(&self) -> Option<Arc<dyn SerialField>> {
        self.serial.clone()
    }

    /// java: @Override public Optional<AutoGenField> autoGen()
    fn auto_gen(&self) -> Option<Arc<dyn AutoGenField>> {
        self.auto_gen.clone()
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
    pub fn new(serial_name: String, comment: Option<String>, required: bool, nullable: bool) -> Self {
        Self { serial_name, comment, required, nullable }
    }
}

impl SerialField for SerialFieldImpl {
    /// java: the record accessor `serialName()`
    fn serial_name(&self) -> String {
        self.serial_name.clone()
    }

    /// java: the record accessor `comment()`
    fn comment(&self) -> Option<String> {
        self.comment.clone()
    }

    /// java: the record accessor `required()`
    fn required(&self) -> bool {
        self.required
    }

    /// java: the record accessor `nullable()`
    fn nullable(&self) -> bool {
        self.nullable
    }
}

/// java: private record AutoGenFieldImpl<T>(String category, Optional<String> group) implements AutoGenField
pub struct AutoGenFieldImpl<T> {
    /// java: String category
    pub category: String,
    /// java: Optional<String> group
    pub group: Option<String>,
    _type: std::marker::PhantomData<fn() -> T>,
}

impl<T> AutoGenFieldImpl<T> {
    /// java: the canonical record constructor
    pub fn new(category: String, group: Option<String>) -> Self {
        Self { category, group, _type: std::marker::PhantomData }
    }
}

impl<T> Clone for AutoGenFieldImpl<T> {
    fn clone(&self) -> Self {
        Self::new(self.category.clone(), self.group.clone())
    }
}

impl<T> PartialEq for AutoGenFieldImpl<T> {
    fn eq(&self, other: &Self) -> bool {
        self.category == other.category && self.group == other.group
    }
}

impl<T: 'static> AutoGenField for AutoGenFieldImpl<T> {
    /// java: the record accessor `category()`
    fn category(&self) -> String {
        self.category.clone()
    }

    /// java: the record accessor `group()`
    fn group(&self) -> Option<String> {
        self.group.clone()
    }
}
