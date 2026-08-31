use std::sync::Arc;
use std::sync::atomic::AtomicU8;
use std::time::Duration;
use crossbeam_channel::Receiver;
use sdl3::{Sdl, VideoSubsystem};
use sdl3::render::{Canvas, TextureCreator, WindowCanvas, Texture};
use sdl3::video::{Window, WindowContext};
use crate::emulator::ppu::Frame;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::PixelFormatEnum;
use sdl3::rect::Rect;
use crate::settings::GLOB_SETTINGS;

const BG_BYTES : &[u8] = include_bytes!("../../assets/dmg_background.png");
const BG_W : u32 = 311;
const BG_H : u32 = 276;
const SCR_W : u32 = 160;
const SCR_H : u32 = 144;
const SCR_X : i32 = 75;
const SCR_Y : i32 = 70;


pub struct UiRenderer {
    pub sdl: Sdl,
    pub video: VideoSubsystem,
    pub canvas: WindowCanvas,
    pub tex_creator: TextureCreator<WindowContext>,
}

pub struct UiAssets<'tc> {
    pub bg_text: Texture<'tc>,
    pub frame_text: Texture<'tc>,
    
    pub screen_pos: Rect,
}

impl UiRenderer {
    pub fn new() -> Result<UiRenderer, Box<dyn std::error::Error>> {
        let sdl = sdl3::init()?;
        let video = sdl.video()?;
        let window = video.window("OxideGB", 311, 276)
            .position_centered()
            .build()?;
        let canvas = window.into_canvas();
        let tex_creator = canvas.texture_creator();

        Ok(UiRenderer {
            sdl, video, canvas, tex_creator
        })
    }
}

impl<'tc> UiAssets<'tc> {
    pub fn new(creator: &'tc TextureCreator<WindowContext>) -> Result<Self, Box<dyn std::error::Error>> {
        let bg_image = image::load_from_memory(BG_BYTES)?.to_rgba8();
        let mut bg_text = creator.create_texture_streaming(
            Some(PixelFormatEnum::ABGR8888.into()), BG_W, BG_H)?;
        bg_text.set_blend_mode(sdl3::render::BlendMode::Blend);

        bg_text.with_lock(None, |buf, pitch | {
            let src = bg_image.as_raw();

            for y in 0..BG_H as usize {
                let src_row = &src[y * (BG_W as usize) * 4 .. (y + 1) * (BG_W as usize) * 4];
                let dst_row = &mut buf[y * pitch.. y * pitch + (BG_W as usize) * 4];
                dst_row.copy_from_slice(src_row);
            }
        })?;

        let frame_text = creator.create_texture_streaming(
            Some(PixelFormatEnum::ARGB8888.into()), SCR_W, SCR_H)?;
        Ok(Self {
            bg_text,
            frame_text,
            screen_pos: Rect::new(SCR_X, SCR_Y, SCR_W, SCR_H),
        })
    }
    
    pub fn write_frame(&mut self, frame: &Frame) -> Result<(), Box<dyn std::error::Error>>{
        self.frame_text.with_lock(None, |buf, pitch| {
            for y in 0..SCR_H as usize {
                let row = &frame[y * SCR_W as usize .. (y + 1) * SCR_W as usize];
                let dst = &mut buf[y * pitch as usize .. y * pitch as usize + (SCR_W as usize) * 4];

                for (x, &px) in row.iter().enumerate() {
                    let colors = &GLOB_SETTINGS.get().unwrap().colors;
                    let cur_color = colors[px as usize];
                    let r = ((cur_color >> 16) & 0xFF) as u8;
                    let g = ((cur_color >>  8) & 0xFF) as u8;
                    let b = ( cur_color        & 0xFF) as u8;
                    let i = x * 4;
                    // ARGB8888 (little-endian) expects BGRA bytes here:
                    dst[i + 0] = b;
                    dst[i + 1] = g;
                    dst[i + 2] = r;
                    dst[i + 3] = 0xFF;
                }
            }
        })?;
        Ok(())
    }
}

pub fn start_gui(rx_frame: Receiver<Frame>, joystate: Arc<AtomicU8>) -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = UiRenderer::new()?;
    let mut assets = UiAssets::new(&ui.tex_creator)?;
    let mut event_pump = ui.sdl.event_pump()?;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape), ..
                } => break 'main,
                _ => {}
            }
        }

        if let Ok(frame) = rx_frame.try_recv() {
            assets.write_frame(&frame)?;
        }

        ui.canvas.clear();
        ui.canvas.copy(&assets.frame_text, None, assets.screen_pos)?;
        ui.canvas.copy(&assets.bg_text, None, None)?;
        ui.canvas.present();
        
        std::thread::sleep(Duration::from_millis(5));
    }
    Ok(())
}