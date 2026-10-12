//! Ported from dev.isxander.yacl3.config.v2.api.ConfigField by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v2::api::autogen::auto_gen_field::AutoGenField;
use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::ConfigClassHandler;
use crate::dev::isxander::yacl3::config::v2::api::field_access::FieldAccess;
use crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess;
use crate::dev::isxander::yacl3::config::v2::api::serial_field::SerialField;

/// java: public interface ConfigField<T>
///
/// A field in a config class: its metadata, and access to it and its default value. A Java `ConfigField<?>` is
/// `Arc<dyn ConfigField<compat::Object>>` (the value as Java's `Object`).
pub trait ConfigField<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: FieldAccess<T> access();
    fn access(&self) -> Arc<dyn FieldAccess<T>>;

    /// java: ReadOnlyFieldAccess<T> defaultAccess();
    fn default_access(&self) -> Arc<dyn ReadOnlyFieldAccess<T>>;

    /// java: ConfigClassHandler<?> parent();
    fn parent(&self) -> Arc<dyn ConfigClassHandler<compat::Object>>;

    /// java: Optional<SerialField> serial();
    fn serial(&self) -> Option<Arc<dyn SerialField>>;

    /// java: Optional<AutoGenField> autoGen();
    fn auto_gen(&self) -> Option<Arc<dyn AutoGenField>>;
}
