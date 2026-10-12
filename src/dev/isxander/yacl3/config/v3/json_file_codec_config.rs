//! Ported from dev.isxander.yacl3.config.v3.JsonFileCodecConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v2::r#impl::serializer::gson_config_serializer;
use crate::dev::isxander::yacl3::config::v3::codec_config::CodecConfig;
use crate::dev::isxander::yacl3::config::v3::codec_config::CodecConfigBase;
use mcrust::com::mojang::serialization::data_result::DataResult;
use mcrust::com::mojang::serialization::Obj;
use mcrust::java::lang::throwable::Throwable;

/// java: @ApiStatus.Experimental public abstract class JsonFileCodecConfig<T extends JsonFileCodecConfig<T>> extends CodecConfig<T>
/// - its fields.
pub struct JsonFileCodecConfigBase<T: 'static> {
    /// java: extends CodecConfig
    pub base: CodecConfigBase<T>,
    /// java: private final Path configPath;
    pub config_path: std::path::PathBuf,
    /// java: private final Gson gson; - `this.gson = createGson()` in the constructor. `createGson` is virtual and a
    /// Rust base cannot call the subclass's override before the subclass exists, so the field is filled by the first
    /// read (`JsonFileCodecConfig::gson`), through the override.
    pub gson: std::cell::OnceCell<compat::Gson>,
}

impl<T: 'static> std::ops::Deref for JsonFileCodecConfigBase<T> {
    type Target = CodecConfigBase<T>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> JsonFileCodecConfigBase<T> {
    /// java: public JsonFileCodecConfig(Path configPath)
    pub fn new(config_path: std::path::PathBuf) -> Self {
        Self { base: CodecConfigBase::new(), config_path, gson: std::cell::OnceCell::new() }
    }
}

/// java: the virtual methods of JsonFileCodecConfig (implemented by every subclass)
pub trait JsonFileCodecConfig<T: 'static>: CodecConfig<T> {
    fn json_file_codec_config_base(&self) -> &JsonFileCodecConfigBase<T>;

    /// java: private final Gson gson; (read)
    fn gson(&self) -> compat::Gson {
        self.json_file_codec_config_base().gson.get_or_init(|| self.create_gson()).clone()
    }

    /// java: public void saveToFile()
    fn save_to_file(&self) {
        let json_tree_result: DataResult<Obj> = self.encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance());
        if let Some(error) = json_tree_result.error_ref() {
            self.on_save_error(
                SaveError::Encoding,
                Some(Throwable::of_class("java.lang.IllegalStateException", Some(format!("Failed to encode: {}", error.message())))),
            );
            return;
        }

        let json_tree: Obj = json_tree_result.result().expect("NoSuchElementException: No value present");
        // java: String json = gson.toJson(jsonTree);
        let json: String = compat::gson_text(&compat::GsonValue::of_json(&compat::json_from_port(&json_tree)), self.gson().pretty);

        // java: Files.writeString(configPath, json, StandardOpenOption.TRUNCATE_EXISTING, StandardOpenOption.CREATE);
        if let Err(e) = std::fs::write(&self.json_file_codec_config_base().config_path, &json) {
            self.on_save_error(SaveError::Writing, Some(Throwable::of_class("java.io.IOException", Some(e.to_string()))));
        }
    }

    /// java: public boolean loadFromFile()
    fn load_from_file(&self) -> bool {
        let config_path = self.json_file_codec_config_base().config_path.clone();
        if !config_path.exists() {
            return false;
        }

        let json: String = match std::fs::read_to_string(&config_path) {
            Ok(json) => json,
            Err(e) => {
                self.on_load_error(LoadError::Reading, Some(Throwable::of_class("java.io.IOException", Some(e.to_string()))));
                return false;
            }
        };

        // java: JsonParser.parseString(json) - Gson's lenient parse
        let json_tree: serde_json::Value = match gson_config_serializer::parse_lenient(&json) {
            Ok(json_tree) => json_tree,
            Err(e) => {
                self.on_load_error(LoadError::JsonParsing, Some(Throwable::of_class("com.google.gson.JsonParseException", Some(e))));
                return false;
            }
        };

        self.decode_r_dynamic_ops(&compat::json_to_port(&json_tree), &mcrust::com::mojang::serialization::json_ops::JsonOps::instance())
    }

    /// java: protected Gson createGson()
    fn create_gson(&self) -> compat::Gson {
        compat::GsonBuilder::new().set_pretty_printing().create()
    }

    /// java: protected void onSaveError(SaveError error, @Nullable Throwable e)
    fn on_save_error(&self, error: SaveError, e: Option<Throwable>) {
        panic!(
            "IllegalStateException: Error whilst {}{}",
            error.name().to_lowercase(),
            e.and_then(|e| e.get_message().map(|m| format!(" (caused by: {})", m))).unwrap_or_default()
        );
    }

    /// java: protected void onLoadError(LoadError error, @Nullable Throwable e)
    fn on_load_error(&self, error: LoadError, e: Option<Throwable>) {
        panic!(
            "IllegalStateException: Error whilst {}{}",
            error.name().to_lowercase(),
            e.and_then(|e| e.get_message().map(|m| format!(" (caused by: {})", m))).unwrap_or_default()
        );
    }
}

/// java: protected enum SaveError
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SaveError {
    Writing,
    Encoding,
}

impl SaveError {
    pub const VALUES: [SaveError; 2] = [SaveError::Writing, SaveError::Encoding];

    /// java: values()
    pub fn values() -> Vec<SaveError> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            SaveError::Writing => "WRITING",
            SaveError::Encoding => "ENCODING",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> SaveError {
        match name {
            "WRITING" => SaveError::Writing,
            "ENCODING" => SaveError::Encoding,
            _ => panic!("IllegalArgumentException: No enum constant SaveError.{name}"),
        }
    }
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// java: protected enum LoadError
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LoadError {
    Reading,
    JsonParsing,
    Decoding,
}

impl LoadError {
    pub const VALUES: [LoadError; 3] = [LoadError::Reading, LoadError::JsonParsing, LoadError::Decoding];

    /// java: values()
    pub fn values() -> Vec<LoadError> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            LoadError::Reading => "READING",
            LoadError::JsonParsing => "JSON_PARSING",
            LoadError::Decoding => "DECODING",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> LoadError {
        match name {
            "READING" => LoadError::Reading,
            "JSON_PARSING" => LoadError::JsonParsing,
            "DECODING" => LoadError::Decoding,
            _ => panic!("IllegalArgumentException: No enum constant LoadError.{name}"),
        }
    }
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
