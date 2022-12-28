use std::fmt::Write;

use crate::Error;

// outputs the hook to a string
// the hook expects one command line input (the commit message)
// and will modify it by adding the mjis
pub fn hook() -> String {
    "".into()
}

// install the hook the the specified path
pub fn install_hook(path: &str) -> Result<(), Error> {
    Ok(())
}
