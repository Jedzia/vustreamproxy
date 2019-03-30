use std::fs::File;
use std::io::Read;

pub fn load_local_mp3_buffer() -> Vec<u8> {
    //let mut f = File::open("./p.mp3").expect("failed to open mp3 file!");
    let mut filepath = "/media/hdd/jedzia/rust/p.mp3";
    if cfg!(target_os = "windows") {
        filepath = "p.mp3";
    }

    let mut f = File::open(filepath).expect("failed to open mp3 file!");
    //let mut f = File::open(filepath);
    //if let Err(err) = f{
    //    error!("failed to open mp3 file! {}", err);
    //}
    let mut buffer: Vec<u8> = Vec::new();
    f.read_to_end(&mut buffer)
        .expect("failed to read mp3 file.");
    buffer
}
