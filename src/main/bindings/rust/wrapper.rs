/* automatically generated by rust-bindgen */

pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PluginPtr {
    pub val: u64,
}
#[test]
fn bindgen_test_layout_PluginPtr() {
    assert_eq!(
        ::std::mem::size_of::<PluginPtr>(),
        8usize,
        concat!("Size of: ", stringify!(PluginPtr))
    );
    assert_eq!(
        ::std::mem::align_of::<PluginPtr>(),
        8usize,
        concat!("Alignment of ", stringify!(PluginPtr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PluginPtr>())).val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginPtr),
            "::",
            stringify!(val)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _SysCallReg {
    pub as_i64: i64,
    pub as_u64: u64,
    pub as_ptr: PluginPtr,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout__SysCallReg() {
    assert_eq!(
        ::std::mem::size_of::<_SysCallReg>(),
        8usize,
        concat!("Size of: ", stringify!(_SysCallReg))
    );
    assert_eq!(
        ::std::mem::align_of::<_SysCallReg>(),
        8usize,
        concat!("Alignment of ", stringify!(_SysCallReg))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallReg>())).as_i64 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallReg),
            "::",
            stringify!(as_i64)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallReg>())).as_u64 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallReg),
            "::",
            stringify!(as_u64)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallReg>())).as_ptr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallReg),
            "::",
            stringify!(as_ptr)
        )
    );
}
pub type SysCallReg = _SysCallReg;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _SysCallArgs {
    pub number: ::std::os::raw::c_long,
    pub args: [SysCallReg; 6usize],
}
#[test]
fn bindgen_test_layout__SysCallArgs() {
    assert_eq!(
        ::std::mem::size_of::<_SysCallArgs>(),
        56usize,
        concat!("Size of: ", stringify!(_SysCallArgs))
    );
    assert_eq!(
        ::std::mem::align_of::<_SysCallArgs>(),
        8usize,
        concat!("Alignment of ", stringify!(_SysCallArgs))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallArgs>())).number as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallArgs),
            "::",
            stringify!(number)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallArgs>())).args as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallArgs),
            "::",
            stringify!(args)
        )
    );
}
pub type SysCallArgs = _SysCallArgs;
pub const SysCallReturnState_SYSCALL_DONE: SysCallReturnState = 0;
pub const SysCallReturnState_SYSCALL_BLOCK: SysCallReturnState = 1;
pub const SysCallReturnState_SYSCALL_NATIVE: SysCallReturnState = 2;
pub type SysCallReturnState = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _SysCallCondition {
    _unused: [u8; 0],
}
pub type SysCallCondition = _SysCallCondition;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _SysCallReturn {
    pub state: SysCallReturnState,
    pub retval: SysCallReg,
    pub cond: *mut SysCallCondition,
}
#[test]
fn bindgen_test_layout__SysCallReturn() {
    assert_eq!(
        ::std::mem::size_of::<_SysCallReturn>(),
        24usize,
        concat!("Size of: ", stringify!(_SysCallReturn))
    );
    assert_eq!(
        ::std::mem::align_of::<_SysCallReturn>(),
        8usize,
        concat!("Alignment of ", stringify!(_SysCallReturn))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallReturn>())).state as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallReturn),
            "::",
            stringify!(state)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallReturn>())).retval as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallReturn),
            "::",
            stringify!(retval)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallReturn>())).cond as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallReturn),
            "::",
            stringify!(cond)
        )
    );
}
pub type SysCallReturn = _SysCallReturn;
pub type size_t = ::std::os::raw::c_ulong;
pub type pid_t = __pid_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Thread {
    _unused: [u8; 0],
}
pub type Thread = _Thread;
extern "C" {
    pub fn thread_ref(thread: *mut Thread);
}
extern "C" {
    pub fn thread_unref(thread: *mut Thread);
}
extern "C" {
    pub fn thread_run(
        thread: *mut Thread,
        argv: *mut *mut ::std::os::raw::c_char,
        envv: *mut *mut ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn thread_resume(thread: *mut Thread);
}
extern "C" {
    pub fn thread_terminate(thread: *mut Thread);
}
extern "C" {
    pub fn thread_getReturnCode(thread: *mut Thread) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn thread_getReadablePtr(
        thread: *mut Thread,
        plugin_src: PluginPtr,
        n: size_t,
    ) -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn thread_getReadableString(
        thread: *mut Thread,
        plugin_src: PluginPtr,
        n: size_t,
        str_: *mut *const ::std::os::raw::c_char,
        strlen: *mut size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn thread_getWriteablePtr(
        thread: *mut Thread,
        plugin_src: PluginPtr,
        n: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn thread_getMutablePtr(
        thread: *mut Thread,
        plugin_src: PluginPtr,
        n: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn thread_flushPtrs(thread: *mut Thread);
}
extern "C" {
    pub fn thread_nativeSyscall(
        thread: *mut Thread,
        n: ::std::os::raw::c_long,
        ...
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn thread_mallocPluginPtr(thread: *mut Thread, size: size_t) -> PluginPtr;
}
extern "C" {
    pub fn thread_freePluginPtr(thread: *mut Thread, ptr: PluginPtr, size: size_t);
}
extern "C" {
    pub fn thread_isRunning(thread: *mut Thread) -> bool;
}
extern "C" {
    pub fn thread_getProcessId(thread: *mut Thread) -> u32;
}
extern "C" {
    pub fn thread_getHostId(thread: *mut Thread) -> u32;
}
extern "C" {
    pub fn thread_getNativePid(thread: *mut Thread) -> pid_t;
}
extern "C" {
    pub fn thread_getNativeTid(thread: *mut Thread) -> pid_t;
}
extern "C" {
    pub fn thread_getID(thread: *mut Thread) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn thread_clone(
        thread: *mut Thread,
        flags: ::std::os::raw::c_ulong,
        child_stack: PluginPtr,
        ptid: PluginPtr,
        ctid: PluginPtr,
        newtls: ::std::os::raw::c_ulong,
        child: *mut *mut Thread,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn thread_setTidAddress(thread: *mut Thread, addr: PluginPtr);
}
extern "C" {
    pub fn thread_isLeader(thread: *mut Thread) -> bool;
}
