extern crate kernel32;
extern crate winapi;

use super::*;
use self::kernel32::*;
use self::winapi::windef::*;

pub struct LibraryInterface
{
    hmodule: HMODULE
}

impl LibraryInterface
{
    pub fn new(name: &str) -> Result<Self, LibraryInterfaceCreationError>
    {

        let hmodule: HMODULE;
        match load_library_a(name)
        {
            None =>
            {
                let err_code = get_last_error();
                return Err(
                        LibraryInterfaceCreationError {
                            side: Box::new(SystemError::create_from_code(err_code))
                });
            },
            Some(value) =>
            {
                hmodule = value;
            }
        }

        let library_interface = LibraryInterface
        {
            hmodule: hmodule
        };

        Ok(library_interface)
    }
}

impl LibraryInterfaceLike for LibraryInterface
{
    fn get_function(&self, name: &str) -> Result<extern fn(), LibraryFunctionLoadingError>
    {
        match get_proc_address(self.hmodule, name)
        {
            None =>
            {
                let err_code = get_last_error();
                return Err(
                    LibraryFunctionLoadingError {
                        side: Box::new(SystemError::create_from_code(err_code))
                    }
                );
            },
            Some(value) =>
            {
                Ok(value)
            }
        }
    }
}

impl Drop for LibraryInterface
{
    fn drop(&mut self)
    {
        free_library(self.hmodule);
    }
}

pub fn create_library_interface(name: &str) -> Result<Box<dyn LibraryInterfaceLike>, LibraryInterfaceCreationError>
{
    let library_interface;
    match LibraryInterface::new(name)
    {
        Err(err) =>
        {
            return Err(err);
        },
        Ok(trap) =>
        {
            library_interface = trap;
        }
    }

    Ok(Box::new(library_interface))
}
