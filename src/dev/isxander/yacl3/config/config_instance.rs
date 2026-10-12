//! Ported from dev.isxander.yacl3.config.ConfigInstance by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Deprecated public abstract class ConfigInstance<T> - its fields, and the bodies of its concrete methods
/// (the trait's provided methods read them).
///
/// Holds the config instance along with a final default instance. A config object is the mod's object, referenced as
/// `compat::Shared<T>` (`getConfig()` is the live object, mutated in place).
pub struct ConfigInstanceBase<T: 'static> {
    /// java: private final Class<T> configClass;
    pub config_class: compat::ClassOf<T>,
    /// java: private final T defaultInstance;
    pub default_instance: compat::Shared<T>,
    /// java: private T instance;
    pub instance: std::cell::RefCell<compat::Shared<T>>,
}

impl<T: 'static> ConfigInstanceBase<T> {
    /// java: public ConfigInstance(Class<T> configClass)
    pub fn new(config_class: compat::ClassOf<T>) -> Self {
        // java: this.defaultInstance = this.instance = configClass.getConstructor().newInstance(); - ONE object, held
        // by both fields until `setConfig`
        let created = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| config_class.new_instance()));
        let instance = match created {
            Ok(instance) => compat::shared(instance),
            Err(_) => panic!(
                "IllegalStateException: {}",
                format!(
                    "Could not create default instance of config for {}. Make sure there is a default constructor!",
                    config_class.get_simple_name()
                )
            ),
        };
        Self { config_class, default_instance: instance.clone(), instance: std::cell::RefCell::new(instance) }
    }
}

/// java: the virtual methods of ConfigInstance (implemented by every subclass)
///
/// `&self` methods: `save`/`load` of a subclass read the instance through these accessors while it runs.
pub trait ConfigInstance<T: 'static> {
    fn config_instance_base(&self) -> &ConfigInstanceBase<T>;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public abstract void save();
    fn save(&self);

    /// java: public abstract void load();
    fn load(&self);

    /// java: public T getConfig()
    fn get_config(&self) -> compat::Shared<T> {
        self.config_instance_base().instance.borrow().clone()
    }

    /// java: protected void setConfig(T instance)
    fn set_config(&self, instance: compat::Shared<T>) {
        *self.config_instance_base().instance.borrow_mut() = instance;
    }

    /// java: public T getDefaults()
    fn get_defaults(&self) -> compat::Shared<T> {
        self.config_instance_base().default_instance.clone()
    }

    /// java: public Class<T> getConfigClass()
    fn get_config_class(&self) -> compat::ClassOf<T> {
        self.config_instance_base().config_class
    }
}
