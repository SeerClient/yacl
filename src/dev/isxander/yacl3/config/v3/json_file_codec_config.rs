//! Ported from dev.isxander.yacl3.config.v3.JsonFileCodecConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v3::codec_config::CodecConfig;
use crate::dev::isxander::yacl3::config::v3::codec_config::CodecConfigBase;
use mcrust::com::mojang::serialization::data_result::DataResult;
use mcrust::java::lang::throwable::Throwable;

/// java: @ApiStatus.Experimental public abstract class JsonFileCodecConfig<T extends JsonFileCodecConfig<T>> extends CodecConfig<T>
pub struct JsonFileCodecConfigBase<T> {
    /// java: extends CodecConfig
    pub base: CodecConfigBase<T>,
    /// java: private final Path configPath;
    pub config_path: std::path::PathBuf,
    /// java: private final Gson gson;
    pub gson: Option<compat::Gson>,
}

impl<T: 'static> std::ops::Deref for JsonFileCodecConfigBase<T> {
    type Target = CodecConfigBase<T>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for JsonFileCodecConfigBase<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> JsonFileCodecConfigBase<T> {
    /// java: public JsonFileCodecConfig(Path configPath)
    pub fn new(mut config_path: std::path::PathBuf) -> Self {
        let __init_config_path = config_path.clone();
        let mut this = Self {
            base: CodecConfigBase::<T>::new(),
            config_path: __init_config_path,
            gson: None,
        };
        this.gson = Some(this.create_gson());
        this
    }

}

/// java: the virtual methods of JsonFileCodecConfig (implemented by JsonFileCodecConfigBase and every subclass)
pub trait JsonFileCodecConfig<T: Clone + 'static>: CodecConfig {
    fn json_file_codec_config_base(&self) -> &JsonFileCodecConfigBase<T>;
    fn json_file_codec_config_base_mut(&mut self) -> &mut JsonFileCodecConfigBase<T>;

    /// java: public void saveToFile()
    fn save_to_file(&self) {
        let mut json_tree_result: DataResult<serde_json::Value> = self.encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance());
        if json_tree_result.error_ref().is_some() {
            // TODO(fabric2seer): `new IllegalStateException(..)` outside `throw`
            //   java: onSaveError(
            //   java:                     SaveError.ENCODING,
            //   java:                     new IllegalStateException("Failed to encode: " + jsonTreeResult.error().get().message())
            //   java:             );
            return;
        }
        let mut json_tree: serde_json::Value = json_tree_result.result().unwrap();
        // TODO(fabric2seer): `Gson.toJson` (Gson over the generated per-class forms)
        //   java: String json = gson.toJson(jsonTree);
        let mut json: String = todo!("fabric2seer: the initialiser of `json` (TODO above)");
        // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            std::fs::write(&self.json_file_codec_config_base().config_path, &json).expect("IOException");
        }));
        if let Err(e) = __result {
            self.on_save_error(SaveError::Writing, Some(e.clone()));
        }
    }

    /// java: public boolean loadFromFile()
    fn load_from_file(&self) -> bool {
        if !self.json_file_codec_config_base().config_path.exists() {
            return false;
        }
        let mut json: String;
        // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            json = std::fs::read_to_string(&self.json_file_codec_config_base().config_path).expect("IOException");
        }));
        if let Err(e) = __result {
            self.on_load_error(LoadError::Reading, Some(e.clone()));
            return false;
        }
        let mut json_tree: serde_json::Value;
        // java: try {..} catch (JsonParseException e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            json_tree = serde_json::from_str::<serde_json::Value>(&json).expect("JsonSyntaxException");
        }));
        if let Err(e) = __result {
            self.on_load_error(LoadError::JsonParsing, Some(e.clone()));
            return false;
        }
        return self.decode(&json_tree, &mcrust::com::mojang::serialization::json_ops::JsonOps::instance());
    }

    /// java: protected Gson createGson()
    fn create_gson(&self) -> compat::Gson {
        return compat::GsonBuilder::new().set_pretty_printing().create();
    }

    /// java: protected void onSaveError(SaveError error, @Nullable Throwable e)
    fn on_save_error(&self, mut error: SaveError, mut e: Option<Throwable>) {
        panic!("IllegalStateException: {}", format!("Error whilst {}", error.name().to_lowercase()).to_string());
    }

    /// java: protected void onLoadError(LoadError error, @Nullable Throwable e)
    fn on_load_error(&self, mut error: LoadError, mut e: Option<Throwable>) {
        panic!("IllegalStateException: {}", format!("Error whilst {}", error.name().to_lowercase()).to_string());
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
