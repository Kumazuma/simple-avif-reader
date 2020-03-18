extern crate aom_av1_sys;
extern crate avif_parser;
extern crate bmp;
extern crate cvt_yuv_rgb;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use aom_av1_sys::decode::Av1Decoder;
use cvt_yuv_rgb::*;
fn main()->Result<(), ()>{
    let mut avif_file = File::open("./hato.profile0.8bpc.yuv420.avif").unwrap();
    let mut avif_data  = Vec::new();
    avif_file.read_to_end(&mut avif_data).map_err(|err|{ eprintln!("{:?}", err); return ();})?;
    let avif = avif_parser::parse(Cursor::new(&mut avif_data))?;
    println!("{:?}", avif);
    let mut decoder:Av1Decoder = Av1Decoder::new().map_err(|err|{
        eprintln!("{:?}", err);
        return ();
    })?;
    if let Some(item) = avif.media_boxes().first(){
        let tmp = &avif_data.as_slice()[item.0 as usize .. (item.0 + item.1) as usize];
        eprintln!("{}\n{}\n{}", tmp.len(), item.1, item.1 == tmp.len() as u32);
        decoder.decode(tmp).map_err(|err|{
            eprintln!("{:?}", err);
            return ();
        })?;
        let image = decoder.get_frame().ok_or(())?;
        let mut bmp_image = bmp::Image::new(image.d_width(), image.d_height());
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
        for (index, color) in RGB24Writer::new(YUV2RGB::new(yuv.iter(), color_prime)).enumerate(){
            let index= index as u32;
            let pixel = bmp::Pixel::new(color[0],color[1],color[2]);
            bmp_image.set_pixel(index % image.d_width(), index / image.d_width(),  pixel);
        }
        bmp_image.save("./test-result.bmp").map_err(|err|{
            eprintln!("{:?}", err);
            return ();
        });
    }
    
    println!("Hello, world!");
    return Ok(());
}
