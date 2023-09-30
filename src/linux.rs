extern crate glibc;

use self::glibc::{dlclose, dlerror, dlopen, dlsym, FileHandle, RTLD_LAZY};
use super::*;

pub struct LibraryInterface {
    handle: FileHandle,
}

impl LibraryInterface {
    pub fn new(name: &str) -> Result<Self, LibraryInterfaceCreationError> {
        match dlopen(name, RTLD_LAZY) {
            Err(e) => {
                return Err(LibraryInterfaceCreationError {
                    msg: String::from("An unexpected error occured"),
                    side: Some(e),
                });
            }
            Ok(handle) => {
                if !handle.is_valid() {
                    return Err(LibraryInterfaceCreationError {
                        msg: String::from(format!("{} could not be found", name)),
                        side: None,
                    });
                }

                return Ok(LibraryInterface { handle: handle });
            }
        }
    }
}

impl LibraryInterfaceLike for LibraryInterface {
    fn get_function(&self, name: &str) -> Result<extern "C" fn(), LibraryFunctionLoadingError> {
        // The documentation for dlsym states that the proper way to check if a symbol could not be
        // loaded is by calling dlerror, then dlsym and then dlerror again.
        let _ = dlerror();
        match dlsym(&self.handle, name) {
            Err(e) => {
                return Err(LibraryFunctionLoadingError {
                    msg: format!("An unexpected error occured"),
                    side: Some(e),
                })
            }
            Ok(symbol_address) => {
                if symbol_address.is_null() {
                    match dlerror() {
                        Err(err) => {
                            return Err(LibraryFunctionLoadingError {
                                msg: format!("An unexpected error occurred"),
                                side: Some(Box::new(err)),
                            });
                        }
                        Ok(message) => match message {
                            Some(message) => {
                                return Err(LibraryFunctionLoadingError {
                                    msg: format!("Failed to load symbol {}: {}", name, message),
                                    side: None,
                                })
                            }
                            None => {
                                let fn_ptr = unsafe { std::mem::transmute(symbol_address) };
                                return Ok(fn_ptr);
                            }
                        },
                    }
                } else {
                    let fn_ptr = unsafe { std::mem::transmute(symbol_address) };
                    return Ok(fn_ptr);
                }
            }
        }
    }
}

pub fn create_library_interface(
    name: &str,
) -> Result<Box<dyn LibraryInterfaceLike>, LibraryInterfaceCreationError> {
    match LibraryInterface::new(name) {
        Err(e) => Err(e),
        Ok(library_interface) => Ok(Box::new(library_interface)),
    }
}

impl Drop for LibraryInterface {
    fn drop(&mut self) {
        // We need to move the handle out of the struct because dlclose
        // does not accept borrows
        let invalid_handle = FileHandle::invalid();
        let valid_handle = std::mem::replace(&mut self.handle, invalid_handle);
        dlclose(valid_handle);
    }
}
