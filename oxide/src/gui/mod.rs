use std::sync::Arc;
use std::sync::atomic::AtomicU8;
use crossbeam_channel::Receiver;
use sdl3::{Sdl, VideoSubsystem};
use sdl3::render::{Canvas, TextureCreator, WindowCanvas, Texture};
use sdl3::video::{Window, WindowContext};
use crate::emulator::ppu::Frame;
use image::GenericImageView;
use sdl3::pixels::PixelFormatEnum;

const BG_BYTES : &[u8] = include_bytes!("../../assets/dmg_background.png");
const BG_W : u32 = 311;
const BG_H : u32 = 276;
const SCR_W : u32 = 160;
const SCR_H : u32 = 144;
const SCR_X : i32 = 75;
const SCR_Y : i32 = 70;
pub struct Sdl_Ui {
    pub sdl: Sdl,
    pub video: VideoSubsystem,
    pub canvas: WindowCanvas,
    pub tex_creator: TextureCreator<WindowContext>,
}

impl Sdl_Ui {
    pub fn new() -> Result<Sdl_Ui, Box<dyn std::error::Error>> {
        let sdl = sdl3::init()?;
        let video = sdl.video()?;
        let window = video.window("OxideGB", 311, 276)
            .position_centered()
            .build()?;
        let canvas = window.into_canvas();
        let tex_creator = canvas.texture_creator();

        Ok(Sdl_Ui {
            sdl, video, canvas, tex_creator
        })
    }

    fn get_bg_texture(&mut self) -> Result<Texture, Box<dyn std::error::Error>> {
        let bg_image = image::load_from_memory(BG_BYTES)?.to_rgba8();
        let mut bg_tex = self.tex_creator.create_texture_streaming(
            Some(PixelFormatEnum::ABGR8888.into()), BG_W, BG_H)?;
        bg_tex.set_blend_mode(sdl3::render::BlendMode::Blend);

        bg_tex.with_lock(None, |buf, pitch | {
            let src = bg_image.as_raw();

            for y in 0..BG_H as usize {
                let src_row = &src[y * (BG_W as usize) * 4 .. (y + 1) * (BG_W as usize) * 4];
                let dst_row = &mut buf[y * pitch as usize .. y * pitch as usize + (BG_W as usize) * 4];
                dst_row.copy_from_slice(src_row);
            }
        })?;
        Ok(bg_tex)
    }
    
}

fn write_frame(tex: &mut Texture, frame: &Frame) {
    tex.with_lock(None, |buf, pitch| {
        for y in 0..SCR_H as usize {
            let row = &frame[y * SCR_W as usize .. (y + 1) * SCR_W as usize];
            let dst = &mut buf[y * pitch as usize .. y * pitch as usize + (SCR_W as usize) * 4];

            for (x, &px) in row.iter().enumerate() {
                let r = ((px >> 16) & 0xFF) as u8;
                let g = ((px >>  8) & 0xFF) as u8;
                let b = ( px        & 0xFF) as u8;
                let i = x * 4;
                // ARGB8888 (little-endian) expects BGRA bytes here:
                dst[i + 0] = b;
                dst[i + 1] = g;
                dst[i + 2] = r;
                dst[i + 3] = 0xFF;
            }
        }
    }).unwrap();
}

pub fn start_gui(rx_frame: Receiver<Frame>, joystate: Arc<AtomicU8>) {
    let mut ui = Sdl_Ui::new().unwrap();
    let scren_text = ui.tex_creator.create_texture_streaming(
        Some(PixelFormatEnum::ARGB8888.into()), SCR_W, SCR_H);
    let bg_text = ui.get_bg_texture().unwrap();
        
}