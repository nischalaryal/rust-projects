// similar to import in other programs
use std::fmt;

// Defining a structure. we will implement fmt::Display for it
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt:: Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

