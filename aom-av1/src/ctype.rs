extern crate libc;


#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq)]
#[repr(C)]
pub enum AomCodecErr{
    Ok = 0,
    Error = 1,
    MemError = 2,
    AbiMisMatch =3 ,
    Incapable =4 ,
    UnsupBitStream =5 ,
    UnsupFeature = 6,
    CorruptFrame = 7,
    InvalidParam = 8,
    ListEnd = 9
}
#[repr(C)]
pub(crate) struct aom_codec_ctx{
    name:*const std::os::raw::c_char,
    iface:pvoid,
    err:AomCodecErrT,
    err_detail:*const  std::os::raw::c_char,
    init_flags:long,
    config:pvoid,
    private:pvoid
}
const AOM_IMG_FMT_PLANAR:isize = 0x100;
const AOM_IMG_FMT_UV_FLIP:isize = 0x200;
const AOM_IMG_FMT_HIGHBITDEPTH:isize = 0x800;

impl Copy for AomImgFmt{}
#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone)]
#[repr(C)]
pub(crate) enum AomImgFmt{
    NONE = 0,
    YV12 = AOM_IMG_FMT_PLANAR | AOM_IMG_FMT_UV_FLIP | 1,
    I420 = AOM_IMG_FMT_PLANAR | 2,
    AOMYV12 = AOM_IMG_FMT_PLANAR | AOM_IMG_FMT_UV_FLIP |3,
    AOMI420 = AOM_IMG_FMT_PLANAR | 4,
    I422 = AOM_IMG_FMT_PLANAR | 5,
    I444 = AOM_IMG_FMT_PLANAR | 6,
    I42016 = AomImgFmt::I420 as isize | AOM_IMG_FMT_HIGHBITDEPTH,
    YV1216 = AomImgFmt::YV12 as isize | AOM_IMG_FMT_HIGHBITDEPTH,
    I42216 = AomImgFmt::I422 as isize | AOM_IMG_FMT_HIGHBITDEPTH,
    I44416 = AomImgFmt::I444 as isize | AOM_IMG_FMT_HIGHBITDEPTH
}

impl Copy for AomTransferCharacteristics{}
#[allow(dead_code)]
#[derive(Clone)]
#[repr(C)]
pub(crate) enum AomTransferCharacteristics {
    Reserved0 = 0,  /**< For future use */
    BT709 = 1,      /**< BT.709 */
    Unspecifed = 2, /**< Unspecified */
    Reserved3 = 3,  /**< For future use */
    BT470M = 4,    /**< BT.470 System M (historical)  */
    BT470BG = 5,  /**< BT.470 System B, G (historical) */
    BT601 = 6,      /**< BT.601 */
    SMPTE240 = 7,   /**< SMPTE 240 M */
    LINEAR = 8,      /**< Linear */
    LOG100 = 9,     /**< Logarithmic (100 : 1 range) */
    LOG100SQRT10 = 10, /**< Logarithmic (100 * Sqrt(10) : 1 range) */
    IEC61966 = 11, /**< IEC 61966-2-4 */
    BT1361 = 12,   /**< BT.1361 */
    SRGB = 13,      /**< sRGB or sYCC*/
    BT202010BIT = 14, /**< BT.2020 10-bit systems */
    BT202012BIT = 15, /**< BT.2020 12-bit systems */
    SMPTE2084 = 16,     /**< SMPTE ST 2084, ITU BT.2100 PQ */
    SMPTE428 = 17,      /**< SMPTE ST 428 */
    HLG = 18,            /**< BT.2100 HLG, ARIB STD-B67 */
    Reserved19 = 19     //**< For future use (values 19-255) */
}
//*!\brief List of supported matrix coefficients */
impl Copy for AomMatrixCoefficientsT{}
#[allow(dead_code)]
#[derive(Clone)]
#[repr(C)]
pub(crate) enum AomMatrixCoefficientsT {
    Identity = 0,    /**< Identity matrix */
    BT709 = 1,      /**< BT.709 */
    Unspecified = 2, /**< Unspecified */
    Reserved3 = 3,  /**< For future use */
    FCC = 4,         /**< US FCC 73.628 */
    BT470BG = 5,  /**< BT.470 System B, G (historical) */
    BT601 = 6,      /**< BT.601 */
    Smpte240 = 7,   /**< SMPTE 240 M */
    SmpteYCgCo = 8, /**< YCgCo */
    BT2020NCL =
        9, /**< BT.2020 non-constant luminance, BT.2100 YCbCr  */
    BT2020CL = 10, /**< BT.2020 constant luminance */
    Smpte2085 = 11, /**< SMPTE ST 2085 YDzDx */
    ChromatNCL =
        12, /**< Chromaticity-derived non-constant luminance */
    ChromatCL = 13, /**< Chromaticity-derived constant luminance */
    ICTCP = 14,      /**< BT.2100 ICtCp */
    Reserved15 = 15 //**< For future use (values 15-255)  */
}
//*!\brief List of chroma sample positions */
impl Copy for AomChromaSamplePositionT{}
#[allow(dead_code)]
#[derive(Clone)]
#[repr(C)]
pub(crate) enum AomChromaSamplePositionT {
    Unknown = 0,          /**< Unknown */
    Vertical = 1,         /**< Horizontally co-located with luma(0, 0)*/
                                  /**< sample, between two vertical samples */
    Colocated = 2,        /**< Co-located with luma(0, 0) sample */
    Reserved = 3          //**< Reserved value */
} //**< alias for enum aom_transfer_function */
//*!\brief List of supported color range */
impl Copy for AomColorRangeT{}
#[allow(dead_code)]
#[derive(Clone)]
#[repr(C)]
pub(crate) enum AomColorRangeT {
    StudioRange = 0, /**< Y [16..235], UV [16..240] */
    FullRange = 1    //**< YUV/RGB [0..255] */
}       //**< alias for enum aom_color_range */
#[repr(C)]
pub(crate) struct aom_metadata_array_t{
    _private: [u8; 0]
}

#[repr(C)]
#[derive(Clone)]
pub(crate)  struct AomImage{
    pub(crate) fmt:AomImgFmt,
    pub(crate) cp:AomColorPrimariesT,
    pub(crate) tc:AomTransferCharacteristicsT,
    pub(crate) mc:AomMatrixCoefficientsT,
    pub(crate) monochrome:int,
    pub(crate) csp:AomChromaSamplePositionT,
    pub(crate) range:AomColorRangeT,
    pub(crate) width:uint,
    pub(crate) height:uint,
    pub(crate) bit_depth:uint,
    pub(crate) displayed_width:uint,
    pub(crate) displayed_height:uint,
    pub(crate) rendering_width:uint,
    pub(crate) rendering_height:uint,
    pub(crate) x_chroma_shift:uint,
    pub(crate) y_chroma_shift:uint,
    pub(crate) planes:[*const u8;3],
    pub(crate) stride:[int;3],
    pub(crate) size:size_t,
    pub(crate) bps:int,
    pub(crate) temporal_id:int,
    pub(crate) spatial_id:int,
    user_priv:pvoid,
    img_data:*const u8,
    img_data_owner:int,
    self_allocd:int,
    metadata:*const aom_metadata_array_t,
    fb_priv:pvoid,
}

pub type AomCodecErrT = AomCodecErr;
pub type AomColorPrimariesT = super::AomColorPrimaries;
pub(crate)  type AomImageT = AomImage;
pub(crate) type AomImgFmtT= AomImgFmt;

pub(crate) type AomTransferCharacteristicsT = AomTransferCharacteristics;

pub(crate) type void = libc::c_void;
pub(crate) type pvoid = *const void;
pub(crate) type long = libc::c_long;
pub(crate) type int = libc::c_int;
pub(crate) type uint = libc::c_uint;
pub(crate) type size_t = libc::size_t;
pub(crate) type AomCodecIterT = *const void;
#[allow(dead_code)]
pub(crate) const AOM_IMAGE_ABI_VERSION:i32 = 6;
#[allow(dead_code)]
pub(crate) const AOM_CODEC_ABI_VERSION:i32 = AOM_IMAGE_ABI_VERSION + 3;
#[allow(dead_code)]
pub(crate) const AOM_DECODER_ABI_VERSION:i32 = AOM_CODEC_ABI_VERSION + 4;