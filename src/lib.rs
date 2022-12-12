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

#[allow(unused)]
fn decompress(s:&String) -> Option<String>{
    let mut c_size : usize = s.len();
    let mut u_size : usize = 0;
    let src = s.as_bytes();
    let mut sp = 0;
    return if c_size > 0 {
        if src[0] > 0 {
            if (src[0] & 0x80) == 0 && c_size >= 1 {
                c_size -= 1;
                u_size = src[sp] as usize & 0xff;
                sp += 1;
            } else if (src[0] & 0x20) == 0 && c_size >= 2 {
                c_size -= 2;
                u_size = src[sp] as usize & 0x1f;
                sp += 1;
                u_size = (u_size << 6) | (src[sp] as usize & 0x3f);
                sp += 1;
            } else if (src[0] & 0x10) == 0 && c_size >= 3 {
                c_size -= 3;
                u_size = src[sp] as usize & 0x0f;
                sp += 1;
                u_size = (u_size << 6) | (src[sp] as usize & 0x3f);
                sp += 1;
                u_size = (u_size << 6) | (src[sp] as usize & 0x3f);
                sp += 1;
            } else if (src[0] & 0x08) == 0 && c_size >= 4 {
                c_size -= 4;
                u_size = src[sp] as usize & 0x07;
                sp += 1;
                u_size = (u_size << 6) | (src[sp] as usize & 0x3f);
                sp += 1;
                u_size = (u_size << 6) | (src[sp] as usize & 0x3f);
                sp += 1;
                u_size = (u_size << 6) | (src[sp] as usize & 0x3f);
                sp += 1;
            } else if (src[0] & 0x04) == 0 && c_size >= 5 {
                c_size -= 5;
                u_size = src[sp] as usize & 0x03;
                sp += 1;
                u_size = (u_size << 6) | (src[sp] as usize & 0x3f);
                sp += 1;
                u_size = (u_size << 6) | (src[sp] as usize & 0x3f);
                sp += 1;
                u_size = (u_size << 6) | (src[sp] as usize & 0x3f);
                sp += 1;
                u_size = (u_size << 6) | (src[sp] as usize & 0x3f);
                sp += 1;
            } else if (src[0] & 0x02) == 0 && c_size >= 6 {
                c_size -= 6;
                u_size = src[sp] as usize & 0x01;
                sp += 1;
                u_size = (u_size << 6) | (src[sp] as usize & 0x3f);
                sp += 1;
                u_size = (u_size << 6) | (src[sp] as usize & 0x3f);
                sp += 1;
                u_size = (u_size << 6) | (src[sp] as usize & 0x3f);
                sp += 1;
                u_size = (u_size << 6) | (src[sp] as usize & 0x3f);
                sp += 1;
                u_size = (u_size << 6) | (src[sp] as usize & 0x3f);
                sp += 1;
            } else {
                return None;
            }

            if u_size == 0 {
                return None;
            }
            let u_size = u_size as usize;
            let output = vec![0_u8; u_size];
            // let res = decompress_in_rust(&src[sp..], u_size);
            let res = unsafe {
                lzf_decompress(src[sp..].as_ptr() as _, c_size as _, output.as_ptr() as _, u_size as _)
            };
            if res as usize != u_size {
                return None;
            }
            Some(String::from_utf8(output).unwrap())
        } else {
            u_size = c_size - 1;
            Some(String::from(&s[1..]))
        }
    } else {
        Some(String::new())
    }
}
#[allow(unused)]
// lzf decompress in rust from https://github.com/badboy/lzf-rs/blob/5715fe6571d6ad6da7201253a79ceb7ca411a67a/src/decompress.rs#L20
fn decompress_in_rust(data: &[u8], out_len_should: usize) -> Vec<u8> {
    let mut current_offset = 0;

    let in_len = data.len();
    if in_len == 0 {
        println!("lzfError::DataCorrupted 0");
    }

    // We have sanity checks to not exceed this capacity.
    let mut output = vec![0; out_len_should];
    let mut out_len: usize = 0;

    while current_offset < in_len {
        let mut ctrl = data[current_offset] as usize;
        current_offset += 1;

        if ctrl < (1 << 5) {
            ctrl += 1;

            if out_len + ctrl > out_len_should {
                println!("lzfError::BufferTooSmall");
            }

            if current_offset + ctrl > in_len {
                println!("lzfError::DataCorrupted 0");
            }

            // We can simply memcpy everything from the input to the output
            output[out_len..(out_len + ctrl)]
                .copy_from_slice(&data[current_offset..(current_offset + ctrl)]);

            current_offset += ctrl;
            out_len += ctrl;
        } else {
            let mut len = ctrl >> 5;
            let mut ref_offset = (((ctrl & 0x1f) << 8) + 1) as i64;
            if current_offset >= in_len {
                println!("lzfError::DataCorrupted");
            }

            if len == 7 {
                len += data[current_offset] as usize;
                current_offset += 1;

                if current_offset >= in_len {
                    println!("lzfError::DataCorrupted");
                }
            }

            ref_offset += data[current_offset] as i64;
            current_offset += 1;

            if out_len + len + 2 > out_len_should {
                println!("lzfError::DataCorrupted");
            }

            let mut ref_pos = (out_len as i64) - ref_offset;
            // println!("ref pos = {}", ref_pos);
            if ref_pos < 0 {
                println!("lzfError::DataCorrupted");
            }

            let c = output[ref_pos as usize];
            output[out_len] = c;
            out_len += 1;
            ref_pos += 1;

            let c = output[ref_pos as usize];
            output[out_len] = c;
            out_len += 1;
            ref_pos += 1;

            while len > 0 {
                let c = output[ref_pos as usize];
                output[out_len] = c;
                out_len += 1;
                ref_pos += 1;
                len -= 1;
            }
        }
    }

    // Set the real length now, user might have passed a bigger buffer in the first place.
    unsafe { output.set_len(out_len) };
    output
}
#[cfg(test)]
mod tests {
    #[cfg(feature = "paranoid")]
    use crate::*;

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
    use std::io::Read;
    use std::io::BufReader;
    use std::fs;
    #[test]
    fn test_decompress(){
        let path = "./compressed_results.txt";
        let f = fs::File::open(path).unwrap();
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

        // Read file into vector.
        reader.read_to_end(&mut buffer).unwrap();
        let s = unsafe {
            String::from_utf8_unchecked(buffer)
        };

        let res1 = decompress(&s).unwrap();
        println!("======================================\n{}\n======================================", res1);
        assert_eq!(res1.len(), 869_usize);
    }

}