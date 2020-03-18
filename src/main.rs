extern crate libc;

// #[derive(PartialEq)]
// #[repr(C)]
// enum aom_codec_err_t{
//     AOM_CODEC_OK,
//     AOM_CODEC_ERROR,
//     AOM_CODEC_MEM_ERROR,
//     AOM_CODEC_ABI_MISMATCH,
//     AOM_CODEC_INCAPABLE,
//     AOM_CODEC_UNSUP_BITSTREAM,
//     AOM_CODEC_UNSUP_FEATURE,
//     AOM_CODEC_CORRUPT_FRAME,
//     AOM_CODEC_INVALID_PARAM,
//     AOM_CODEC_LIST_END
// }
// #[repr(C)]
// struct aom_codec_ctx{
//     name:*const std::os::raw::c_char,
//     iface:pvoid,
//     err:aom_codec_err_t,
//     err_detail:*const  std::os::raw::c_char,
//     config:pvoid,
//     private:pvoid
// }
// type void = std::os::raw::c_void;
// type pvoid = *const void;
// type long = libc::c_long;
// type int = libc::c_int;
// type aom_codec_iter_t = *const void;
// const AOM_IMAGE_ABI_VERSION:i32 = 6;
// const AOM_CODEC_ABI_VERSION:i32 = AOM_IMAGE_ABI_VERSION + 3;
// const AOM_DECODER_ABI_VERSION:i32 = AOM_CODEC_ABI_VERSION + 4;
// #[link(name="aom", kind="static")]
// extern{
//     fn aom_codec_av1_dx()-> pvoid;
//     fn aom_img_free(img: pvoid);
//     fn aom_codec_destroy(codec:&mut aom_codec_ctx);
//     fn aom_codec_decode(codec:&mut aom_codec_ctx, data:*const u8, len: libc::size_t, _:pvoid)->aom_codec_err_t;
//     fn aom_codec_get_frame(codec:&mut aom_codec_ctx, iter: *const aom_codec_iter_t)->pvoid;
//     fn aom_codec_dec_init_ver(ctx:&mut aom_codec_ctx, iface:pvoid, cfg:pvoid, flags:long, ver:int)->aom_codec_err_t;
// }

// struct Av1Decoder
// {
//     decode_context:aom_codec_ctx
// }
// use std::mem::MaybeUninit;
// impl Av1Decoder{
//     fn new()->Result<Self,aom_codec_err_t>{
//         unsafe{
//             let decoder_interface = aom_codec_av1_dx();
//             let mut decode_context:aom_codec_ctx = MaybeUninit::zeroed().assume_init();
//             let res = aom_codec_dec_init_ver(&mut decode_context, decoder_interface, std::ptr::null(), 0, AOM_DECODER_ABI_VERSION);
//             if res != aom_codec_err_t::AOM_CODEC_OK {
//                 return Err(res);
//             }
//             Ok(Self{
//                 decode_context:decode_context
//             })
//         }
//     }
//     fn decode(&mut self, data:&[u8])->Result<(), aom_codec_err_t>{
//         unsafe{
//             let res = aom_codec_decode(&mut self.decode_context, data.as_ptr(), data.len(), std::ptr::null());
//             if res != aom_codec_err_t::AOM_CODEC_OK{
//                 return Err(res);
//             }
//             return Ok(());
//         }
//     }
//     fn get_frame(&mut self)->Option<()>{
//         let mut s:aom_codec_iter_t = std::ptr::null();
//         unsafe{
//             let res = aom_codec_get_frame(&mut self.decode_context, &s);
//             if res == std::ptr::null(){
//                 return None;
//             }
//         }
//         return None;
//     }
// }
fn main() {
    let decoder =unsafe{aom_codec_av1_dx()};
    println!("Hello, world!");
}
