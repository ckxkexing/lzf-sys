#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! Crate for linking to the native c library [liblzf](http://software.schmorp.de/pkg/liblzf.html).

#[cfg(feature = "paranoid")]
include!(concat!(env!("OUT_DIR"), "/lzf_bindings.rs"));

#[cfg(not(feature = "paranoid"))]
mod pregenerated;
#[cfg(not(feature = "paranoid"))]
pub use pregenerated::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let message = "This very interesting long string which contains much valuable information";

        let mut compressed_buffer = vec![0_u8; message.len() + 1];

        let bytes_used = unsafe {
            lzf_compress(
                message.as_ptr() as *const _,
                message.len() as _,
                compressed_buffer.as_mut_ptr() as *mut _,
                compressed_buffer.len() as _,
            )
        };
        assert_ne!(bytes_used, 0);

        let mut output = vec![0_u8; message.len()];
        let bytes_used = unsafe {
            lzf_decompress(
                compressed_buffer.as_ptr() as *mut _,
                bytes_used,
                output.as_mut_ptr() as *mut _,
                output.len() as _,
            )
        };
        assert_eq!(bytes_used, message.len() as _);

        assert_eq!(message.as_bytes(), &output[..]);
    }
}
