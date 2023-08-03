extern crate glibc;

use super::*;
use self::glibc::{FileHandle, dlopen, dlclose, dlsym, RTLD_LAZY};

pub struct LibraryInterface
{
    handle: FileHandle
}

impl LibraryInterface
{
    pub fn new(name: &str) -> Result<Self, LibraryInterfaceCreationError>
    {
        match dlopen(name, RTLD_LAZY)
        {
            Err(e) =>
            {
                return Err(
                    LibraryInterfaceCreationError {
                        msg: String::from("Unable to load library."),
                        side: Some(e)
                });
            },
            Ok(handle) =>
            {
                if !handle.is_valid()
                {
                    return Err(
                        LibraryInterfaceCreationError {
                            msg: String::from(format!("{} could not be found", name)),
                            side: None
                        }
                    );
                }
                
                return Ok(
                    LibraryInterface {
                        handle: handle
                });
            }
        }
    }
}

impl LibraryInterfaceLike for LibraryInterface
{
    fn get_function(&self, name: &str) -> Result<extern fn(), LibraryFunctionLoadingError>
    {
        // TODO: use the dlerror->dlsym->dlerror approach described in the
        //       glibc documentation to check for a valid NULL return value
        match dlsym(&self.handle, name)
        {
            Err(e) =>
            {
                return Err(
                    LibraryFunctionLoadingError {
                        side: e
                    }
                )
            },
            Ok(symbol_address) =>
            {
                let fn_ptr = unsafe
                {
                    std::mem::transmute(symbol_address)
                };
                return Ok(fn_ptr)
            }
        }
    }
}

pub fn create_library_interface(name: &str) -> Result<Box<dyn LibraryInterfaceLike>, LibraryInterfaceCreationError>
{
    match LibraryInterface::new(name)
    {
        Err(e) =>
        {
            Err(e)
        },
        Ok(library_interface) =>
        {
            Ok(Box::new(library_interface))
        }
    }
}

impl Drop for LibraryInterface
{
    fn drop(&mut self)
    {
        // We need to move the handle out of the struct because dlclose
        // does not accept borrows
        let invalid_handle = FileHandle::invalid();
        let valid_handle = std::mem::replace(&mut self.handle, invalid_handle);
        dlclose(valid_handle);
    }
}