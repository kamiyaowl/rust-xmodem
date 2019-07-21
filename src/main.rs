use std::io;
use std::fs::File;
use std::io::prelude::*;

struct Image {
    width: usize,
    height: usize,
    channels: usize,
    
    // width -> height -> channels
    // self->data[y][x][c]
    data: Vec<Vec<Vec<u8>>>,
}

impl Image {
    fn new(w: usize, h: usize, c: usize) -> Image {
        assert!(w > 0);
        assert!(h > 0);
        assert!(c > 0);

        Image {
            width: w,
            height: h,
            channels: c,
            data: vec![vec![vec![0; c]; w]; h],
        }
    }
    fn clear(&mut self) -> () {
        let pixel = vec![0, 0, 0];
        self.fill(&pixel);
    }
    fn fill(&mut self, pixel: &Vec<u8>) -> () {
        assert!(self.width > 0);
        assert!(self.height > 0);
        assert!(pixel.len() == self.channels);

        for y in 0..self.height {
            for x in 0..self.width {
                for c in 0..self.channels {
                    self.data[y][x][c] = pixel[c];
                }
            }
        }
    }
}

fn decode_jpeg(binary: &Vec<u8>) {
}

fn main() -> io::Result<()> {
    // #TODO: 切り替えられるように置き換え
    let src = "/Users/user/Documents/rust-xmodem/test-image/sample1.jpeg";
    // read file
    let mut file = File::open(src)?;

    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    println!("Hello, world!");

    let img = Image::new(100, 50, 3);
    
    Ok(())
}
