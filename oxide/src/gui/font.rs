use sdl3::pixels::Color;
use sdl3::render::{FRect, WindowCanvas};

pub const GLYPH_W: usize = 3;
pub const GLYPH_H: usize = 5;

// Each row is a 3-bit mask (MSB = leftmost column) for a tiny 3x5 pixel font.
// Only the characters an FPS-style overlay needs are defined; anything else renders blank.
fn glyph_rows(c: char) -> [u8; GLYPH_H] {
    match c {
        '0' => [0b111, 0b101, 0b101, 0b101, 0b111],
        '1' => [0b010, 0b110, 0b010, 0b010, 0b111],
        '2' => [0b111, 0b001, 0b111, 0b100, 0b111],
        '3' => [0b111, 0b001, 0b111, 0b001, 0b111],
        '4' => [0b101, 0b101, 0b111, 0b001, 0b001],
        '5' => [0b111, 0b100, 0b111, 0b001, 0b111],
        '6' => [0b111, 0b100, 0b111, 0b101, 0b111],
        '7' => [0b111, 0b001, 0b010, 0b010, 0b010],
        '8' => [0b111, 0b101, 0b111, 0b101, 0b111],
        '9' => [0b111, 0b101, 0b111, 0b001, 0b111],
        'F' => [0b111, 0b100, 0b111, 0b100, 0b100],
        'P' => [0b111, 0b101, 0b111, 0b100, 0b100],
        'S' => [0b111, 0b100, 0b111, 0b001, 0b111],
        ':' => [0b000, 0b010, 0b000, 0b010, 0b000],
        _ => [0; GLYPH_H],
    }
}

/// Draws `text` on `canvas`, top-left corner at (`x`, `y`), each font dot drawn as a
/// `scale`x`scale` block. Leaves the canvas's draw color set to `color` afterwards.
pub fn draw_text(
    canvas: &mut WindowCanvas,
    text: &str,
    x: i32,
    y: i32,
    scale: i32,
    color: Color,
) -> Result<(), Box<dyn std::error::Error>> {
    canvas.set_draw_color(color);

    let mut cursor_x = x;
    for c in text.chars() {
        for (row, bits) in glyph_rows(c).iter().enumerate() {
            for col in 0..GLYPH_W {
                if bits & (1 << (GLYPH_W - 1 - col)) != 0 {
                    let px = cursor_x + col as i32 * scale;
                    let py = y + row as i32 * scale;
                    canvas.fill_rect(FRect::new(px as f32, py as f32, scale as f32, scale as f32))?;
                }
            }
        }
        cursor_x += (GLYPH_W as i32 + 1) * scale; // 1 dot of spacing between glyphs
    }
    Ok(())
}
