extern crate aom_av1_sys;
extern crate avif_parser;
extern crate png;
extern crate cvt_yuv_rgb;
use std::fs::File;
use std::io::{Read, Cursor, Write};
use aom_av1_sys::decode::Av1Decoder;
use cvt_yuv_rgb::*;
struct AomImageWrapper{
    image:aom_av1_sys::Image
}
impl cvt_yuv_rgb::YuvCompatible for AomImageWrapper{
    fn y_row(&self, at:usize)->&[u8]{
        let stride = self.image.y_stride()as usize;
        return &self.image.y_plane()[stride * at ..(at + 1) * stride];
    }
    fn y_row_mut(&mut self, at:usize)->&mut [u8]{
        let stride = self.image.y_stride() as usize;
        return &mut self.image.y_plane_mut()[stride * at ..(at + 1) * stride];
    }
    fn u_row(&self, at:usize)->&[u8]{
        let stride = self.image.u_stride() as usize;
        return &self.image.u_plane()[stride * at ..(at + 1) * stride];
    }
    fn u_row_mut(&mut self, at:usize)->&mut [u8]{
        let stride = self.image.u_stride() as usize;
        return &mut self.image.u_plane_mut()[stride  * at ..(at + 1) * stride];
    }
    fn v_row(&self, at:usize)->&[u8]{
        let stride = self.image.v_stride() as usize;
        return &self.image.v_plane()[stride * at ..(at + 1) * stride];
    }
    fn v_row_mut(&mut self, at:usize)->&mut [u8]{
        let stride = self.image.v_stride() as usize;
        return &mut self.image.v_plane_mut()[stride * at .. (at + 1) * stride ];
    }
    fn chroma_shift_x(&self)->u8{
        return self.image.x_chroma_shift() as u8;
    }
    fn chroma_shift_y(&self)->u8{
        return self.image.y_chroma_shift() as u8;
    }
    fn depth(&self)->u8{
        return self.image.depth() as u8;
    }
    fn width(&self)->usize{
        return self.image.width() as usize;
    }
    fn height(&self)->usize{
        return self.image.height() as usize;
    }
}
fn map_err<T:std::fmt::Debug>(err:T){
    eprintln!("{:?}", err);
}
fn main()->Result<(),()>{
    let mut avif_file = File::open("./hato.profile2.12bpc.yuv422.avif").unwrap();
    let mut avif_data  = Vec::new();
    avif_file.read_to_end(&mut avif_data).map_err(map_err)?;
    let avif = avif_parser::parse(Cursor::new(&mut avif_data))?;
    println!("{:?}", avif);
    let mut decoder:Av1Decoder = Av1Decoder::new().map_err(map_err)?;
    if let Some(item) = avif.media_boxes().first(){
        let tmp = &avif_data.as_slice()[item.0 as usize .. (item.0 + item.1) as usize];
        decoder.decode(tmp).map_err(map_err)?;
        let image = decoder.get_frame().ok_or(())?;
        
        let yuv = YUVImageBuilder::new()
        .plane_y(image.y_plane(), image.y_stride())
        .plane_u(image.u_plane(), image.u_stride())
        .plane_v(image.v_plane(), image.v_stride())
        .depth(image.depth() as u32)
        .width(image.d_width())
        .height(image.d_height())
        .chroma_shift_x(image.x_chroma_shift())
        .chroma_shift_y(image.y_chroma_shift())
        .build();
        let color_prime = match image.color_primary(){
            aom_av1_sys::AomColorPrimaries::BT2020=>cvt_yuv_rgb::color_primaries::BT2020,
            aom_av1_sys::AomColorPrimaries::BT601=>cvt_yuv_rgb::color_primaries::BT601,
            aom_av1_sys::AomColorPrimaries::BT709=>cvt_yuv_rgb::color_primaries::BT709,
            _=>{
                unimplemented!();
            }
        };
        // let mut file = File::create("./result.png").map_err(map_err)?;
        // let mut png_image = png::Encoder::new(file, image.d_width(), image.d_height());
        // png_image.set_color(png::ColorType::RGB);
        // png_image.set_depth(png::BitDepth::Sixteen);
        // let mut writer = png_image.write_header().map_err(map_err)?;
        // let mut stream_writer = writer.stream_writer();
        // for color in RGB48Writer::new(YUV2RGB::new(yuv.iter(), color_prime), 16u8){            
        //     stream_writer.write(&color).map_err(map_err)?;
        // }
        // stream_writer.finish().map_err(map_err)?;
        // std::mem::drop(writer);

        let mut image_for_encode = aom_av1_sys::Image::new_as_yuv420(image.d_width(), image.d_height()).unwrap();
        println!("w: {}, h:{}\ndw:{}, dh:{}", 
        image_for_encode.width(), 
        image_for_encode.height(), 
        image_for_encode.d_width(),
        image_for_encode.d_height());
        let mut y_plane = Vec::with_capacity((image_for_encode.y_stride() * image_for_encode.height()) as usize);
        let mut u_plane = Vec::with_capacity((image_for_encode.u_stride() * image_for_encode.height()) as usize);
        let mut v_plane = Vec::with_capacity((image_for_encode.v_stride() * image_for_encode.height()) as usize);
        y_plane.resize((image_for_encode.y_stride() * image_for_encode.height()) as usize, 0u8);
        u_plane.resize((image_for_encode.u_stride() * image_for_encode.height()) as usize, 0u8);
        v_plane.resize((image_for_encode.v_stride() * image_for_encode.height()) as usize, 0u8);
        //method 1
        // let mut yuv_image = YUVImageBuilder::new()
        // .chroma_shift_x(image_for_encode.x_chroma_shift())
        // .chroma_shift_y(image_for_encode.y_chroma_shift())
        // .depth(image_for_encode.depth() as u32)
        // .width(image_for_encode.d_width())
        // .height(image_for_encode.d_height())
        // .plane_y_mut(y_plane.as_mut_slice(), image_for_encode.y_stride())
        // .plane_u_mut(u_plane.as_mut_slice(), image_for_encode.u_stride())
        // .plane_v_mut(v_plane.as_mut_slice(), image_for_encode.v_stride())
        // .build();
        //method 2
        let mut test = AomImageWrapper{image:image_for_encode};
        test.write_yuv(&mut yuv.iter());
        image_for_encode = test.image;
    }
    return Ok(());
}
