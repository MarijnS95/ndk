//! Bindings for [`AIBinder`]
#![cfg(feature = "binder")]

// TODO: Move
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[doc(alias = "AParcel")]
pub struct Parcel {}

use std::{
    ffi::CStr,
    os::{fd::BorrowedFd, raw::c_void},
    ptr::NonNull,
};

use crate::utils::abort_on_panic;

/**
 * This is called whenever a new AIBinder object is needed of a specific class.
 *
 * \param args these can be used to construct a new class. These are passed from AIBinder_new.
 * \return this is the userdata representing the class. It can be retrieved using
 * AIBinder_getUserData.
 */
#[doc(alias = "AIBinder_Class_onCreate")]
pub type IBinderClassOnCreate = Box<dyn FnMut(*mut c_void) -> *mut c_void>;

/**
 * This is called whenever an AIBinder object is no longer referenced and needs destroyed.
 *
 * Typically, this just deletes whatever the implementation is.
 *
 * \param userData this is the same object returned by AIBinder_Class_onCreate
 */
#[doc(alias = "AIBinder_Class_onDestroy")]
pub type IBinderClassOnDestroy = Box<dyn FnMut(*mut c_void)>;

/**
 * This is called whenever a transaction needs to be processed by a local implementation.
 *
 * This method will be called after the equivalent of
 * android.os.Parcel#enforceInterface is called. That is, the interface
 * descriptor associated with the AIBinder_Class descriptor will already be
 * checked.
 *
 * \param binder the object being transacted on.
 * \param code implementation-specific code representing which transaction should be taken.
 * \param in the implementation-specific input data to this transaction.
 * \param out the implementation-specific output data to this transaction.
 *
 * \return the implementation-specific output code. This may be forwarded from another service, the
 * result of a parcel read or write, or another error as is applicable to the specific
 * implementation. Usually, implementation-specific error codes are written to the output parcel,
 * and the transaction code is reserved for kernel errors or error codes that have been repeated
 * from subsequent transactions.
 */
#[doc(alias = "AIBinder_Class_onTransact")]
pub type IBinderClassOnTransact =
    Box<dyn FnMut(&IBinder, ffi::transaction_code_t, &Parcel, &mut Parcel) -> ffi::binder_status_t>;

/**
 * Dump information about an AIBinder (usually for debugging).
 *
 * When no arguments are provided, a brief overview of the interview should be given.
 *
 * \param binder interface being dumped
 * \param fd file descriptor to be dumped to, should be flushed, ownership is not passed.
 * \param args array of null-terminated strings for dump (may be null if numArgs is 0)
 * \param numArgs number of args to be sent
 *
 * \return binder_status_t result of transaction (if remote, for instance)
 */
#[doc(alias = "AIBinder_onDump")]
pub type IBinderOnDump =
    Box<dyn FnMut(&IBinder, BorrowedFd<'_>, &mut [&CStr]) -> ffi::binder_status_t>;

/// Represents a type of AIBinder object which can be sent out.
// TODO: This is some sort of singleton per interface_descriptor
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[doc(alias = "AIBinder_Class")]
pub struct IBinderClass {
    ptr: NonNull<ffi::AIBinder_Class>,
}

impl IBinderClass {
    /**
     * This creates a new instance of a class of binders which can be instantiated. This is called one
     * time during library initialization and cleaned up when the process exits or execs.
     *
     * None of these parameters can be null.
     *
     * \param interfaceDescriptor this is a unique identifier for the class. This is used internally for
     * validity checks on transactions. This should be utf-8.
     * \param onCreate see AIBinder_Class_onCreate.
     * \param onDestroy see AIBinder_Class_onDestroy.
     * \param onTransact see AIBinder_Class_onTransact.
     *
     * \return the class object representing these parameters or null on error.
     */
    #[doc(alias = "AIBinder_Class_define")]
    #[must_use]
    pub fn define(
        // TODO: Str lifetime?
        interface_descriptor: &CStr,
        onCreate: IBinderClassOnCreate,
        onDestroy: IBinderClassOnDestroy,
        onTransact: IBinderClassOnTransact,
    ) -> Option<Self> {
        unsafe extern "C" fn onCreate() -> ffi::binder_status_t {
            abort_on_panic(|| {
                // TODO: This is the class itself?
                //       let callback = user_data as *mut
            })
        }

        unsafe extern "C" fn onDestroy() -> ffi::binder_status_t {
            abort_on_panic(|| {
                // TODO: This is the class itself?
                //       let callback = user_data as *mut
            })
        }

        unsafe extern "C" fn onTransact() -> ffi::binder_status_t {
            abort_on_panic(|| {
                // TODO: This is the class itself?
                //       let callback = user_data as *mut
            })
        }

        let class = unsafe {
            ffi::AIBinder_Class_define(
                interface_descriptor.as_ptr(),
                onCreate,
                onDestroy,
                onTransact,
            )
        };
        NonNull::new(class).map(|ptr| Self { ptr })
    }

    /**
     * This sets the implementation of the dump method for a class.
     *
     * If this isn't set, nothing will be dumped when dump is called (for instance with
     * android.os.Binder#dump). Must be called before any instance of the class is created.
     *
     * \param clazz class which should use this dump function
     * \param onDump function to call when an instance of this binder class is being dumped.
     */
    #[doc(alias = "AIBinder_Class_setOnDump")]
    pub fn set_on_dump(&mut self, onDump: IBinderOnDump) {
        // TODO: Store boxed fn?
        unsafe extern "C" fn onDump(
            binder: *mut ffi::AIBinder,
            fd: ::std::os::raw::c_int,
            args: *mut *const ::std::os::raw::c_char,
            numArgs: u32,
        ) -> ffi::binder_status_t {
            let binder = IBinder::from_ptr(binder);
            let fd = unsafe { BorrowedFd::borrow_raw(fd) };

            // TODO: Pull Box<onDump> from

            abort_on_panic(|| {
                onDump(
                    binder,
                    fd,
                    std::slice::from_raw_parts_mut(args, numArgs as usize),
                )
                // TODO: This is the class itself?
                //       let callback = user_data as *mut
            })
        }

        unsafe { ffi::AIBinder_Class_setOnDump(self.ptr.as_ptr(), Some(onDump)) }
    }

    /**
     * This tells users of this class not to use a transaction header. By default, libbinder_ndk users
     * read/write transaction headers implicitly (in the SDK, this must be manually written by
     * android.os.Parcel#writeInterfaceToken, and it is read/checked with
     * android.os.Parcel#enforceInterface). This method is provided in order to talk to legacy code
     * which does not write an interface token. When this is disabled, type safety is reduced, so you
     * must have a separate way of determining the binder you are talking to is the right type. Must
     * be called before any instance of the class is created.
     *
     * WARNING: this API interacts badly with linkernamespaces. For correct behavior, you must
     * use it on all instances of a class in the same process which share the same interface
     * descriptor. In general, it is recommended you do not use this API, because it is disabling
     * type safety.
     *
     * \param clazz class to disable interface header on.
     */
    #[cfg(feature = "api-level-33")]
    #[doc(alias = "AIBinder_Class_disableInterfaceTokenHeader")]
    pub fn disable_interface_token_header(&mut self) {
        unsafe { ffi::AIBinder_Class_disableInterfaceTokenHeader(self.ptr.as_ptr()) }
    }

    /**
     * Retrieve the class descriptor for the class.
     *
     * Available since API level 31.
     *
     * \param clazz the class to fetch the descriptor from
     *
     * \return the class descriptor string. This pointer will never be null; a
     * descriptor is required to define a class. The pointer is owned by the class
     * and will remain valid as long as the class does. For a local class, this will
     * be the same value (not necessarily pointer equal) as is passed into
     * [`AIBinderClass::define()`]. Format is utf-8.
     */
    #[cfg(feature = "api-level-31")]
    #[doc(alias = "AIBinder_Class_getDescriptor")]
    pub fn descriptor(&self) -> &str {
        let res = unsafe { ffi::AIBinder_Class_getDescriptor(self.ptr.as_ptr()) };
        unsafe { CStr::from_ptr(res) }
            .to_str()
            .expect("AIBinder_Class_getDescriptor should return UTF-8 per documentation")
    }
}
