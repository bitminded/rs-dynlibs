extern crate errors;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct LibraryInterfaceCreationError
{
    msg: String,
    side: Option<Box<dyn Error>>
}

impl Error for LibraryInterfaceCreationError
{
    fn source(&self) -> Option<&(dyn Error + 'static )>
    {
        match &self.side
        {
            None =>
            {
                None
            },
            Some(side) =>
            {
                Some(side.as_ref())
            }
        }
    }
}

impl fmt::Display for LibraryInterfaceCreationError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "Unable to create library interface.")
    }
}

#[derive(Debug)]
pub struct LibraryFunctionLoadingError
{
    side: Box<dyn Error>
}

impl Error for LibraryFunctionLoadingError
{
    fn source(&self) -> Option<&(dyn Error + 'static )>
    {
        Some(self.side.as_ref())
    }
}

impl fmt::Display for LibraryFunctionLoadingError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "Unable to load function from library.")
    }
}

pub trait LibraryInterfaceLike
{
    fn get_function(&self, name: &str) -> Result<extern fn(), LibraryFunctionLoadingError>;
}

type FnCreateLibraryInterface = fn(name: &str) -> Result<Box<dyn LibraryInterfaceLike>, LibraryInterfaceCreationError>;

#[cfg(target_os = "windows")]
mod win32;

#[allow(non_upper_case_globals)]
#[cfg(target_os = "windows")]
pub static create_library_interface: FnCreateLibraryInterface = win32::create_library_interface;

#[cfg(target_os = "linux")]
mod linux;

#[allow(non_upper_case_globals)]
#[cfg(target_os = "linux")]
pub static create_library_interface: FnCreateLibraryInterface = linux::create_library_interface;