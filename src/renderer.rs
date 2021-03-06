#![allow(clippy::unusual_byte_groupings)]

use std::cmp;

use skia_safe::{
    Paint, PaintStyle, Path, ISize, Rect,
    FontMgr, Font,
};

use crate::{sheet_state::*, sheet::{CellIdx}};

const FONT_NAME: &'static str = "DejaVu Sans Mono";
const CELL_SIZE: (usize, usize) = (80, 20);

fn col_to_letters(col: usize) -> String {
    let mut scratch = col-1;
    let mut text = String::new();
    loop {
        let current: u8 = (scratch % 26) as u8;
        text.insert(0, (current+65) as char);
        scratch /=  26;
        if scratch == 0 {
            break;
        }
        scratch -= 1;
    }

    text
}

fn cell_rect(offset: Option<&CellIdx>, idx: &CellIdx) -> Rect {
    const NONE_OFFSET: CellIdx = CellIdx{col: 0, row: 0};
    let offset = match offset {
        Some(o) => o,
        _ => &NONE_OFFSET
    };
    let x = (idx.col as f32 - offset.col as f32) * CELL_SIZE.0 as f32;
    let y = (idx.row as f32 - offset.row as f32) * CELL_SIZE.1 as f32;

    Rect::new(x+1.0, y+1.0, (x+CELL_SIZE.0 as f32 - 1.0) as f32, (y+CELL_SIZE.1 as f32 - 1.0) as f32)
}

fn calc_lines_col_width() -> f32 {
    let mgr = FontMgr::new();
    let typeface = mgr.match_family_style(FONT_NAME, skia_safe::FontStyle::normal()).unwrap();
    let font = Font::new(typeface, Some(14.0));
    let (_, bounds) = font.measure_str("0000", None);
    bounds.width() * 1.2
}

fn render_grid(canvas: &mut skia_safe::canvas::Canvas, size: &ISize, state: &mut SheetState) {
    const STROKE: f32 = 1.0;

    let mgr = FontMgr::new();
    let typeface = mgr.match_family_style(FONT_NAME, skia_safe::FontStyle::normal()).unwrap();
    let font = Font::new(typeface, Some(14.0));

    let lines_col_width = calc_lines_col_width();

    let text_paint = Paint::default();

    if (size.width as f32) < (lines_col_width + (CELL_SIZE.0 as f32) * 2.0) || (size.height as f32) < ((CELL_SIZE.1 as f32) * 3.0) {
        // Too small
        return;
    }

    {
        loop {
            let mut changed = false;
            let rect = cell_rect(Some(&state.view_offset), &state.selected);
            if rect.left() < 0.0 {
                state.view_offset.col -= 1;
                changed = true;
            }

            if rect.right() > (size.width as f32 - lines_col_width) {
                state.view_offset.col += 1;
                changed = true;
            }

            if rect.top() < 0.0 {
                state.view_offset.row -= 1;
                changed = true;
            }

            if rect.bottom() > (size.height - CELL_SIZE.1 as i32) as f32 {
                state.view_offset.row += 1;
                changed = true;
            }

            if !changed { break; }
        }

        // Protect from overflow
        state.selected.col = cmp::max(state.selected.col, state.view_offset.col);
        state.selected.row = cmp::max(state.selected.row, state.view_offset.row);
    }

    // Headers
    {
        let mut paint = Paint::default();
        //paint.set_stroke_width(STROKE);
        paint.set_color(0xff_e5e5e5);

        canvas.draw_rect(Rect::new(0.0, 0.0, size.width as f32, CELL_SIZE.1 as f32), &paint);
        canvas.draw_rect(Rect::new(0.0, 0.0, lines_col_width as f32, size.height as f32), &paint);

        // Selected
        {
            let mut selected_paint =  Paint::default();
            selected_paint.set_color(0xff_25a3fc);
            let x = ((state.selected.col-state.view_offset.col) * CELL_SIZE.0 as u32) as f32 + lines_col_width;
            let y = ((state.selected.row-state.view_offset.row+1) * CELL_SIZE.1 as u32) as f32;
            canvas.draw_rect(Rect::new(x, 0.0, x+CELL_SIZE.0 as f32, CELL_SIZE.1 as f32), &selected_paint);
            canvas.draw_rect(Rect::new(0.0, y, lines_col_width as f32, y+CELL_SIZE.1 as f32), &selected_paint);
        }

        // Draw rows
        for j in 1..((size.height / CELL_SIZE.1 as i32) as usize) {
            let y = (j * CELL_SIZE.1) as f32;

            let text = (j + state.view_offset.row as usize).to_string();
            let (_, bounds) = font.measure_str(text.as_str(), None);
            canvas.draw_str(text.as_str(), ((lines_col_width-bounds.width())/2.0, (y + CELL_SIZE.1 as f32) - bounds.height() / 2.0), &font, &text_paint);
        }

        // Draw cols
        for i in 0..((size.width / CELL_SIZE.0 as i32) as usize) {
            let x = (i * CELL_SIZE.0) as f32;
            let text = col_to_letters(i+1 + state.view_offset.col as usize);
            let (_, bounds) = font.measure_str(text.as_str(), None);
            canvas.draw_str(text.as_str(), (lines_col_width as f32 + x + (CELL_SIZE.0 as f32-bounds.width())/2.0, CELL_SIZE.1 as f32 - bounds.height()/2.0), &font, &text_paint);
        }

    }

    // Main grid
    {
        canvas.save();
        canvas.translate((lines_col_width as f32, CELL_SIZE.1 as f32));

        let mut paint = Paint::default();
        paint.set_stroke_width(STROKE);
        paint.set_color(0xff_c1c1c1);

        paint.set_style(PaintStyle::Stroke);

        let mut path = Path::new();
        // Draw rows
        for j in 0..((size.height / CELL_SIZE.1 as i32) as usize) {
            let y = j * CELL_SIZE.1;
            path.move_to((0.0f32, y as f32));
            path.line_to((size.width as f32, y as f32));
        }

        // Draw cols
        for i in 0..((size.width / CELL_SIZE.0 as i32) as usize) {
            let x = i * CELL_SIZE.0;
            path.move_to((x as f32, 0.0f32));
            path.line_to((x as f32, size.height as f32));
        }
        canvas.draw_path(&path, &paint);

        // Selected
        {
            let mut paint = Paint::default();
            paint.set_stroke_width(2.0);
            paint.set_style(PaintStyle::Stroke);

            canvas.draw_rect(cell_rect(Some(&state.view_offset), &state.selected), &paint);
        }

        // Values
        {
            for j in 0..((size.height / CELL_SIZE.1 as i32) as u32) {
                for i in 0..((size.width / CELL_SIZE.0 as i32) as u32) {

                    let rect = {
                        // Idx for rect
                        let idx = CellIdx{col: i, row: j};
                        cell_rect(None, &idx)
                    };

                    // Idx for value
                    let idx = CellIdx{col: i, row: j} + state.view_offset.clone();

                    let text = state.get_value(&idx);
                    if !text.is_empty() {
                        let str = text.as_str();
                        let (_, bounds) = font.measure_str(str, None);

                        if rect.width() > bounds.width() {
                            canvas.draw_str(str, (rect.left() + (rect.width() - bounds.width())/2.0, rect.top() + (rect.height() + bounds.height())/2.0), &font, &text_paint);
                        } else {
                            canvas.save();
                            canvas.clip_rect(rect, None, None);
                            canvas.draw_str(str, (rect.left() + 2.0, rect.top() + (rect.height() + bounds.height())/2.0), &font, &text_paint);
                            canvas.restore();
                        }

                    }
                }
            }
        }

        canvas.restore();
    }
}

fn render_input(canvas: &mut skia_safe::canvas::Canvas, size: &ISize, state: &SheetState) {
    {
        let mut paint = Paint::default();
        paint.set_stroke_width(2.0);
        paint.set_style(PaintStyle::Stroke);

        canvas.draw_rect(Rect::new(1.0, 1.0, (size.width-1) as f32, (size.height-1) as f32), &paint);
    }

    {
        let text_paint = Paint::default();

        let mgr = FontMgr::new();
        let typeface = mgr.match_family_style(FONT_NAME, skia_safe::FontStyle::normal()).unwrap();
        let mut font = Font::new(typeface, Some(18.0));
        font.set_subpixel(true);


        let splt = state.text.split('\r');

        let mut offset = 8.0f32;
        for txt in splt {
            let txt_measure = if !txt.is_empty() {
                txt
            } else {
                "X"
            };
            let (_, bounds) = font.measure_str(txt_measure, None);
            canvas.draw_str(txt, (8.0, bounds.height() + offset) , &font, &text_paint);
            offset += bounds.height() + 4.0;
        }
    }
}

pub fn render(canvas: &mut skia_safe::canvas::Canvas, state: &mut SheetState) {
    let full_size = canvas.image_info().dimensions();

    let input_size = ISize{width: (full_size.width as f32 * 0.3) as i32, height: full_size.height};
    let grid_size = ISize{width: full_size.width - input_size.width, height: full_size.height};

    canvas.reset_matrix();
    //canvas.clip_rect(Rect::from_isize(input_size), None, None);
    render_input(canvas, &input_size, state);

    //canvas.reset_matrix();
    //canvas.clip_rect(Rect::new(0.0, input_size.height as f32, full_size.width as f32, full_size.height as f32), None, None);
    canvas.save();
    canvas.translate((input_size.width as f32, 0.0));
    render_grid(canvas, &grid_size, state);
    canvas.restore();

}

#[cfg(test)]
mod tests {
    use skia_safe::Canvas;

    use super::*;

    #[test]
    fn basic_rendering() {
        for s in [(0, 0), (10, 10), (100, 100), (400, 400), (800, 400), (1920, 1080), (7680, 4320)] {
            let size = ISize{width: s.0, height: s.1};
            let mut canvas = Canvas::new(size, None).unwrap();

            let mut state = SheetState::new();

            // Render the grid
            render_grid(&mut canvas, &size, &mut state);
        }
    }

    #[test]
    fn test_selection_handling() {
        let size = ISize{width: 1920, height: 1080};
        let mut canvas = Canvas::new(size, None).unwrap();

        let mut state = SheetState::new();

        // Validate initial state
        assert_eq!(state.selected.col, 0);
        assert_eq!(state.selected.row, 0);
        assert_eq!(state.view_offset.col, 0);
        assert_eq!(state.view_offset.row, 0);

        // Render the grid
        render_grid(&mut canvas, &size, &mut state);

        // Move X selection far far away
        state.selected.col = 1000;
        render_grid(&mut canvas, &size, &mut state);

        assert_eq!(state.selected.col, 1000);
        assert_eq!(state.selected.row, 0);
        assert_ne!(state.view_offset.col, 0);
        assert_eq!(state.view_offset.row, 0);
        assert!(state.view_offset.col < state.selected.col);

        // Move Y selection far far away
        state.selected.row = 1000;
        render_grid(&mut canvas, &size, &mut state);
        assert_eq!(state.selected.col, 1000);
        assert_eq!(state.selected.row, 1000);
        assert_ne!(state.view_offset.col, 0);
        assert_ne!(state.view_offset.row, 0);
        assert!(state.view_offset.col < state.selected.col);
        assert!(state.view_offset.row < state.selected.row);

        // Move X selection back to zero
        state.selected.col = 0;
        render_grid(&mut canvas, &size, &mut state);

        assert_eq!(state.selected.col, 0);
        assert_eq!(state.selected.row, 1000);
        assert_eq!(state.view_offset.col, 0);
        assert_ne!(state.view_offset.row, 0);
        assert!(state.view_offset.row < state.selected.row);

        // Move Y selection back to zero
        state.selected.row = 0;
        render_grid(&mut canvas, &size, &mut state);
        assert_eq!(state.selected.col, 0);
        assert_eq!(state.selected.row, 0);
        assert_eq!(state.view_offset.col, 0);
        assert_eq!(state.view_offset.row, 0);
    }
}
