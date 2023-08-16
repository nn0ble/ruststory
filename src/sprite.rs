// Need Sprite Surface which comes from spritesheet
// Need to define location of sprite
// Need amount of frames for animation
// Need src_rect and dest_rect

use sdl2::{
    rect::Rect,
    render::{Canvas, Texture, TextureAccess},
};

pub struct Sprite {
    canvas: Canvas<sdl2::video::Window>,
}

impl Sprite {
    pub fn new(
        sprite_sheet: Texture,
        canvas: Canvas<sdl2::video::Window>,
        tile_size: i32,
        window_w: i32,
        window_h: i32,
    ) -> Result<Sprite, String> {
        let texture_creator = canvas.texture_creator();
        let mut buffer_tex = texture_creator
            .create_texture(
                None,
                TextureAccess::Target,
                window_w as u32,
                window_h as u32,
            )
            .map_err(|err| format!("Failed to create texture: {}", err.to_string()))?;
    }
}
