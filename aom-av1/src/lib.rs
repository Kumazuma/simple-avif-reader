extern crate libc;
pub mod decode;
pub mod image;

mod ctype;

use ctype::*;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[link(name="aom", kind="static")]
extern "C"{
    fn aom_img_alloc(it:*mut AomImageT, fmt:ctype::AomImgFmt, d_w:uint, d_h:uint, align:uint)->*mut AomImageT;
    fn aom_codec_av1_cx()-> pvoid;
    fn aom_codec_av1_dx()-> pvoid;
    fn aom_img_free(img: pvoid);
    fn aom_codec_destroy(ctx:*mut aom_codec_ctx);
    fn aom_codec_decode(ctx:*mut aom_codec_ctx, data:*const u8, len: libc::size_t, _:pvoid)->AomCodecErrT;
    fn aom_codec_get_frame(ctx:*mut aom_codec_ctx, iter: *const AomCodecIterT)->*const AomImageT;
    fn aom_codec_dec_init_ver(ctx:*mut aom_codec_ctx, iface:pvoid, cfg:pvoid, flags:long, ver:int)->AomCodecErrT;
    fn aom_codec_enc_config_default(ctx:*mut aom_codec_ctx, cfg:*mut AomCodeEncCfgT, reserved:uint)->AomCodecErrT;
}
pub type Image = image::Image;
impl Copy for AomColorPrimaries{}
#[allow(dead_code)]
#[derive(Clone)]
#[repr(C)]
pub enum AomColorPrimaries {
    Reserved0 = 0,  /**< For future use */
    BT709 = 1,      /**< BT.709 */
    Unspecifed = 2, /**< Unspecified */
    Reserved3 = 3,  /**< For future use */
    BT470M = 4,    /**< BT.470 System M (historical) */
    BT470BG = 5,  /**< BT.470 System B, G (historical) */
    BT601 = 6,      /**< BT.601 */
    SMPTE240 = 7,   /**< SMPTE 240 */
    GenericFilm = 8, /**< Generic film (color filters using illuminant C) */
    BT2020 = 9,      //**< BT.2020, BT.2100 */
    XYZ = 10,         //**< SMPTE 428 (CIE 1921 XYZ) */
    SMPTE431 = 11,   //**< SMPTE RP 431-2 */
    SMPTE432 = 12,   //**< SMPTE EG 432-1  */
    Reserved13 = 13, //**< For future use (values 13 - 21)  */
    EBU3213 = 22,    //**< EBU Tech. 3213-E  */
    Reserved23 = 23  //**< For future use (values 23 - 255)  */
}       //**< alias for enum aom_color_primaries */
