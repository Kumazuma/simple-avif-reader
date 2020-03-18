extern crate aom_av1_sys;
extern crate avif_parser;
extern crate png;
extern crate cvt_yuv_rgb;
use std::fs::File;
use std::io::{Read, Cursor, Write};
use aom_av1_sys::decode::Av1Decoder;
use cvt_yuv_rgb::*;
fn map_err<T:std::fmt::Debug>(err:T){
    eprintln!("{:?}", err);
}
fn main()->Result<(), ()>{

    let mut avif_file = File::open("./hato.profile0.8bpc.yuv420.avif").unwrap();
    let mut avif_data  = Vec::new();
    avif_file.read_to_end(&mut avif_data).map_err(map_err)?;
    let avif = avif_parser::parse(Cursor::new(&mut avif_data))?;
    println!("{:?}", avif);
    let mut decoder:Av1Decoder = Av1Decoder::new().map_err(map_err)?;
    if let Some(item) = avif.media_boxes().first(){
        let tmp = &avif_data.as_slice()[item.0 as usize .. (item.0 + item.1) as usize];
        decoder.decode(tmp).map_err(map_err)?;
        let image = decoder.get_frame().ok_or(())?;
        let mut file = File::create("./result.png").map_err(map_err)?;
        let mut png_image = png::Encoder::new(file, image.d_width(), image.d_height());
        png_image.set_color(png::ColorType::RGB);
        png_image.set_depth(png::BitDepth::Eight);
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
        let mut writer = png_image.write_header().map_err(map_err)?;
        let mut stream_writer = writer.stream_writer();
        for (index, color) in RGB24Writer::new(YUV2RGB::new(yuv.iter(), color_prime)).enumerate(){
            let index= index as u32;
            stream_writer.write(&color).map_err(map_err)?;
        }
        stream_writer.finish().map_err(map_err)?;
        std::mem::drop(writer);
    }
    return Ok(());
}
