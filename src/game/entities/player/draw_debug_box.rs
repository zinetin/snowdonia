use crate::game::entities::player::Player;
use crate::vars::DEBUG_FONT_SIZE;
use macroquad::prelude::*;

impl Player {
    pub fn draw_debug_box(&self) {
        // {:#.1?} = pretty-print, floats to 1 decimal place
        let text = format!("{:#.1?}", self);
        let lines: Vec<&str> = text.lines().collect();

        let font_size = DEBUG_FONT_SIZE as f32;
        let line_h = font_size * 1.2;
        let pad = 8.0;

        // Width of the widest line, so the box fits the text
        let text_w = lines
            .iter()
            .map(|l| measure_text(l, None, DEBUG_FONT_SIZE, 1.0).width)
            .fold(0.0, f32::max);

        let box_w = text_w + pad * 2.0;
        let box_h = lines.len() as f32 * line_h + pad * 2.0;
        let x = screen_width() - box_w - pad;
        let y = pad;

        draw_rectangle(x, y, box_w, box_h, Color::new(0.0, 0.0, 0.0, 0.7));
        draw_rectangle_lines(x, y, box_w, box_h, 2.0, WHITE);

        for (i, line) in lines.iter().enumerate() {
            // draw_text's y is the text baseline, so add font_size to sit inside the box
            draw_text(
                line,
                x + pad,
                y + pad + i as f32 * line_h + font_size,
                font_size,
                WHITE,
            );
        }
    }
}
