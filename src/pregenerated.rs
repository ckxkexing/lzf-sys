/* automatically generated by rust-bindgen 0.55.1 */

pub const LZF_VERSION: u32 = 262;
extern "C" {
    pub fn lzf_compress(
        in_data: *const ::std::os::raw::c_void,
        in_len: ::std::os::raw::c_uint,
        out_data: *mut ::std::os::raw::c_void,
        out_len: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn lzf_decompress(
        in_data: *const ::std::os::raw::c_void,
        in_len: ::std::os::raw::c_uint,
        out_data: *mut ::std::os::raw::c_void,
        out_len: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint;
}