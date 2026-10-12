//! Ported from dev.isxander.yacl3.gui.controllers.string.number.NumberFieldController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::gui::controllers::slider::i_slider_controller::ISliderController;
use crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::IStringController;
use mcrust::java::text::decimal_format::DecimalFormat;
use mcrust::java::text::decimal_format_symbols::DecimalFormatSymbols;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

// TODO(fabric2seer): static `NumberFormat.getInstance` has no Rust counterpart in the port
//   java: protected static final NumberFormat NUMBER_FORMAT = NumberFormat.getInstance();
/// java: protected static final NumberFormat NUMBER_FORMAT = NumberFormat.getInstance();
pub static NUMBER_FORMAT: compat::MainThread<DecimalFormat> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `NUMBER_FORMAT` - see the TODO above"));
// TODO(fabric2seer): `get_instance` takes 1 argument(s) in the port (`locale: Locale`), Java passes 0
//   java: private static final DecimalFormatSymbols DECIMAL_FORMAT_SYMBOLS = DecimalFormatSymbols.getInstance();
/// java: private static final DecimalFormatSymbols DECIMAL_FORMAT_SYMBOLS = DecimalFormatSymbols.getInstance();
pub static DECIMAL_FORMAT_SYMBOLS: compat::MainThread<DecimalFormatSymbols> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `DECIMAL_FORMAT_SYMBOLS` - see the TODO above"));

/// java: public abstract class NumberFieldController<T extends Number> implements ISliderController<T>, IStringController<T>
#[derive(Clone)]
pub struct NumberFieldControllerBase<T> {
    /// java: private final Option<T> option;
    pub option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>,
    /// java: private final ValueFormatter<T> displayFormatter;
    pub display_formatter: compat::Shared<dyn ValueFormatter<T>>,
}

impl<T: Clone + 'static> NumberFieldControllerBase<T> {
    /// java: public NumberFieldController(Option<T> option, Function<T, Component> displayFormatter)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>, display_formatter: Arc<dyn Fn(T) -> MutableComponent>) -> Self {
        let __init_option = option.clone();
        let __init_display_formatter = Arc::new(move |__a0| (display_formatter)(__a0.clone()));
        let mut this = Self {
            option: __init_option,
            display_formatter: __init_display_formatter,
        };
        this
    }

}

/// java: the virtual methods of NumberFieldController (implemented by NumberFieldControllerBase and every subclass)
pub trait NumberFieldController<T: Clone + 'static>: ISliderController + IStringController {
    fn number_field_controller_base(&self) -> &NumberFieldControllerBase<T>;
    fn number_field_controller_base_mut(&mut self) -> &mut NumberFieldControllerBase<T>;

    /// java: protected String transformInput(String input)
    fn transform_input(&self, mut input: String) -> String {
        if input.is_empty() {
            input = "0".to_string();
        }
        if input == "-" {
            input = "-0".to_string();
        }
        // TODO(fabric2seer): `.getGroupingSeparator(..)` on a DecimalFormatSymbols
        //   java: return input.replace(DECIMAL_FORMAT_SYMBOLS.getGroupingSeparator() + "", "");
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

// NumberFieldController.pendingValue implements ISliderController.pendingValue: it is emitted in each concrete subclass's `impl ISliderController` (Rust has no inherited impls)
// NumberFieldController.interval implements ISliderController.interval: it is emitted in each concrete subclass's `impl ISliderController` (Rust has no inherited impls)
// NumberFieldController.option implements Controller.option: it is emitted in each concrete subclass's `impl Controller` (Rust has no inherited impls)
// NumberFieldController.formatValue implements Controller.formatValue: it is emitted in each concrete subclass's `impl Controller` (Rust has no inherited impls)
// NumberFieldController.provideWidget implements Controller.provideWidget: it is emitted in each concrete subclass's `impl Controller` (Rust has no inherited impls)
// NumberFieldController.setFromString implements IStringController.setFromString: it is emitted in each concrete subclass's `impl IStringController` (Rust has no inherited impls)
// NumberFieldController.isInputValid implements IStringController.isInputValid: it is emitted in each concrete subclass's `impl IStringController` (Rust has no inherited impls)
