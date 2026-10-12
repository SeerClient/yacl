//! Ported from dev.isxander.yacl3.gui.controllers.slider.FloatSliderController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::slider::i_slider_controller::ISliderController;
use crate::dev::isxander::yacl3::gui::controllers::slider::slider_controller_element::SliderControllerElement;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use crate::dev::isxander::yacl3::api::option::Option as _;

/// java: public static final Function<Float, Component> DEFAULT_FORMATTER = value -> Component.literal(String.format("%,.1f", value).replaceAll("[\u00a0\u202F]", " "));
pub static DEFAULT_FORMATTER: compat::MainThread<Arc<dyn Fn(f32) -> MutableComponent>> = compat::MainThread::new(|| Arc::new(move |value| component::literal(&compat::regex::replace_all(&(compat::string_format("%,.1f", &[compat::FormatArg::Float((value) as f64)])), "[  ]", " "))));

/// java: public class FloatSliderController implements ISliderController<Float>
#[derive(Clone)]
pub struct FloatSliderController {
    /// java: private final Option<Float> option;
    pub option: Option<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<f32>>>,
    /// java: private final float min, max, interval;
    pub min: f32,
    /// java: private final float min, max, interval;
    pub max: f32,
    /// java: private final float min, max, interval;
    pub interval: f32,
    /// java: private final ValueFormatter<Float> valueFormatter;
    pub value_formatter: Option<compat::Shared<dyn ValueFormatter<f32>>>,
}

impl FloatSliderController {
    /// java: public FloatSliderController(Option<Float> option, float min, float max, float interval)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<f32>>, mut min: f32, mut max: f32, mut interval: f32) -> Self {
        let mut this = Self::new_5(option.clone(), min, max, interval, DEFAULT_FORMATTER.get_mut().clone());
        this
    }

    /// java: public FloatSliderController(Option<Float> option, float min, float max, float interval, Function<Float, Component> valueFormatter)
    pub fn new_5(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<f32>>, mut min: f32, mut max: f32, mut interval: f32, value_formatter: Arc<dyn Fn(f32) -> MutableComponent>) -> Self {
        let mut this = Self {
            option: None,
            min: 0.0,
            max: 0.0,
            interval: 0.0,
            value_formatter: None,
        };
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(max > min, "`max` cannot be smaller than `min`");
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(interval > 0, "`interval` must be more than 0");
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(valueFormatter, "`valueFormatter` must not be null");
        this.option = Some(option.clone());
        this.min = min;
        this.max = max;
        this.interval = interval;
        this.value_formatter = Some(Arc::new(move |__a0| (value_formatter)(__a0.clone())));
        this
    }

    /// java: public static FloatSliderController createInternal(Option<Float> option, float min, float max, float interval, ValueFormatter<Float> formatter)
    pub fn create_internal(option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<f32>>, mut min: f32, mut max: f32, mut interval: f32, formatter: &compat::Shared<dyn ValueFormatter<f32>>) -> FloatSliderController {
        return FloatSliderController::new_5(option.clone(), min, max, interval, Arc::new(move |__a0| formatter.lock().borrow().format(&__a0)));
    }

}

impl ISliderController<f32> for FloatSliderController {
    /// java: @Override public double min()
    fn min(&self) -> f64 {
        return (self.min) as f64;
    }

    /// java: @Override public double max()
    fn max(&self) -> f64 {
        return (self.max) as f64;
    }

    /// java: @Override public double interval()
    fn interval(&self) -> f64 {
        return (self.interval) as f64;
    }

    /// java: @Override public void setPendingValue(double value)
    fn set_pending_value(&self, mut value: f64) {
        { let (__a0,) = (((value) as f32),); self.option().lock().borrow_mut().request_set(__a0) };
    }

    /// java: @Override public double pendingValue()
    fn pending_value(&self) -> f64 {
        return self.option().lock().borrow().pending_value();
    }

}

impl Controller<compat::Object> for FloatSliderController {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<Float> option()
    fn option(&self) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<f32>> {
        return self.option.clone().unwrap();
    }

    /// java: @Override public Component formatValue()
    fn format_value(&self) -> MutableComponent {
        return { let (__a0,) = (&self.option().lock().borrow().pending_value(),); self.value_formatter.as_ref().unwrap().lock().borrow().format(__a0) };
    }

    /// java: @Override default AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(&self, screen: &compat::Shared<YACLScreen>, widget_dimension: &compat::Shared<dyn Dimension<i32>>) -> compat::Shared<dyn AbstractWidget> {
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return new SliderControllerElement(this, screen, widgetDimension, min(), max(), interval());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
