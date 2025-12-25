use std::{mem::MaybeUninit, ptr::NonNull, sync::atomic::{AtomicU32, Ordering}};

use shader_slang_sys::{ICastableVtable, ISlangUnknown__bindgen_vtable, SlangResult, SlangUUID as UUID};
use crate::{Blob, Error, Result, result_from_blob, succeeded, uuid, uuid_eq, vcall};
pub(crate) use shader_slang_sys as sys;

use crate::{IUnknown, Interface};

#[repr(transparent)]
#[derive(Clone)]
pub struct FileSystem(IUnknown);

impl FileSystem {
    /// Loads a file from the file system.
    pub fn load_file(&self, path: &str) -> Result<Blob> {
        let path = std::ffi::CString::new(path).unwrap();

        let mut blob = MaybeUninit::<*mut sys::ISlangBlob>::uninit();
        let code = vcall!(self, loadFile(path.as_ptr(), blob.as_mut_ptr()));

        if succeeded(code) {
            // SAFETY: The blob is initialized by the call to loadFile.
            let blob = unsafe { blob.assume_init() };
            let blob = NonNull::new(blob).unwrap();

            Ok(Blob(IUnknown(unsafe { std::mem::transmute(blob) })))
        } else {
            Err(Error::Code(code))
        }
    }
}

unsafe impl Interface for FileSystem {
	type Vtable = sys::IFileSystemVtable;
	const IID: UUID = uuid(0x8f241361_f5bd_4ca0_a3ac02f7fa2402b8);
}

/// A trait for user-provided file systems that can be used to load files into slang. 
pub trait FileSystemTrait: Sync + Send {
	fn load_file(&self, path: &str) -> Option<Blob>;
}

/// An adapter from the [`FileSystemTrait`] into an FFI-compatible type 
/// that can be used in the Slang API.
#[repr(C)]
pub(crate) struct FileSystemForeignAdapter {
    pub _vtable: &'static sys::IFileSystemVtable,
    pub ref_count: AtomicU32,
    pub user: Box<dyn FileSystemTrait>,
}

impl FileSystemForeignAdapter {
    const VTABLE: sys::IFileSystemVtable = sys::IFileSystemVtable {
        _base: ICastableVtable{
            _base: ISlangUnknown__bindgen_vtable{
                ISlangUnknown_queryInterface: Self::query_interface,
                ISlangUnknown_addRef: Self::add_ref,
                ISlangUnknown_release: Self::release,
            },
            castAs: Self::cast_as,
        },
	    loadFile: Self::load_file,
    };

    pub(crate) fn new(fs: Box<dyn FileSystemTrait>) -> *mut sys::ISlangFileSystem {
        let adapter = Box::new(FileSystemForeignAdapter {
            _vtable: &Self::VTABLE,
            ref_count: AtomicU32::new(1),
            user: fs,
        });

        let ptr = Box::into_raw(adapter);

        unsafe { std::mem::transmute(ptr) }
    }

    unsafe extern "C" fn query_interface(this: *mut sys::ISlangUnknown, guid: *const sys::SlangUUID, out_interface: *mut *mut std::ffi::c_void) -> SlangResult {
        let casted = unsafe { Self::cast_as(this as *mut _, guid) };
        if casted.is_null() {
            return -1;
        }

        unsafe { 
            Self::add_ref(casted as *mut _) ;
            out_interface.write(casted);
        };

        0
    }

    unsafe extern "C" fn add_ref(this: *mut sys::ISlangUnknown) -> u32 {
        let fs: &Self = unsafe { std::mem::transmute(this) };
        fs.ref_count.fetch_add(1, Ordering::Acquire) + 1
    }

    unsafe extern "C" fn release(this: *mut sys::ISlangUnknown) -> u32 {
        let fs: &Self = unsafe { std::mem::transmute(this) };
        let count = fs.ref_count.fetch_sub(1, Ordering::Release) - 1;
        if count == 0 {
            // Drop the adapter
            let ptr = this.cast::<Self>();
            
            // SAFETY: this pointer was created in 
            let _ = unsafe { Box::from_raw(ptr) };
        }

        count
    }

    unsafe extern "C" fn cast_as(this: *mut std::ffi::c_void, guid: *const sys::SlangUUID) -> *mut std::ffi::c_void {
        if guid.is_null() {
            return std::ptr::null_mut();
        }

        let guid = unsafe { &*guid };
        if uuid_eq(guid, &FileSystem::IID) {
            this
        } else {
            std::ptr::null_mut()
        }
    }

    unsafe extern "C" fn load_file(this: *mut std::ffi::c_void, path: *const i8, out_blob: *mut *mut sys::ISlangBlob) -> SlangResult {
        let fs: &Self = unsafe { std::mem::transmute(this) };
        let path = unsafe { std::ffi::CStr::from_ptr(path).to_str().unwrap() };

        match fs.user.load_file(path) {
            Some(blob) => {
                unsafe { out_blob.write(blob.as_raw()); }
                
                // SAFETY: The this blob's ownership is transferred from Rust to C++.
                //
                // Slang is responsible for calling release() on the blob.
                std::mem::forget(blob);

                0
            }
            None => -1,
        }
    }
}
