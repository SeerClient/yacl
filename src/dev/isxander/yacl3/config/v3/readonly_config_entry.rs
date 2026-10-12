//! Ported from dev.isxander.yacl3.config.v3.ReadonlyConfigEntry by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::dynamic_ops::DynamicOps;
use mcrust::com::mojang::serialization::record_builder::RecordBuilder;

/// java: @ApiStatus.Experimental public interface ReadonlyConfigEntry<T>
pub trait ReadonlyConfigEntry<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: String fieldName();
    fn field_name(&self) -> String;

    /// java: T get();
    fn get(&self) -> T;

    /// java: ReadonlyConfigEntry<T> modifyGet(UnaryOperator<T> modifier);
    fn modify_get(&mut self, modifier: Arc<dyn Fn(T) -> T>) -> compat::Shared<dyn ReadonlyConfigEntry<T>>;

    /// java: default ReadonlyConfigEntry<T> onGet(Consumer<T> consumer)
    fn on_get(&self, consumer: Arc<dyn Fn(T)>) -> compat::Shared<dyn ReadonlyConfigEntry<T>> {
        return self.modify_get(Arc::new(move |v| {
            (consumer)(v.clone());
            return v;
        }));
    }

    /// java: <R> RecordBuilder<R> encode(DynamicOps<R> ops, RecordBuilder<R> recordBuilder);
    fn encode<R: 'static>(&self, ops: &Arc<dyn DynamicOps<R>>, record_builder: &dyn RecordBuilder<R>) -> Box<dyn RecordBuilder<R>> where Self: Sized;

    /// java: <R> boolean decode(R encoded, DynamicOps<R> ops);
    fn decode<R: 'static>(&mut self, encoded: &R, ops: &Arc<dyn DynamicOps<R>>) -> bool where Self: Sized;

}
