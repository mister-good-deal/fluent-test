use std::env;
use std::sync::Once;

// Initialization flag to ensure we only initialize once
static INIT: Once = Once::new();

// Environment variable to control enhanced output
const ENV_ENHANCED_OUTPUT: &str = "FLUENT_TEST_ENHANCED_OUTPUT";

/// Configuration for FluentTest's output and behavior
pub struct Config {
    pub(crate) use_colors: bool,
    pub(crate) use_unicode_symbols: bool,
    pub(crate) show_success_details: bool,
    /// Enable enhanced test output (fluent assertions instead of standard output)
    pub(crate) enhanced_output: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

// Implement Clone for Config
impl Clone for Config {
    fn clone(&self) -> Self {
        Self {
            use_colors: self.use_colors,
            use_unicode_symbols: self.use_unicode_symbols,
            show_success_details: self.show_success_details,
            enhanced_output: self.enhanced_output,
        }
    }
}

impl Config {
    /// Creates a new configuration with default settings
    pub fn new() -> Self {
        // Check for environment variable to enable enhanced output
        let enhanced_from_env = match env::var(ENV_ENHANCED_OUTPUT) {
            Ok(val) => val.to_lowercase() == "true" || val == "1" || val == "yes",
            Err(_) => false, // Default to standard output if env var not set
        };

        Self { use_colors: true, use_unicode_symbols: true, show_success_details: true, enhanced_output: enhanced_from_env }
    }

    /// Enable or disable colored output
    pub fn use_colors(mut self, enable: bool) -> Self {
        self.use_colors = enable;
        self
    }

    /// Enable or disable Unicode symbols
    pub fn use_unicode_symbols(mut self, enable: bool) -> Self {
        self.use_unicode_symbols = enable;
        self
    }

    /// Control whether to show details for successful tests
    pub fn show_success_details(mut self, enable: bool) -> Self {
        self.show_success_details = enable;
        self
    }

    /// Enable or disable enhanced output (fluent assertions)
    pub fn enhanced_output(mut self, enable: bool) -> Self {
        self.enhanced_output = enable;
        self
    }

    /// Apply the configuration
    pub fn apply(self) {
        use crate::reporter::GLOBAL_CONFIG;

        // Clone self before moving it into the global config
        let config = self.clone();
        *GLOBAL_CONFIG.write().unwrap() = self;

        // Initialize the event system if enhanced output is enabled
        if config.enhanced_output {
            crate::initialize();
        }
    }
}

/// Initialize the FluentTest system
/// This is called automatically when needed but can also be called explicitly
pub fn initialize() {
    INIT.call_once(|| {
        // Check if enhanced output is enabled in the config
        let config = crate::reporter::GLOBAL_CONFIG.read().unwrap();

        if config.enhanced_output {
            // Initialize event system
            crate::events::EventEmitter::init();

            // Register event handlers
            crate::Reporter::init();
        }
    });
}

/// Check if enhanced output is enabled in the current configuration
pub fn is_enhanced_output_enabled() -> bool {
    let config = crate::reporter::GLOBAL_CONFIG.read().unwrap();
    return config.enhanced_output;
}
