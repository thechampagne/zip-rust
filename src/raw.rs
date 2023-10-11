pub const ZIP_DEFAULT_COMPRESSION_LEVEL: u32 = 6;

pub const ZIP_ENOINIT: i32 = -1;
pub const ZIP_EINVENTNAME: i32 = -2;
pub const ZIP_ENOENT: i32 = -3;
pub const ZIP_EINVMODE: i32 = -4;
pub const ZIP_EINVLVL: i32 = -5;
pub const ZIP_ENOSUP64: i32 = -6;
pub const ZIP_EMEMSET: i32 = -7;
pub const ZIP_EWRTENT: i32 = -8;
pub const ZIP_ETDEFLINIT: i32 = -9;
pub const ZIP_EINVIDX: i32 = -10;
pub const ZIP_ENOHDR: i32 = -11;
pub const ZIP_ETDEFLBUF: i32 = -12;
pub const ZIP_ECRTHDR: i32 = -13;
pub const ZIP_EWRTHDR: i32 = -14;
pub const ZIP_EWRTDIR: i32 = -15;
pub const ZIP_EOPNFILE: i32 = -16;
pub const ZIP_EINVENTTYPE: i32 = -17;
pub const ZIP_EMEMNOALLOC: i32 = -18;
pub const ZIP_ENOFILE: i32 = -19;
pub const ZIP_ENOPERM: i32 = -20;
pub const ZIP_EOOMEM: i32 = -21;
pub const ZIP_EINVZIPNAME: i32 = -22;
pub const ZIP_EMKDIR: i32 = -23;
pub const ZIP_ESYMLINK: i32 = -24;
pub const ZIP_ECLSZIP: i32 = -25;
pub const ZIP_ECAPSIZE: i32 = -26;
pub const ZIP_EFSEEK: i32 = -27;
pub const ZIP_EFREAD: i32 = -28;
pub const ZIP_EFWRITE: i32 = -29;
pub const ZIP_ERINIT: i32 = -30;
pub const ZIP_EWINIT: i32 = -31;
pub const ZIP_EWRINIT: i32 = -32;

#[repr(C)]
pub struct zip_t;

#[allow(warnings)]
extern "C" {
    
    pub fn zip_strerror(errnum: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
    
    pub fn zip_open(
        zipname: *const ::std::os::raw::c_char,
        level: ::std::os::raw::c_int,
        mode: ::std::os::raw::c_char,
    ) -> *mut zip_t;
    
    pub fn zip_openwitherror(
        zipname: *const ::std::os::raw::c_char,
        level: ::std::os::raw::c_int,
        mode: ::std::os::raw::c_char,
        errnum: *mut ::std::os::raw::c_int,
    ) -> *mut zip_t;
    
    pub fn zip_close(zip: *mut zip_t);
    
    pub fn zip_is64(zip: *mut zip_t) -> ::std::os::raw::c_int;
    
    pub fn zip_entry_open(
        zip: *mut zip_t,
        entryname: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    
    pub fn zip_entry_opencasesensitive(
        zip: *mut zip_t,
        entryname: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn zip_entry_openbyindex(zip: *mut zip_t, index: usize) -> ::std::os::raw::c_int;
    
    pub fn zip_entry_close(zip: *mut zip_t) -> ::std::os::raw::c_int;
    
    pub fn zip_entry_name(zip: *mut zip_t) -> *const ::std::os::raw::c_char;
    
    pub fn zip_entry_index(zip: *mut zip_t) -> isize;
    
    pub fn zip_entry_isdir(zip: *mut zip_t) -> ::std::os::raw::c_int;
    
    pub fn zip_entry_size(zip: *mut zip_t) -> ::std::os::raw::c_ulonglong;
    
    pub fn zip_entry_uncomp_size(zip: *mut zip_t) -> ::std::os::raw::c_ulonglong;
    
    pub fn zip_entry_comp_size(zip: *mut zip_t) -> ::std::os::raw::c_ulonglong;
    
    pub fn zip_entry_crc32(zip: *mut zip_t) -> ::std::os::raw::c_uint;
    
    pub fn zip_entry_write(
        zip: *mut zip_t,
        buf: *const ::std::os::raw::c_void,
        bufsize: usize,
    ) -> ::std::os::raw::c_int;
    
    pub fn zip_entry_fwrite(
        zip: *mut zip_t,
        filename: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    
    pub fn zip_entry_read(
        zip: *mut zip_t,
        buf: *mut *mut ::std::os::raw::c_void,
        bufsize: *mut usize,
    ) -> isize;
    
    pub fn zip_entry_noallocread(
        zip: *mut zip_t,
        buf: *mut ::std::os::raw::c_void,
        bufsize: usize,
    ) -> isize;
    
    pub fn zip_entry_fread(
        zip: *mut zip_t,
        filename: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    
    pub fn zip_entry_extract(
        zip: *mut zip_t,
        on_extract: ::std::option::Option<
            unsafe extern "C" fn(
                arg: *mut ::std::os::raw::c_void,
                offset: u64,
                data: *const ::std::os::raw::c_void,
                size: usize,
            ) -> usize,
            >,
        arg: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    
    pub fn zip_entries_total(zip: *mut zip_t) -> isize;
    
    pub fn zip_entries_delete(
        zip: *mut zip_t,
        entries: *const *mut ::std::os::raw::c_char,
        len: usize,
    ) -> isize;
    
    pub fn zip_stream_extract(
        stream: *const ::std::os::raw::c_char,
        size: usize,
        dir: *const ::std::os::raw::c_char,
        on_extract: ::std::option::Option<
            unsafe extern "C" fn(
                filename: *const ::std::os::raw::c_char,
                arg: *mut ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
            >,
        arg: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    
    pub fn zip_stream_open(
        stream: *const ::std::os::raw::c_char,
        size: usize,
        level: ::std::os::raw::c_int,
        mode: ::std::os::raw::c_char,
    ) -> *mut zip_t;
    
    pub fn zip_stream_openwitherror(
        stream: *const ::std::os::raw::c_char,
        size: usize,
        level: ::std::os::raw::c_int,
        mode: ::std::os::raw::c_char,
        errnum: *mut ::std::os::raw::c_int,
    ) -> *mut zip_t;
    
    pub fn zip_stream_copy(
        zip: *mut zip_t,
        buf: *mut *mut ::std::os::raw::c_void,
        bufsize: *mut usize,
    ) -> isize;
    
    pub fn zip_stream_close(zip: *mut zip_t);
    
    pub fn zip_create(
        zipname: *const ::std::os::raw::c_char,
        filenames: *mut *const ::std::os::raw::c_char,
        len: usize,
    ) -> ::std::os::raw::c_int;
    
    pub fn zip_extract(
        zipname: *const ::std::os::raw::c_char,
        dir: *const ::std::os::raw::c_char,
        on_extract_entry: ::std::option::Option<
            unsafe extern "C" fn(
                filename: *const ::std::os::raw::c_char,
                arg: *mut ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
            >,
        arg: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
