/// Struct containing options that can be passed to jq to customize input/output
pub struct JqOptions<'a> {
    /// Output raw strings instead of quoted and escaped 
    pub raw_output: bool,
    /// Interpret each input as string instead of json
    pub raw_input: bool,
    /// Order the keys
    pub sort_keys: bool,
    /// Use colors for the output
    pub colorization: JqColorization<'a>,
    /// Apply indentation for the output
    pub indentation: JqIndentation,
}

impl<'a> Default for JqOptions<'a> {
    fn default() -> Self {
        JqOptions {
            raw_input: false,
            raw_output: false,
            sort_keys: false,
            indentation: JqIndentation::Compact,
            colorization: JqColorization::Monochrome,
        }
    }
}

impl<'a> JqOptions<'a> {
    /// Output raw strings instead of quoted and escaped 
    pub fn with_raw_output(&self, raw_output: bool) -> Self {
        JqOptions {
            raw_output,
            raw_input: self.raw_input,
            sort_keys: self.sort_keys,
            colorization: self.colorization,
            indentation: self.indentation,
        }
    }

    /// Interpret each input as string instead of json
    pub fn with_raw_input(&self, raw_input: bool) -> Self {
        JqOptions {
            raw_output: self.raw_output,
            raw_input,
            sort_keys: self.sort_keys,
            colorization: self.colorization,
            indentation: self.indentation,
        }
    }

    /// Order the keys
    pub fn with_sort_keys(&self, sort_keys: bool) -> Self {
        JqOptions {
            raw_output: self.raw_output,
            raw_input: self.raw_input,
            sort_keys,
            colorization: self.colorization,
            indentation: self.indentation,
        }
    }

    /// Use colors for the output
    pub fn with_colorization(&self, colorization: JqColorization<'a>) -> Self {
        JqOptions {
            raw_output: self.raw_output,
            raw_input: self.raw_input,
            sort_keys: self.sort_keys,
            colorization,
            indentation: self.indentation,
        }
    }

    /// Apply indentation for the output
    pub fn with_indentation(&self, indentation: JqIndentation) -> Self {
        JqOptions {
            raw_output: self.raw_output,
            raw_input: self.raw_input,
            sort_keys: self.sort_keys,
            colorization: self.colorization,
            indentation,
        }
    }
}

/// The indentation options for jq
#[derive(Copy, Clone)]
pub enum JqIndentation {
    /// Don't indent, fully compact
    Compact,
    /// Use tabs for indentation
    Tabs,
    /// Use spaces for indentation
    Spaces(i32),
}

/// The two possible colorization options
#[derive(Copy, Clone)]
pub enum JqColorization<'a> {
    /// Apply custom colors
    Custom(&'a str),
    /// Apply full colorization
    Colorize,
    /// Don't apply colorization
    Monochrome,
}
