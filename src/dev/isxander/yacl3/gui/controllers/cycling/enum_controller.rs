//! Ported from dev.isxander.yacl3.gui.controllers.cycling.EnumController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::nameable_enum::NameableEnum;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::cycling::cycling_controller_element::CyclingControllerElement;
use crate::dev::isxander::yacl3::gui::controllers::cycling::cycling_list_controller::CyclingListController;
use crate::dev::isxander::yacl3::gui::controllers::cycling::cycling_list_controller::CyclingListControllerBase;
use crate::dev::isxander::yacl3::gui::controllers::cycling::i_cycling_controller::ICyclingController;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use crate::dev::isxander::yacl3::api::option::Option as _;

/// java: public class EnumController<T extends Enum<T>> extends CyclingListController<T>
#[derive(Clone)]
pub struct EnumController<T> {
    /// java: extends CyclingListController
    pub base: CyclingListControllerBase<T>,
}

impl<T: 'static> std::ops::Deref for EnumController<T> {
    type Target = CyclingListControllerBase<T>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for EnumController<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> EnumController<T> {
    /// java: public EnumController(Option<T> option, Class<T> enumClass)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>, mut enum_class: compat::ClassOf<T>) -> Self {
        // TODO(fabric2seer): `.getEnumConstants(..)` on a Class
        //   java: this(option, getDefaultFormatter(), enumClass.getEnumConstants());
        let mut this: Self = todo!("constructor");
        this
    }

    /// java: public EnumController(Option<T> option, Function<T, Component> valueFormatter, T[] availableValues)
    pub fn new_3(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>, value_formatter: Arc<dyn Fn(T) -> MutableComponent>, mut available_values: Vec<T>) -> Self {
        let mut this = Self {
            base: CyclingListControllerBase::<T>::new_3(option.clone(), available_values.to_vec(), value_formatter.clone()),
        };
        this
    }

    /// java: public static <T extends Enum<T>> Function<T, Component> getDefaultFormatter()
    pub fn get_default_formatter<T: 'static>() -> Arc<dyn Fn(T) -> MutableComponent> {
        return Arc::new(move |value| {
            if let Some(nameable_enum) = value.as_any().downcast_ref::<NameableEnum>() {
                return (nameable_enum)();
            }
            return component::literal(&compat::jstring(&value));
        });
    }

    /// java: public static <T extends Enum<T>> EnumController<T> createInternal(Option<T> option, ValueFormatter<T> formatter, T[] values)
    pub fn create_internal<T: 'static>(option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>, formatter: &compat::Shared<dyn ValueFormatter<T>>, values: &Vec<T>) -> EnumController<T> {
        return EnumController::new_3(option.clone(), Arc::new(move |__a0| formatter.lock().borrow().format(&__a0)), values.clone());
    }

}

impl<T: Clone + 'static> CyclingListController<T> for EnumController<T> {
    fn cycling_list_controller_base(&self) -> &CyclingListControllerBase<T> {
        &self.base
    }
    fn cycling_list_controller_base_mut(&mut self) -> &mut CyclingListControllerBase<T> {
        &mut self.base
    }

}

impl<T: Clone + 'static> ICyclingController<compat::Object> for EnumController<T> {
    /// java: @Override public void setPendingValue(int ordinal)
    fn set_pending_value(&self, mut ordinal: i32) {
        { let __s0 = self.values[(ordinal) as usize].clone(); self.option().lock().borrow_mut().request_set(__s0) };
    }

    /// java: @Override public int getPendingValue()
    fn get_pending_value(&self) -> i32 {
        return self.values.iter().position(|e| *e == self.option().lock().borrow().pending_value()).map_or(-1, |i| i as i32);
    }

    /// java: @Override public int getCycleLength()
    fn get_cycle_length(&self) -> i32 {
        return (self.values.len() as i32);
    }

}

impl<T: Clone + 'static> Controller<compat::Object> for EnumController<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<T> option()
    fn option(&self) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>> {
        return self.option.clone().unwrap();
    }

    /// java: @Override public Component formatValue()
    fn format_value(&self) -> MutableComponent {
        return { let (__a0,) = (&self.option().lock().borrow().pending_value(),); self.value_formatter.as_ref().unwrap().lock().borrow().format(__a0) };
    }

    /// java: @Override default AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(&self, screen: &compat::Shared<YACLScreen>, widget_dimension: &compat::Shared<dyn Dimension<i32>>) -> compat::Shared<dyn AbstractWidget> {
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return new CyclingControllerElement(this, screen, widgetDimension);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
