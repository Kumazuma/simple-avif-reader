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
    Reserved0 = 0,  
    BT709 = 1,      
    Unspecifed = 2, 
    Reserved3 = 3,  
    BT470M = 4,    
    BT470BG = 5,  
    BT601 = 6,      
    SMPTE240 = 7,   
    LINEAR = 8,      
    LOG100 = 9,     
    LOG100SQRT10 = 10, 
    IEC61966 = 11, 
    BT1361 = 12,   
    SRGB = 13,      
    BT202010BIT = 14, 
    BT202012BIT = 15, 
    SMPTE2084 = 16,     
    SMPTE428 = 17,      
    HLG = 18,            
    Reserved19 = 19     
}

impl Copy for AomMatrixCoefficientsT{}
#[allow(dead_code)]
#[derive(Clone)]
#[repr(C)]
pub(crate) enum AomMatrixCoefficientsT {
    Identity = 0,    
    BT709 = 1,      
    Unspecified = 2, 
    Reserved3 = 3,  
    FCC = 4,         
    BT470BG = 5,  
    BT601 = 6,      
    Smpte240 = 7,   
    SmpteYCgCo = 8, 
    BT2020NCL =
        9, 
    BT2020CL = 10, 
    Smpte2085 = 11, 
    ChromatNCL =
        12, 
    ChromatCL = 13, 
    ICTCP = 14,      
    Reserved15 = 15 
}

impl Copy for AomChromaSamplePositionT{}
#[allow(dead_code)]
#[derive(Clone)]
#[repr(C)]
pub(crate) enum AomChromaSamplePositionT {
    Unknown = 0,          
    Vertical = 1,         
                                  
    Colocated = 2,        
    Reserved = 3          
} 

impl Copy for AomColorRangeT{}
#[allow(dead_code)]
#[derive(Clone)]
#[repr(C)]
pub(crate) enum AomColorRangeT {
    StudioRange = 0, 
    FullRange = 1    
}       
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
    pub(crate) planes:[*mut u8;3],
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
#[repr(C)]
enum AomBitDepth {
    AomBits8 = 8,
    AomBits10 = 10,
    AomBits12 = 12,
}
#[repr(C)]
struct AomRational {
    num:int,
    den:int
}
#[repr(C)]
enum AomEncPass {
    AomRcOnePass = 0,
    AomRcFirstPass = 1,
    AomRcLastPass = 2
}
#[repr(C)]
enum AomRcMode {
    AomVBR = 0,
    AomCBR = 1,
    AomCQ  = 2, 
    AomQ   = 3   
}
#[repr(C)]
struct AomFixedBuf {
    buf:*mut u8,
    sz:size_t
}
#[repr(C)]
enum AomKfMode {
    AomKfAuto = 1,   
    AomKfDisable = 0
}
#[repr(C)]
struct CfgOptions {
init_by_cfg_file:uint,
super_block_size:uint,
max_partition_size:uint,
min_partition_size:uint,
disable_ab_partition_type:uint,
disable_rect_partition_type:uint,
disable_1to4_partition_type:uint,
disable_flip_idtx:uint,
disable_cdef:uint,
disable_lr:uint,
disable_obmc:uint,
disable_warp_motion:uint,
disable_global_motion:uint,
disable_dist_wtd_comp:uint,
disable_diff_wtd_comp:uint,
disable_inter_intra_comp:uint,
disable_masked_comp:uint,
disable_one_sided_comp:uint,
disable_palette:uint,
disable_intrabc:uint,
disable_cfl:uint,
disable_smooth_intra:uint,
disable_filter_intra:uint,
disable_dual_filter:uint,
disable_intra_angle_delta:uint,
disable_intra_edge_filter:uint,
disable_tx_64x64:uint,
disable_smooth_inter_intra:uint,
disable_inter_inter_wedge:uint,
disable_inter_intra_wedge:uint,
disable_paeth_intra:uint,
disable_trellis_quant:uint,
disable_ref_frame_mv:uint,
reduced_reference_set:uint,
reduced_tx_type_set:uint,
}
#[repr(C)]
struct AomCodecEncCfg {
    g_usage:uint,
    g_threads:uint,
    g_profile:uint,
    g_w:uint,
    g_h:uint,
    g_limit:uint,
    g_forced_max_frame_width:uint,
    g_forced_max_frame_height:uint,
   g_bit_depth:AomBitDepthT,
    g_input_bit_depth:uint,
    g_timebase:AomRational,
   g_error_resilient:AomCodecErFlagsT,
    g_pass:AomEncPass,
    g_lag_in_frames:uint,
    rc_dropframe_thresh:uint,
    rc_resize_mode:uint,
    rc_resize_denominator:uint,
    rc_resize_kf_denominator:uint,
    rc_superres_mode:uint,
    rc_superres_denominator:uint,
    rc_superres_kf_denominator:uint,
    rc_superres_qthresh:uint,
    rc_superres_kf_qthresh:uint,
    rc_end_usage:AomRcMode,
   rc_twopass_stats_in:AomFixedBufT,
   rc_firstpass_mb_stats_in:AomFixedBufT,
    rc_target_bitrate:uint,
    rc_min_quantizer:uint,
    rc_max_quantizer:uint,
    rc_undershoot_pct:uint,
    rc_overshoot_pct:uint,
    rc_buf_sz:uint,
    rc_buf_initial_sz:uint,
    rc_buf_optimal_sz:uint,
    rc_2pass_vbr_bias_pct:uint,
    rc_2pass_vbr_minsection_pct:uint,
    rc_2pass_vbr_maxsection_pct:uint,
   fwd_kf_enabled:int,
    kf_mode:AomKfMode,
    kf_min_dist:uint,
    kf_max_dist:uint,
    sframe_dist:uint,
    sframe_mode:uint,
    large_scale_tile:uint,
    monochrome:uint,
    full_still_picture_hdr:uint,
    save_as_annexb:uint,
   tile_width_count:int,
   tile_height_count:int,
    tile_widths:[int;64],
    tile_heights:[int;64],
    encoder_cfg:CfgOptionsT
  }
pub(crate) type CfgOptionsT =CfgOptions;
pub(crate) type AomKfModeT = AomKfMode;
pub(crate) type AomFixedBufT = AomFixedBuf;
pub(crate) type AomRcModeT = AomRcMode;
pub(crate) type AomEncPassT = AomEncPass;
pub(crate) type AomCodecErFlagsT = u32;
pub(crate) type AomRationalT = AomRational;
pub type AomCodecErrT = AomCodecErr;
pub type AomColorPrimariesT = super::AomColorPrimaries;
pub(crate) type AomImageT = AomImage;
pub(crate) type AomImgFmtT= AomImgFmt;
pub(crate) type AomBitDepthT = AomBitDepth;
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