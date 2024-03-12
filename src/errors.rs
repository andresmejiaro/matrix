use std::error::Error;
use std::fmt::{self};

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

impl Error for LinAlgError {}
