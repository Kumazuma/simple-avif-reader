extern crate libc;

use super::ctype::*;
use super::decode::Av1Decoder;
use super::*;
pub struct Image{
    image:AomImageT,
    image_ptr:*const AomImageT
}
impl Image{
    pub(crate) fn from_aom_image_t(data:*const AomImageT)->Self{
        Self{
            image:unsafe{data.as_ref().unwrap().clone()},
            image_ptr:data
        }
    }
    pub fn width(&self)->u32{
        self.image.width
    }
    pub fn height(&self)->u32{
        self.image.height
    }
    pub fn d_width(&self)->u32{
        self.image.displayed_width
    }
    pub fn d_height(&self)->u32{
        self.image.displayed_height
    }
    pub fn y_plane(&self)->&[u8]{
        unsafe{
            let len = self.image.stride[0] as u32 * self.image.height;
            &*std::ptr::slice_from_raw_parts(self.image.planes[0], len as usize)
        }
    }
    pub fn u_plane(&self)->&[u8]{
        unsafe{
            let len = self.image.stride[1] as u32 * self.image.height;
            &*std::ptr::slice_from_raw_parts(self.image.planes[1], len as usize)
        }
    }
    pub fn v_plane(&self)->&[u8]{
        unsafe{
            let len = self.image.stride[2] as u32 * self.image.height;
            &*std::ptr::slice_from_raw_parts(self.image.planes[2], len as usize)
        }
    }
    pub fn y_stride(&self)->u32{
        self.image.stride[0] as u32
    }
    pub fn u_stride(&self)->u32{
        self.image.stride[1] as u32
    }
    pub fn v_stride(&self)->u32{
        self.image.stride[2] as u32
    }
    pub fn depth(&self)->u8{
        self.image.bit_depth as u8
    }
    pub fn x_chroma_shift(&self)->u8{
        self.image.x_chroma_shift as u8
    }
    pub fn y_chroma_shift(&self)->u8{
        self.image.y_chroma_shift as u8
    }
    pub fn color_primary(&self)->AomColorPrimariesT{
        self.image.cp
    }
    pub fn y_plane_mut(&mut self)->&mut [u8]{
        unsafe{
            let len = self.image.stride[0] as u32 * self.image.height;
            &mut *std::ptr::slice_from_raw_parts_mut(self.image.planes[0], len as usize)
        }
    }
    pub fn u_plane_mut(&mut self)->&mut [u8]{
        unsafe{
            let len = self.image.stride[1] as u32 * self.image.height;
            &mut *std::ptr::slice_from_raw_parts_mut(self.image.planes[1], len as usize)
        }
    }
    pub fn v_plane_mut(&mut self)->&mut [u8]{
        unsafe{
            let len = self.image.stride[2] as u32 * self.image.height;
            &mut *std::ptr::slice_from_raw_parts_mut(self.image.planes[2], len as usize)
        }
    }
    fn new(fmt:ctype::AomImgFmt, width:u32, height:u32)->Option<Self>{
        unsafe{
            let mut img:AomImageT = std::mem::MaybeUninit::zeroed().assume_init();
            if aom_img_alloc(&mut img, fmt, width, height, 1) == std::ptr::null_mut(){
                return None;
            }
            let obj = Self{
                image_ptr:&img,
                image:img
            };
            return Some(obj);
        }
    }
    pub fn new_as_yuv420(width:u32, height:u32)->Option<Self> {
        Self::new(ctype::AomImgFmtT::I420, width, height)
    }
    pub fn new_as_yuv422(width:u32, height:u32)->Option<Self> {
        Self::new(ctype::AomImgFmtT::I422, width, height)
    }
    pub fn new_as_yuv444(width:u32, height:u32)->Option<Self> {
        Self::new(ctype::AomImgFmtT::I444, width, height)
    }
    pub fn new_as_yuv42016(width:u32, height:u32)->Option<Self> {
        Self::new(ctype::AomImgFmtT::I42016, width, height)
    }
    pub fn new_as_yuv42216(width:u32, height:u32)->Option<Self> {
        Self::new(ctype::AomImgFmtT::I42216, width, height)
    }
    pub fn new_as_yuv44416(width:u32, height:u32)->Option<Self> {
        Self::new(ctype::AomImgFmtT::I44416, width, height)
    }
}
impl Drop for Image{
    fn drop(&mut self){
        unsafe{
            aom_img_free(self.image_ptr as pvoid);
        }
    }
}
