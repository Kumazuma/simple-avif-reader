extern crate libc;

use super::ctype::*;
use super::*;
pub struct Av1Encoder
{
    encode_context:aom_codec_ctx
}
use std::mem::MaybeUninit;
impl Av1Encoder{
    pub fn new()->Result<Self,AomCodecErrT>{
        unsafe{
            let encoder_interface = aom_codec_av1_cx();
            let mut encode_context:aom_codec_ctx = MaybeUninit::zeroed().assume_init();
            let res = aom_codec_dec_init_ver(&mut encode_context, decoder_interface, std::ptr::null(), 0, AOM_DECODER_ABI_VERSION);
            if res != AomCodecErrT::Ok {
                return Err(res);
            }
            Ok(Self{
                encode_context:encode_context
            })
        }
    }
    pub fn add_image(&mut self, data:&[u8], )->Result<(), AomCodecErrT>{
        unsafe{
            let length = data.len();
            let res = aom_codec_decode(&mut self.decode_context, data.as_ptr(), length, std::ptr::null());
            if res != AomCodecErrT::Ok{
                return Err(res);
            }
            return Ok(());
        }
    }
    pub fn flush(&mut self){
        
    }
}
impl Drop for Av1Encoder{
    fn drop(&mut self){
        unsafe{
            aom_codec_destroy(&mut self.encode_context);
        }
    }
}