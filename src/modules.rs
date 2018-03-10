use kmod_sys::{self, kmod_list, kmod_module};
use reduce::Reduce;
use errno;

use std::ffi::{CStr, CString};
use std::fmt;
use errors::{Result, ErrorKind};


/// Wrapper around a kmod_module
pub struct Module {
    inner: *mut kmod_module,
}

impl Drop for Module {
    fn drop(&mut self) {
        trace!("dropping kmod_module: {:?}", self.inner);
        unsafe { kmod_sys::kmod_module_unref(self.inner) };
    }
}

impl Module {
    #[inline]
    pub(crate) fn new(module: *mut kmod_module) -> Module {
        trace!("creating kmod_module: {:?}", module);
        Module {
            inner: module,
        }
    }

    /// Get the name of the module
    #[inline]
    pub fn name(&self) -> String {
        let name = unsafe { kmod_sys::kmod_module_get_name(self.inner) };
        let name = unsafe { CStr::from_ptr(name) };
        name.to_string_lossy().into_owned()
    }

    /// Get the size of the module
    #[inline]
    pub fn size(&self) -> i64 {
        unsafe { kmod_sys::kmod_module_get_size(self.inner) }
    }

    /// Get the number of references to this module
    #[inline]
    pub fn refcount(&self) -> i32 {
        unsafe { kmod_sys::kmod_module_get_refcnt(self.inner) }
    }

    /// Iterate over the modules depending on this module
    #[inline]
    pub fn holders(&self) -> ModuleIterator {
        let holders = unsafe { kmod_sys::kmod_module_get_holders(self.inner) };
        ModuleIterator::new(holders)
    }

    /// Get this modules dependencies
    #[inline]
    pub fn dependencies(&self) -> ModuleIterator {
        let dependencies = unsafe { kmod_sys::kmod_module_get_dependencies(self.inner) };
        ModuleIterator::new(dependencies)
    }

    /// Get module path
    #[inline]
    pub fn path(&self) -> String {
        let path = unsafe { kmod_sys::kmod_module_get_path(self.inner) };
        let path = unsafe { CStr::from_ptr(path) };
        path.to_string_lossy().into_owned()
    }


    /// Get module options
    #[inline]
    pub fn options(&self) -> String {
        let options = unsafe { kmod_sys::kmod_module_get_options(self.inner) };
        let options = unsafe { CStr::from_ptr(options) };
        options.to_string_lossy().into_owned()
    }

    /// Insert the module into the kernel
    #[inline]
    pub fn insert_module(&self, flags: u32, opts: Vec<String>) -> Result<()> {
        let opts = opts.into_iter()
            .reduce(|a, b| a + " " + &b)
            .unwrap_or(String::new());

        let opts = CString::new(opts)?;

        let ret = unsafe { kmod_sys::kmod_module_insert_module(self.inner, flags, opts.as_ptr()) };
        if ret < 0 {
            Err(ErrorKind::Errno(errno::errno()).into())
        } else {
            Ok(())
        }
    }

    /// Remove the module from the kernel
    #[inline]
    pub fn remove_module(&self, flags: u32) -> Result<()> {
        let ret = unsafe { kmod_sys::kmod_module_remove_module(self.inner, flags) };
        if ret < 0 {
            Err(ErrorKind::Errno(errno::errno()).into())
        } else {
            Ok(())
        }
    }
}

impl fmt::Debug for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("Module { .. }")
    }
}

/// Iterator over a kmod_list of modules
pub struct ModuleIterator {
    list: *mut kmod_list,
    iter: *mut kmod_list,
}

impl Drop for ModuleIterator {
    fn drop(&mut self) {
        trace!("dropping kmod_list: {:?}", self.list);
        unsafe { kmod_sys::kmod_module_unref_list(self.list) };
    }
}

impl ModuleIterator {
    #[inline]
    pub(crate) fn new(list: *mut kmod_list) -> ModuleIterator {
        trace!("creating kmod_list: {:?}", list);
        ModuleIterator {
            list: list,
            iter: list,
        }
    }
}

impl Iterator for ModuleIterator {
    type Item = Module;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter = unsafe { kmod_sys::kmod_list_next(self.list, self.iter) };
        trace!("kmod_list->next: {:?}", self.iter);
        if !self.iter.is_null() {
            let module = unsafe { kmod_sys::kmod_module_get_module(self.iter) };
            Some(Module::new(module))
        } else {
            None
        }
    }
}

impl fmt::Debug for ModuleIterator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("ModuleIterator { .. }")
    }
}

