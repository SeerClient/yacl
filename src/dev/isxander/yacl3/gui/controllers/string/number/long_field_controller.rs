//! Ported from dev.isxander.yacl3.gui.controllers.string.number.LongFieldController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::slider::i_slider_controller::ISliderController;
use crate::dev::isxander::yacl3::gui::controllers::slider::long_slider_controller;
use crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::IStringController;
use crate::dev::isxander::yacl3::gui::controllers::string::number::number_field_controller;
use crate::dev::isxander::yacl3::gui::controllers::string::number::number_field_controller::NumberFieldController;
use crate::dev::isxander::yacl3::gui::controllers::string::number::number_field_controller::NumberFieldControllerBase;
use crate::dev::isxander::yacl3::gui::controllers::string::string_controller_element::StringControllerElementBase;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use crate::dev::isxander::yacl3::api::option::Option as _;

/// java: public class LongFieldController extends NumberFieldController<Long>
#[derive(Clone)]
pub struct LongFieldController {
    /// java: extends NumberFieldController
    pub base: NumberFieldControllerBase<i64>,
    /// java: private final long min, max;
    pub min: i64,
    /// java: private final long min, max;
    pub max: i64,
}

impl std::ops::Deref for LongFieldController {
    type Target = NumberFieldControllerBase<i64>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for LongFieldController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl LongFieldController {
    /// java: public LongFieldController(Option<Long> option, long min, long max, Function<Long, Component> formatter)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<i64>>, mut min: i64, mut max: i64, formatter: Arc<dyn Fn(i64) -> MutableComponent>) -> Self {
        let __init_min = min;
        let __init_max = max;
        let mut this = Self {
            base: NumberFieldControllerBase::<i64>::new(option.clone(), formatter.clone()),
            min: __init_min,
            max: __init_max,
        };
        this
    }

    /// java: public LongFieldController(Option<Long> option, long min, long max)
    pub fn new_3(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<i64>>, mut min: i64, mut max: i64) -> Self {
        let mut this = Self::new(option.clone(), min, max, long_slider_controller::DEFAULT_FORMATTER.get_mut().clone());
        this
    }

    /// java: public LongFieldController(Option<Long> option, Function<Long, Component> formatter)
    pub fn new_2(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<i64>>, formatter: Arc<dyn Fn(i64) -> MutableComponent>) -> Self {
        let mut this = Self::new(option.clone(), (-i64::MAX), i64::MAX, formatter.clone());
        this
    }

    /// java: public LongFieldController(Option<Long> option)
    pub fn new_1(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<i64>>) -> Self {
        let mut this = Self::new(option.clone(), (-i64::MAX), i64::MAX, long_slider_controller::DEFAULT_FORMATTER.get_mut().clone());
        this
    }

    /// java: @ApiStatus.Internal public static LongFieldController createInternal(Option<Long> option, long min, long max, ValueFormatter<Long> formatter)
    pub fn create_internal(option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<i64>>, mut min: i64, mut max: i64, formatter: &compat::Shared<dyn ValueFormatter<i64>>) -> LongFieldController {
        return LongFieldController::new(option.clone(), min, max, Arc::new(move |__a0| formatter.lock().borrow().format(&__a0)));
    }

}

impl NumberFieldController<i64> for LongFieldController {
    fn number_field_controller_base(&self) -> &NumberFieldControllerBase<i64> {
        &self.base
    }
    fn number_field_controller_base_mut(&mut self) -> &mut NumberFieldControllerBase<i64> {
        &mut self.base
    }

}

impl ISliderController<compat::Object> for LongFieldController {
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
        { let (__a0,) = (((value) as i64),); self.option().lock().borrow_mut().request_set(__a0) };
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

impl Controller<compat::Object> for LongFieldController {
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

impl IStringController<compat::Object> for LongFieldController {
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
