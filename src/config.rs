/// Configuration for FluentTest's output and behavior
pub struct Config {
    pub(crate) use_colors: bool,
    pub(crate) use_unicode_symbols: bool,
    pub(crate) show_success_details: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    /// Creates a new configuration with default settings
    pub fn new() -> Self {
        Self { use_colors: true, use_unicode_symbols: true, show_success_details: true }
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

    /// Apply the configuration
    pub fn apply(self) {
        use crate::reporter::GLOBAL_CONFIG;
        *GLOBAL_CONFIG.write().unwrap() = self;
    }
}
