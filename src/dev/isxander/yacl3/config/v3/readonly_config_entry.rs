//! Ported from dev.isxander.yacl3.config.v3.ReadonlyConfigEntry by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::dynamic_ops::Ops;
use mcrust::com::mojang::serialization::record_builder::RecordBuilder;
use mcrust::com::mojang::serialization::Obj;

/// java: `ReadonlyConfigEntry<?>` - the members that do not mention `T` (what `CodecConfig`'s
/// `List<ReadonlyConfigEntry<?>> entries` calls). `ReadonlyConfigEntry<T>: AnyReadonlyConfigEntry`, so an
/// `Arc<dyn ConfigEntry<T>>` widens to `Arc<dyn AnyReadonlyConfigEntry>` where Java widens to `<?>`.
///
/// The port's `DynamicOps` is not generic (`Obj` is every format's value, `Ops` the format), so Java's
/// `<R> RecordBuilder<R> encode(DynamicOps<R>, RecordBuilder<R>)` adds to the builder it is lent (the port's
/// `MapEncoder.encode` does the same) and `<R> boolean decode(R, DynamicOps<R>)` takes an `Obj`.
pub trait AnyReadonlyConfigEntry {
    /// java: String fieldName();
    fn field_name(&self) -> String;

    /// java: <R> RecordBuilder<R> encode(DynamicOps<R> ops, RecordBuilder<R> recordBuilder);
    fn encode(&self, ops: &Ops, record_builder: &mut dyn RecordBuilder);

    /// java: <R> boolean decode(R encoded, DynamicOps<R> ops);
    fn decode(&self, encoded: &Obj, ops: &Ops) -> bool;
}

/// java: @ApiStatus.Experimental public interface ReadonlyConfigEntry<T>
///
/// `&self` methods: an entry is shared (`Arc`) by the config that registered it and every reader.
pub trait ReadonlyConfigEntry<T: Clone + 'static>: AnyReadonlyConfigEntry {
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: T get();
    fn get(&self) -> T;

    /// java: ReadonlyConfigEntry<T> modifyGet(UnaryOperator<T> modifier);
    fn modify_get(&self, modifier: Arc<dyn Fn(T) -> T>) -> Arc<dyn ReadonlyConfigEntry<T>>;

    /// java: default ReadonlyConfigEntry<T> onGet(Consumer<T> consumer)
    fn on_get(&self, consumer: Arc<dyn Fn(T)>) -> Arc<dyn ReadonlyConfigEntry<T>> {
        self.modify_get(Arc::new(move |v: T| {
            consumer(v.clone());
            v
        }))
    }
}
