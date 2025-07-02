//!
//! Log Entry Level
//!

use std::fmt;

/// Log entry level setting.\
/// Default level: INFO.
#[allow(unused)]
#[derive(Debug, serde::Serialize, Clone)]
pub enum Level {
    FINEST,
    FINER,
    FINE,
    CONFIG,
    INFO,
    WARNING,
    SEVERE,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let label =  match self {
            Level::FINEST => "FINEST",
            Level::FINER => "FINER",
            Level::FINE => "FINE",
            Level::CONFIG => "CONFIG",
            Level::INFO => "INFO",
            Level::WARNING => "WARNING",
            Level::SEVERE => "SEVERE",
        };

        writeln!(f, "{label}")?;
        Ok(())
    }
}

impl Default for Level {
    fn default() -> Self {
        Level::INFO
    }
}
