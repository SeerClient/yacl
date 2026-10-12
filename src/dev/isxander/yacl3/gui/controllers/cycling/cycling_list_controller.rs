//! Ported from dev.isxander.yacl3.gui.controllers.cycling.CyclingListController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::cycling::cycling_controller_element::CyclingControllerElement;
use crate::dev::isxander::yacl3::gui::controllers::cycling::i_cycling_controller::ICyclingController;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use crate::dev::isxander::yacl3::api::option::Option as _;

/// java: public class CyclingListController<T> implements ICyclingController<T>
#[derive(Clone)]
pub struct CyclingListControllerBase<T> {
    /// java: private final Option<T> option;
    pub option: Option<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>>,
    /// java: private final ValueFormatter<T> valueFormatter;
    pub value_formatter: Option<compat::Shared<dyn ValueFormatter<T>>>,
    /// java: private final ImmutableList<T> values;
    pub values: Vec<T>,
}

impl<T: Clone + 'static> CyclingListControllerBase<T> {
    /// java: public CyclingListController(Option<T> option, Iterable<? extends T> values)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>, mut values: Vec<T>) -> Self {
        let mut this = Self::new_3(option.clone(), values.clone(), Arc::new(move |value| component::literal(&compat::jstring(&value))));
        this
    }

    /// java: public CyclingListController(Option<T> option, Iterable<? extends T> values, Function<T, Component> valueFormatter)
    pub fn new_3(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>, mut values: Vec<T>, value_formatter: Arc<dyn Fn(T) -> MutableComponent>) -> Self {
        let __init_option = Some(option.clone());
        let __init_value_formatter = Some(Arc::new(move |__a0| (value_formatter)(__a0.clone())));
        let __init_values = values.iter().cloned().collect::<Vec<_>>();
        let mut this = Self {
            option: __init_option,
            value_formatter: __init_value_formatter,
            values: __init_values,
        };
        this
    }

    /// java: @ApiStatus.Internal public static <T> CyclingListController<T> createInternal(Option<T> option, Iterable<? extends T> values, ValueFormatter<T> formatter)
    pub fn create_internal<T: 'static>(option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>, values: &Vec<T>, formatter: &compat::Shared<dyn ValueFormatter<T>>) -> compat::Shared<dyn CyclingListController<T>> {
        return (compat::shared(CyclingListControllerBase::new_3(option.clone(), values.clone(), Arc::new(move |__a0| formatter.lock().borrow().format(&__a0)))) as compat::Shared<dyn CyclingListController<compat::Object>>);
    }

}

/// java: the virtual methods of CyclingListController (implemented by CyclingListControllerBase and every subclass)
pub trait CyclingListController<T: Clone + 'static>: ICyclingController {
    fn cycling_list_controller_base(&self) -> &CyclingListControllerBase<T>;
    fn cycling_list_controller_base_mut(&mut self) -> &mut CyclingListControllerBase<T>;

}

impl<T: Clone + 'static> CyclingListController<T> for CyclingListControllerBase<T> {
    fn cycling_list_controller_base(&self) -> &CyclingListControllerBase<T> {
        self
    }
    fn cycling_list_controller_base_mut(&mut self) -> &mut CyclingListControllerBase<T> {
        self
    }

}

impl<T: Clone + 'static> ICyclingController<T> for CyclingListControllerBase<T> {
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

impl<T: Clone + 'static> Controller<compat::Object> for CyclingListControllerBase<T> {
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
