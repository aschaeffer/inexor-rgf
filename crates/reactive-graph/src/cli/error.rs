use std::fmt::Debug;

use thiserror::Error;

use reactive_graph_client::InexorRgfClientExecutionError;

#[derive(Debug, Error)]
pub enum SerializationError {
    #[error("Failed to serialize JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[cfg(feature = "json5")]
    #[error("Failed to serialize JSON5: {0}")]
    Json5(#[from] json5::Error),
    #[cfg(feature = "toml")]
    #[error("Failed to serialize TOML: {0}")]
    Toml(#[from] toml::ser::Error),
}

#[derive(Debug, Error)]
pub(crate) enum CommandError {
    #[error("Missing sub command")]
    MissingSubCommand,
    #[error("Execution failed: {0}")]
    InexorRgfClientExecutionError(InexorRgfClientExecutionError),
    #[error("Not yet implemented")]
    NotImplemented,
    #[error("Rejected: {0}")]
    Rejected(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("No change: {0}")]
    NoChange(String),
    #[error("No content: {0}")]
    NoContent(String),
    #[error("Not created: {0}")]
    NotCreated(String),
    #[error("Serialization failed: {0}")]
    SerializationError(SerializationError),
}

impl CommandError {
    pub(crate) fn exit_code(&self) -> i32 {
        match self {
            CommandError::MissingSubCommand => 254,
            CommandError::InexorRgfClientExecutionError(_) => 253,
            CommandError::NotImplemented => 252,
            CommandError::Rejected(_) => 4,
            CommandError::NotFound(_) => 3,
            CommandError::NoChange(_) => 2,
            CommandError::NoContent(_) => 1,
            CommandError::NotCreated(_) => 5,
            CommandError::SerializationError(_) => 6,
        }
    }
}

// impl Display for CommandError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             CommandError::MissingSubCommand => {
//                 writeln!(f, "Missing sub command")
//             }
//             CommandError::InexorRgfClientExecutionError(e) => {
//                 writeln!(f, "{e}")
//             }
//             CommandError::Rejected(message) => {
//                 writeln!(f, "{}", message)
//             }
//             CommandError::NotFound(message) => {
//                 writeln!(f, "{}", message)
//             }
//             CommandError::NoChange(message) => {
//                 writeln!(f, "{}", message)
//             }
//             CommandError::NoContent(message) => {
//                 writeln!(f, "{}", message)
//             }
//             CommandError::NotCreated(message) => {
//                 writeln!(f, "{}", message)
//             }
//         }
//     }
// }

// impl Error for CommandError {
//     fn description(&self) -> &str {
//         match self {
//             CommandError::MissingSubCommand => "Missing sub command",
//             CommandError::InexorRgfClientExecutionError(_) => "Client execution error",
//             CommandError::Rejected(message) => message,
//             CommandError::NotFound(message) => message,
//             CommandError::NoChange(message) => message,
//             CommandError::NoContent(message) => message,
//             CommandError::NotCreated(message) => message,
//         }
//     }
// }

impl From<InexorRgfClientExecutionError> for CommandError {
    fn from(e: InexorRgfClientExecutionError) -> Self {
        CommandError::InexorRgfClientExecutionError(e)
    }
}

impl From<SerializationError> for CommandError {
    fn from(e: SerializationError) -> Self {
        CommandError::SerializationError(e)
    }
}
