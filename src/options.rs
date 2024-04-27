/// Struct containing options that can be passed to jq to customize input/output
pub struct JqOptions<'a> {
    pub raw_output: bool,
    pub raw_input: bool,
    pub slurp: bool,
    pub sort_keys: bool,
    pub colorization: JqColorization<'a>,
    pub indentation: JqIndentation,
}

impl<'a> Default for JqOptions<'a> {
    fn default() -> Self {
        JqOptions {
            raw_input: false, // TODO
            raw_output: false,
            slurp: false, // TODO
            sort_keys: false, // TODO
            indentation: JqIndentation::Compact,
            colorization: JqColorization::Monochrome,
        }
    }
}

impl<'a> JqOptions<'a> {
    pub fn with_raw_output(&self, raw_output: bool) -> Self {
        JqOptions {
            raw_output,
            raw_input: self.raw_input,
            slurp: self.slurp,
            sort_keys: self.sort_keys,
            colorization: self.colorization,
            indentation: self.indentation,
        }
    }

    pub fn with_raw_input(&self, raw_input: bool) -> Self {
        JqOptions {
            raw_output: self.raw_output,
            raw_input,
            slurp: self.slurp,
            sort_keys: self.sort_keys,
            colorization: self.colorization,
            indentation: self.indentation,
        }
    }

    pub fn with_slurp(&self, slurp: bool) -> Self {
        JqOptions {
            raw_output: self.raw_output,
            raw_input: self.raw_input,
            slurp,
            sort_keys: self.sort_keys,
            colorization: self.colorization,
            indentation: self.indentation,
        }
    }

    pub fn with_sort_keys(&self, sort_keys: bool) -> Self {
        JqOptions {
            raw_output: self.raw_output,
            raw_input: self.raw_input,
            slurp: self.slurp,
            sort_keys,
            colorization: self.colorization,
            indentation: self.indentation,
        }
    }

    pub fn with_colorization(&self, colorization: JqColorization<'a>) -> Self {
        JqOptions {
            raw_output: self.raw_output,
            raw_input: self.raw_input,
            slurp: self.slurp,
            sort_keys: self.sort_keys,
            colorization,
            indentation: self.indentation,
        }
    }

    pub fn with_indentation(&self, indentation: JqIndentation) -> Self {
        JqOptions {
            raw_output: self.raw_output,
            raw_input: self.raw_input,
            slurp: self.slurp,
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
