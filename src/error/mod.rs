//! This module provides the error-handling framework for the whole project.

pub use std::error::Error;
use std::fmt;
use std::result;

///
/// custom Result type for functions which can
/// potentially fail
///
pub type Result<T> = result::Result<T, MazeError>;

///
/// Categories of potential errors
///
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    /// Provided dimensions were not positive
    /// integers
    InvalidDimensions,

    /// special case where the provided dimensions are not
    /// valid numbers
    InvalidDimensionsNotNumber,

    /// Provided vertex/vertices was/were
    /// invalid
    InvalidVertexOrVertices,
}

impl ErrorKind {
    fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::InvalidDimensionsNotNumber => "invalid dimensions: non-numeric values",
            ErrorKind::InvalidDimensions => "invalid dimensions: non (positive) integer values",
            ErrorKind::InvalidVertexOrVertices => "invalid vertex or vertices",
        }
    }
}

///
/// Custom error type for the Maze project
///
pub struct MazeError {
    repr: Repr,
}

///
/// Possible variants for errors - the basic ones listed in the
/// ErrorKind enum, or custom types for more flexibility.
///
#[derive(Debug)]
enum Repr {
    Simple(ErrorKind),
    Custom(Box<Custom>),
}

#[derive(Debug)]
struct Custom {
    kind: ErrorKind,
    error: Box<dyn Error + Send + Sync>,
}

impl MazeError {
    /// allow creation of errors from known kinds of errors
    /// as well as from custom types
    pub fn new<E>(kind: ErrorKind, error: E) -> Self
    where
        E: Into<Box<dyn Error + Send + Sync>>,
    {
        MazeError {
            repr: Repr::Custom(Box::new(Custom {
                kind: kind,
                error: error.into(),
            })),
        }
    }

    /// allows the creation of a simple error by
    /// simply passing in the error kind
    pub fn of(kind: ErrorKind) -> Self {
        MazeError {
            repr: Repr::Simple(kind),
        }
    }
}

/// make this custom error type an actual error type that can be used
/// in conjunction with other error types.
impl Error for MazeError {}

impl fmt::Debug for MazeError {
    /// allows errors to be printed using the `{:?}` formatter
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.repr, f)
    }
}

impl fmt::Display for MazeError {
    /// allows errors to be printed using the `{}` formatter
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.repr {
            Repr::Simple(kind) => write!(f, "Error: {}", kind.as_str()),
            Repr::Custom(ref custom_error) => write!(f, "{}", custom_error.error),
        }
    }
}

impl From<ErrorKind> for MazeError {
    /// Allows conversions from other error types into
    /// our custom error type
    #[inline]
    fn from(kind: ErrorKind) -> MazeError {
        MazeError {
            repr: Repr::Simple(kind),
        }
    }
}
