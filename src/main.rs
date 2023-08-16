extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadSurface};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, TextureAccess};
use sdl2::surface::Surface;
use std::time::Duration;

mod sprite;
mod window;

pub fn main() -> Result<(), String> {
    let screen_width: u32 = 800;
    let screen_height: u32 = 600;

    let mut window = window::Window::new("Sprite Blitting", screen_width, screen_height)?;

    let _image_context = sdl2::image::init(InitFlag::PNG)?;

    let mut sprite_surface = Surface::from_file("content/SpriteSheet.bmp")
        .map_err(|err| format!("failed to load spritesheet surface: {}", err.to_string()))?;

    sprite_surface.set_color_key(true, Color::RGB(0, 0, 0))?;

    let texture_creator = window.canvas.texture_creator();
    let mut sprite_sheet = texture_creator
        .create_texture_from_surface(sprite_surface)
        .map_err(|err| format!("Failed to create spritesheet texture: {}", err.to_string()))?;

    sprite_sheet.set_blend_mode(BlendMode::Add);

    let window_size = Rect::new(0, 0, window.width, window.height);

    let mut buffer_tex = texture_creator
        .create_texture(
            None,
            TextureAccess::Target,
            window_size.w as u32,
            window_size.h as u32,
        )
        .map_err(|err| format!("Failed to create texture: {}", err.to_string()))?;

    window.set_color(Color::RGB(0, 0, 0));
    window.draw();

    let mut running = true;
    let mut is_animating = false;
    let mut direction = 0;
    let mut locx = 0 as i32;
    let mut locy = 0 as i32;

    while running {
        let start_time_ms = window.timer.ticks() as i32;
        let max_frames = 6;
        let elapsed_frame = (start_time_ms / 60) % max_frames;

        for event in window.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    running = false;
                }
                Event::KeyDown { keycode, .. } => match keycode.unwrap() {
                    Keycode::Escape => {
                        running = false;
                    }
                    Keycode::Right => {
                        if locx <= screen_width as i32 - 32 {
                            locx += 8;
                            is_animating = true;
                            direction = 16;
                        }
                    }
                    Keycode::Left => {
                        if locx > 0 {
                            locx -= 8;
                            is_animating = true;
                            direction = 0;
                        }
                    }
                    Keycode::Up => {
                        if locy > 0 {
                            locy -= 8;
                            is_animating = true;
                        }
                    }
                    Keycode::Down => {
                        if locy < screen_height as i32 - 32 {
                            locy += 8;
                            is_animating = true;
                        }
                    }
                    _ => {}
                },
                Event::KeyUp { keycode, .. } => match keycode.unwrap() {
                    Keycode::Right => {
                        if (locx < screen_width as i32 - 16) && (locx >= 0) {
                            is_animating = false;
                        }
                    }
                    Keycode::Left => {
                        if (locx < screen_width as i32 - 16) && (locx > 0) {
                            is_animating = false;
                        }
                    }
                    Keycode::Up => {
                        if (locy < screen_height as i32 - 16) && (locy > 0) {
                            locy -= 8;
                            is_animating = false;
                        }
                    }
                    Keycode::Down => {
                        if (locy < screen_height as i32 - 16) && (locy >= 0) {
                            locy += 8;
                            is_animating = false;
                        }
                    }
                    _ => {}
                },
                Event::MouseMotion {
                    x, y, xrel, yrel, ..
                } => {
                    println!("Mouse x: {}, y: {}", x, y);
                    println!("Relative x: {}, y: {}", xrel, yrel);
                }
                _ => {}
            }
        }

        window
            .canvas
            .with_texture_canvas(&mut buffer_tex, |tex| {
                let mut src_rect = Rect::new(0, direction, 16, 16);
                tex.clear();
                println!("{}", elapsed_frame);
                if is_animating {
                    src_rect.set_x(elapsed_frame * 16);
                } else {
                    src_rect.set_x(0);
                }
                let dest_rect = Rect::new(locx, locy, 32, 32);
                tex.fill_rect(dest_rect).unwrap();
                tex.copy(&sprite_sheet, src_rect, dest_rect).unwrap();
            })
            .ok()
            .unwrap();

        window.canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        window.canvas.clear();
        window.canvas.copy(&buffer_tex, None, None)?;
        window.canvas.present();
        std::thread::sleep(Duration::new(0, 1000 / 60));
    }

    Ok(())
}
