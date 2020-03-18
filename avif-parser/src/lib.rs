#[cfg(test)]
mod tests {
    use std::io::Read;
    use std::io::Cursor;
    #[test]
    fn it_works() {
        let bytes = [1u8,2u8,3u8,4u8,5u8];
        let mut stream = Cursor::new( &bytes);
        let mut res:[u8;2] = [0u8;2];
        while let Ok(read_count) = stream.read(&mut res){
            if read_count == 0{
                break;
            }
            for i in 0..read_count{
                print!("0x{:02X} ", res[i]);
            }
            print!("\n");
        }
    }
}
use std::io::Read;
use std::io::Write;
use std::io::Cursor;
#[derive(Debug)]
pub struct Avif{
    media_boxes:Vec<(u32, u32)>
}

impl Avif{
    pub fn new()->Self{
        Self{
            media_boxes:vec![]
        }
    }
    pub fn media_boxes(&self)->&[(u32, u32)]{
        self.media_boxes.as_slice()
    }
}
struct Header{
    box_type:u32,
    offset:u32,
    size:u32
}
impl Header{
    fn end(&self)->u32{
        self.offset + self.size
    }
}
fn read_u32<Reader>(reader:&mut Cursor<Reader>)->Option<u32>
where Reader:AsRef<[u8]>{
    let mut data = [0u8;4];
    let mut written_count = 0;
    return loop{
        match reader.read(&mut data[written_count..]){
            Ok(read_count)=>{
                written_count += read_count;
                if written_count == 4{
                    let res = 
                    (data[0] as u32) << 24 |
                    (data[1] as u32) << 16 |
                    (data[2] as u32) << 8 |
                    (data[3] as u32) << 0;
                    break Some(res);
                }
                else if read_count == 0{
                    break None;
                }
            }
            Err(_)=>break None
        }
    };
}
const fn boy_type(text:&'static [u8;4])->u32{
    (text[0] as u32) << 24 |
    (text[1] as u32) << 16 |
    (text[2] as u32) << 8 |
    (text[3] as u32) << 0
}
fn cvt2readable(box_type:u32)->String{
    let tmp:[u8;4] = [
        (box_type >> 24) as u8,
        (box_type >> 16)  as u8,
        (box_type >> 8)  as u8,
        (box_type >> 0)  as u8
    ];
    return String::from_utf8_lossy(&tmp).to_string();
}
const MEDIA_DATA_BOX:u32 = boy_type(b"mdat");
const SKIP_BOX:u32 = boy_type(b"free");
const FREE_BOX:u32 = boy_type(b"skip");
const META_BOX:u32 = boy_type(b"meta");
const FILE_TYPE_BOX:u32 = boy_type(b"ftyp");
enum BoxType{
    MediaDataBox = MEDIA_DATA_BOX as isize,
    SkipBox = SKIP_BOX as isize,
    FreeBox = FREE_BOX as isize,
    MetaBox = META_BOX as isize,
    FileTypeBox = FILE_TYPE_BOX as isize,
}
fn read_header<Reader>(reader:&mut Cursor<Reader>)->Option<Header>
where Reader:AsRef<[u8]>{
    let offset = reader.position();
    let size = read_u32(reader)?;
    let box_type = read_u32(reader)?;

    return Some(Header{box_type:box_type, offset:offset as u32, size:size});
}
pub fn parse<Reader>(mut reader:Cursor<Reader>)->Result<Avif, ()>
where Reader:AsRef<[u8]>{
    let mut avif = Avif::new();
    let len =  reader.get_ref().as_ref().len();
    eprintln!("len is {}", len);
    while let Some(mut header) = read_header(&mut reader){
        eprintln!("{} {} {}",cvt2readable(header.box_type), header.offset, header.size);
        header.size = if header.size > 1{
            eprintln!("header.size > 1" );
            header.size
        }
        else if header.size == 0{
            eprintln!("header.size  == 0" );
            len as u32 - header.offset
        }
        else{
            return Err(());
        };
        match header.box_type{
            MEDIA_DATA_BOX=>{
                eprintln!("MEDIA_DATA_BOX");
                let offset = reader.position() as u32;
                eprintln!("{} {}", header.offset, offset);
                avif.media_boxes.push((offset, header.end() - offset));
            },
            META_BOX=>{
                eprintln!("META_BOX");
            }
            FILE_TYPE_BOX=>{
                eprintln!("FILE_TYPE_BOX");
            }
            SKIP_BOX|FREE_BOX=>{
                eprintln!("FREE_BOX|SKIP_BOX");
            }
            _=>{

            }
        }
        reader.set_position(header.end() as u64);
    }
    return Ok(avif);
}