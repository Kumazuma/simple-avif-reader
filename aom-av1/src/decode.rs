extern crate libc;

use super::ctype::*;
use super::*;
pub struct Av1Decoder
{
    decode_context:aom_codec_ctx
}
use std::mem::MaybeUninit;
impl Av1Decoder{
    pub fn new()->Result<Self,AomCodecErrT>{
        unsafe{
            let decoder_interface = aom_codec_av1_dx();
            let mut decode_context:aom_codec_ctx = MaybeUninit::zeroed().assume_init();
            let res = aom_codec_dec_init_ver(&mut decode_context, decoder_interface, std::ptr::null(), 0, AOM_DECODER_ABI_VERSION);
            if res != AomCodecErrT::Ok {
                return Err(res);
            }
            Ok(Self{
                decode_context:decode_context
            })
        }
    }
    pub fn decode(&mut self, data:&[u8])->Result<(), AomCodecErrT>{
        unsafe{
            let length = data.len();
            let res = aom_codec_decode(&mut self.decode_context, data.as_ptr(), length, std::ptr::null());
            if res != AomCodecErrT::Ok{
                return Err(res);
            }
            return Ok(());
        }
    }
    pub fn get_frame(&mut self)->Option<Image>{
        let mut s:AomCodecIterT= std::ptr::null();
        unsafe{
            let res = aom_codec_get_frame(&mut self.decode_context, &s);
            if res == std::ptr::null(){
                return None;
            }
            return Some(Image::from_aom_image_t(res));
        }
        
    }
}
impl Drop for Av1Decoder{
    fn drop(&mut self){
        unsafe{
            aom_codec_destroy(&mut self.decode_context);
        }
    }
}