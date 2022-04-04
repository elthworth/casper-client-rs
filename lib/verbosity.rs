/// The level of verbosity of the output to stdout.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Verbosity {
    /// Minimal output.
    Low = 0,
    /// Moderate amount of output, with any long hex strings shortened to a string indicating the
    /// character count of the string.
    Medium = 1,
    /// High level of output, with all fields displayed in full.
    High = 2,
}
