//! Ported from dev.isxander.yacl3.gui.controllers.string.number.IntegerFieldController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::slider::i_slider_controller::ISliderController;
use crate::dev::isxander::yacl3::gui::controllers::slider::integer_slider_controller;
use crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::IStringController;
use crate::dev::isxander::yacl3::gui::controllers::string::number::number_field_controller;
use crate::dev::isxander::yacl3::gui::controllers::string::number::number_field_controller::NumberFieldController;
use crate::dev::isxander::yacl3::gui::controllers::string::number::number_field_controller::NumberFieldControllerBase;
use crate::dev::isxander::yacl3::gui::controllers::string::string_controller_element::StringControllerElementBase;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use crate::dev::isxander::yacl3::api::option::Option as _;

/// java: public class IntegerFieldController extends NumberFieldController<Integer>
#[derive(Clone)]
pub struct IntegerFieldController {
    /// java: extends NumberFieldController
    pub base: NumberFieldControllerBase<i32>,
    /// java: private final int min, max;
    pub min: i32,
    /// java: private final int min, max;
    pub max: i32,
}

impl std::ops::Deref for IntegerFieldController {
    type Target = NumberFieldControllerBase<i32>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for IntegerFieldController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl IntegerFieldController {
    /// java: public IntegerFieldController(Option<Integer> option, int min, int max, Function<Integer, Component> formatter)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<i32>>, mut min: i32, mut max: i32, formatter: Arc<dyn Fn(i32) -> MutableComponent>) -> Self {
        let __init_min = min;
        let __init_max = max;
        let mut this = Self {
            base: NumberFieldControllerBase::<i32>::new(option.clone(), formatter.clone()),
            min: __init_min,
            max: __init_max,
        };
        this
    }

    /// java: public IntegerFieldController(Option<Integer> option, int min, int max)
    pub fn new_3(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<i32>>, mut min: i32, mut max: i32) -> Self {
        let mut this = Self::new(option.clone(), min, max, integer_slider_controller::DEFAULT_FORMATTER.get_mut().clone());
        this
    }

    /// java: public IntegerFieldController(Option<Integer> option, Function<Integer, Component> formatter)
    pub fn new_2(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<i32>>, formatter: Arc<dyn Fn(i32) -> MutableComponent>) -> Self {
        let mut this = Self::new(option.clone(), (-i32::MAX), i32::MAX, formatter.clone());
        this
    }

    /// java: public IntegerFieldController(Option<Integer> option)
    pub fn new_1(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<i32>>) -> Self {
        let mut this = Self::new(option.clone(), (-i32::MAX), i32::MAX, integer_slider_controller::DEFAULT_FORMATTER.get_mut().clone());
        this
    }

    /// java: @ApiStatus.Internal public static IntegerFieldController createInternal(Option<Integer> option, int min, int max, ValueFormatter<Integer> formatter)
    pub fn create_internal(option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<i32>>, mut min: i32, mut max: i32, formatter: &compat::Shared<dyn ValueFormatter<i32>>) -> IntegerFieldController {
        return IntegerFieldController::new(option.clone(), min, max, Arc::new(move |__a0| formatter.lock().borrow().format(&__a0)));
    }

}

impl NumberFieldController<i32> for IntegerFieldController {
    fn number_field_controller_base(&self) -> &NumberFieldControllerBase<i32> {
        &self.base
    }
    fn number_field_controller_base_mut(&mut self) -> &mut NumberFieldControllerBase<i32> {
        &mut self.base
    }

}

impl ISliderController<compat::Object> for IntegerFieldController {
    /// java: @Override public double min()
    fn min(&self) -> f64 {
        return (self.min) as f64;
    }

    /// java: @Override public double max()
    fn max(&self) -> f64 {
        return (self.max) as f64;
    }

    /// java: @Override public void setPendingValue(double value)
    fn set_pending_value(&self, mut value: f64) {
        { let (__a0,) = (((value) as i32),); self.option().lock().borrow_mut().request_set(__a0) };
    }

    /// java: @Override public double pendingValue()
    fn pending_value(&self) -> f64 {
        return self.option().lock().borrow().pending_value();
    }

    /// java: @Override public double interval()
    fn interval(&self) -> f64 {
        return ((-1) as f64) as f64;
    }

}

impl Controller<compat::Object> for IntegerFieldController {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<T> option()
    fn option(&self) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>> {
        return self.option.clone();
    }

    /// java: @Override public Component formatValue()
    fn format_value(&self) -> MutableComponent {
        return { let (__a0,) = (&self.option().lock().borrow().pending_value(),); self.display_formatter.lock().borrow().format(__a0) };
    }

    /// java: @Override public AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(&self, screen: &compat::Shared<YACLScreen>, widget_dimension: &compat::Shared<dyn Dimension<i32>>) -> compat::Shared<dyn AbstractWidget> {
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return new StringControllerElement(this, screen, widgetDimension, false);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl IStringController<compat::Object> for IntegerFieldController {
    /// java: @Override public String getString()
    fn get_string(&self) -> String {
        // TODO(fabric2seer): `DecimalFormat.format` has no Rust method in the port (looked for `format` in DecimalFormat and JavaSrc)
        // rust: return number_field_controller::NUMBER_FORMAT.get_mut().format(&self.option().lock().borrow().pending_value());
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override public void setFromString(String value)
    fn set_from_string(&self, mut value: String) {
        // java: try {..} catch (ParseException ignore) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut transformed: String = self.transform_input(value.clone());
            // TODO(fabric2seer): `.doubleValue(..)` on a value of unknown type
            //   java: setPendingValue(Mth.clamp(NUMBER_FORMAT.parse(transformed).doubleValue(), min(), max()));
        }));
        if let Err(ignore) = __result {
            logger().warn(&format!("Failed to parse number: {}", value));
        }
    }

}
