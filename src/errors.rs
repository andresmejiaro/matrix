// Import Error trait and formatting utilities from the standard library
use std::error::Error;
use std::fmt::{self};
// Define a public enum `LinAlgError` for linear algebra errors with detailed cases
#[derive(Debug)]
pub enum LinAlgError {
    OperationNonConforming {
        operation: String,
    },
    SinglarMatrix,
    BuildNonconforming {
        expected: usize,
        recieved: usize,
    },
    OutofBoundsVector {
        size: usize,
        recieved: usize,
    },
    OutofBoundsMatrix {
        size: (usize, usize),
        recieved: (usize, usize),
    },
    EmptyArgs,
}

// Implement the Display trait for `LinAlgError` to enable custom error messages
impl fmt::Display for LinAlgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LinAlgError::OperationNonConforming { operation } => {
                write!(f, "{}: Nonconforming dimentions", operation)
            }
            LinAlgError::SinglarMatrix => {
                write!(f, "Operation not defined for a Singular Matrix")
            }
            LinAlgError::BuildNonconforming { expected, recieved } => write!(
                f,
                "Build error: expected {} entries got {}",
                expected, recieved
            ),
            LinAlgError::OutofBoundsMatrix { size, recieved } => write!(
                f,
                "Trying to access ({},{}) in a ({},{}) element",
                recieved.0, recieved.1, size.0, size.1
            ),
            LinAlgError::OutofBoundsVector { size, recieved } => {
                write!(f, "Trying to access {} in a {} element", recieved, size)
            }
            LinAlgError::EmptyArgs => write!(f, "Recieved an empty object"),
        }
    }
}

// Implement the Error trait for `LinAlgError` to integrate with error handling
impl Error for LinAlgError {}
