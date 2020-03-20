extern crate libc;

use super::ctype::*;
use super::decode::Av1Decoder;
use super::*;
pub struct Image{
    image_ptr:*mut AomImageT,
    free_method:fn(*mut AomImageT)
}
fn get_ref<'a, T>(ptr:* mut T)->&'a mut T{
    unsafe{
        ptr.as_mut().unwrap()
    }
}
fn non_free(_:*mut AomImageT){
    
}
fn need_free(ptr:*mut AomImageT){
    unsafe{
        aom_img_free(ptr as pvoid);
        let s = Box::from_raw(ptr);
        std::mem::drop(s);
    }
}
impl Image{
    pub(crate) fn from_aom_image_t(data:*mut AomImageT)->Self{
        Self{
            image_ptr:data,
            free_method:non_free
        }
    }
    pub fn width(&self)->u32{
        get_ref(self.image_ptr).width
    }
    pub fn height(&self)->u32{
        get_ref(self.image_ptr).height
    }
    pub fn d_width(&self)->u32{
        get_ref(self.image_ptr).displayed_width
    }
    pub fn d_height(&self)->u32{
        get_ref(self.image_ptr).displayed_height
    }
    pub fn y_plane(&self)->&[u8]{
        let image = get_ref(self.image_ptr);
        unsafe{
            let len = image.stride[0] as u32 * image.height;
            &*std::ptr::slice_from_raw_parts(image.planes[0], len as usize)
        }
    }
    pub fn u_plane(&self)->&[u8]{
        let image = get_ref(self.image_ptr);
        unsafe{
            let len = image.stride[1] as u32 * image.height;
            &*std::ptr::slice_from_raw_parts(image.planes[1], len as usize)
        }
    }
    pub fn v_plane(&self)->&[u8]{
        let image = get_ref(self.image_ptr);
        unsafe{
            let len = image.stride[2] as u32 * image.height;
            &*std::ptr::slice_from_raw_parts(image.planes[2], len as usize)
        }
    }
    pub fn y_stride(&self)->u32{
        let image = get_ref(self.image_ptr);
        image.stride[0] as u32
    }
    pub fn u_stride(&self)->u32{
        let image = get_ref(self.image_ptr);
        image.stride[1] as u32
    }
    pub fn v_stride(&self)->u32{
        let image = get_ref(self.image_ptr);
        image.stride[2] as u32
    }
    pub fn depth(&self)->u8{
        let image = get_ref(self.image_ptr);
        image.bit_depth as u8
    }
    pub fn x_chroma_shift(&self)->u8{
        let image = get_ref(self.image_ptr);
        image.x_chroma_shift as u8
    }
    pub fn y_chroma_shift(&self)->u8{
        let image = get_ref(self.image_ptr);
        image.y_chroma_shift as u8
    }
    pub fn color_primary(&self)->AomColorPrimariesT{
        let image = get_ref(self.image_ptr);
        image.cp
    }
    pub fn y_plane_mut(&mut self)->&mut [u8]{
        let image = get_ref(self.image_ptr);
        unsafe{
            let len = image.stride[0] as u32 * image.height;
            &mut *std::ptr::slice_from_raw_parts_mut(image.planes[0], len as usize)
        }
    }
    pub fn u_plane_mut(&mut self)->&mut [u8]{
        let image = get_ref(self.image_ptr);
        unsafe{
            let len = image.stride[1] as u32 * image.height;
            &mut *std::ptr::slice_from_raw_parts_mut(image.planes[1], len as usize)
        }
    }
    pub fn v_plane_mut(&mut self)->&mut [u8]{
        let image = get_ref(self.image_ptr);
        unsafe{
            let len = image.stride[2] as u32 * image.height;
            &mut *std::ptr::slice_from_raw_parts_mut(image.planes[2], len as usize)
        }
    }
    fn new(fmt:ctype::AomImgFmt, width:u32, height:u32)->Option<Self>{
        unsafe{
            let mut img = Box::<AomImageT>::new(std::mem::MaybeUninit::zeroed().assume_init());
            if aom_img_alloc(img.as_mut(), fmt, width, height, 1) == std::ptr::null_mut(){
                return None;
            }
            let obj = Self{
                image_ptr:Box::into_raw(img),
                free_method:need_free
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
        let free_method = self.free_method;
        free_method(self.image_ptr);
    }
}
