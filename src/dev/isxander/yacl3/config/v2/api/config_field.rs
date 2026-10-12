//! Ported from dev.isxander.yacl3.config.v2.api.ConfigField by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v2::api::autogen::auto_gen_field::AutoGenField;
use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::ConfigClassHandler;
use crate::dev::isxander::yacl3::config::v2::api::field_access::FieldAccess;
use crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess;
use crate::dev::isxander::yacl3::config::v2::api::serial_field::SerialField;

/// java: public interface ConfigField<T>
pub trait ConfigField<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: FieldAccess<T> access();
    fn access(&self) -> compat::Shared<dyn FieldAccess<T>>;

    /// java: ReadOnlyFieldAccess<T> defaultAccess();
    fn default_access(&self) -> compat::Shared<dyn ReadOnlyFieldAccess<T>>;

    /// java: ConfigClassHandler<?> parent();
    fn parent(&self) -> compat::Shared<dyn ConfigClassHandler<compat::Object>>;

    /// java: Optional<SerialField> serial();
    fn serial(&self) -> Option<compat::Shared<dyn SerialField>>;

    /// java: Optional<AutoGenField> autoGen();
    fn auto_gen(&self) -> Option<compat::Shared<dyn AutoGenField>>;

}
