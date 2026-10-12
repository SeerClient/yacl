//! Ported from dev.isxander.yacl3.config.ConfigInstance by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Deprecated public abstract class ConfigInstance<T>
pub struct ConfigInstanceBase<T> {
    /// java: private final Class<T> configClass;
    pub config_class: compat::ClassOf<T>,
    /// java: private final T defaultInstance;
    pub default_instance: Option<T>,
    /// java: private T instance;
    pub instance: Option<T>,
}

impl<T: Clone + 'static> ConfigInstanceBase<T> {
    /// java: public ConfigInstance(Class<T> configClass)
    pub fn new(mut config_class: compat::ClassOf<T>) -> Self {
        let __init_config_class = config_class.clone();
        let mut this = Self {
            config_class: __init_config_class,
            default_instance: None,
            instance: None,
        };
        // java: try {..} catch (InstantiationException | IllegalAccessException | NoSuchMethodException | InvocationTargetException e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): an assignment used as a value
            //   java: this.defaultInstance = this.instance = configClass.getConstructor().newInstance();
        }));
        if let Err(e) = __result {
            panic!("IllegalStateException: {}", format!("Could not create default instance of config for {}. Make sure there is a default constructor!", this.config_class.get_simple_name()).to_string());
        }
        this
    }

}

/// java: the virtual methods of ConfigInstance (implemented by ConfigInstanceBase and every subclass)
pub trait ConfigInstance<T: Clone + 'static> {
    fn config_instance_base(&self) -> &ConfigInstanceBase<T>;
    fn config_instance_base_mut(&mut self) -> &mut ConfigInstanceBase<T>;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public abstract void save();
    fn save(&self);

    /// java: public abstract void load();
    fn load(&mut self);

    /// java: public T getConfig()
    fn get_config(&self) -> T {
        return self.config_instance_base().instance.clone().unwrap();
    }

    /// java: protected void setConfig(T instance)
    fn set_config(&mut self, mut instance: T) {
        self.config_instance_base_mut().instance = Some(instance.clone());
    }

    /// java: public T getDefaults()
    fn get_defaults(&self) -> T {
        return self.config_instance_base().default_instance.clone().unwrap();
    }

    /// java: public Class<T> getConfigClass()
    fn get_config_class(&self) -> compat::ClassOf<T> {
        return self.config_instance_base().config_class.clone();
    }

}
