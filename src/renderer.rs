#![allow(clippy::unusual_byte_groupings)]

use skia_safe::{
    Paint, PaintStyle, Path, ISize, Rect,
    FontMgr, Font,
};

use crate::sheet_state::*;

const CELL_SIZE: (usize, usize) = (80, 20);

fn col_to_letters(col: usize) -> String {
    let mut scratch = col-1;
    let mut text = String::new();
    loop {
        let current: u8 = (scratch % 26) as u8;
        text.insert(0, (current+65) as char);
        //text.push((current+65) as char);
        scratch /=  26;
        if scratch == 0 {
            break;
        }
        scratch -= 1;
    } 
    
    text
}

fn render_grid(canvas: &mut skia_safe::canvas::Canvas, size: &ISize, state: &SheetState) {
    const STROKE: f32 = 1.0;
  
    let mgr = FontMgr::new();
    let typeface = mgr.match_family_style("DejaVu Sans Mono", skia_safe::FontStyle::normal()).unwrap();
    let font = Font::new(typeface, Some(14.0));
    let (_, bounds) = font.measure_str("0000", None);
    let lines_col_width = bounds.width() * 1.2;

    // Headers
    {
        let mut paint = Paint::default();
        //paint.set_stroke_width(STROKE);
        paint.set_color(0xff_e5e5e5);
                            
        canvas.draw_rect(Rect::new(0.0, 0.0, size.width as f32, CELL_SIZE.1 as f32), &paint);
        canvas.draw_rect(Rect::new(0.0, 0.0, lines_col_width as f32, size.height as f32), &paint);

        {
            let mut selected_paint =  Paint::default();
            selected_paint.set_color(0xff_25a3fc);
            let x = (state.selected.col * CELL_SIZE.0 as u32) as f32 + lines_col_width;
            let y = ((state.selected.row+1) * CELL_SIZE.1 as u32) as f32;
            canvas.draw_rect(Rect::new(x, 0.0, x+CELL_SIZE.0 as f32, CELL_SIZE.1 as f32), &selected_paint);
            canvas.draw_rect(Rect::new(0.0, y, lines_col_width as f32, y+CELL_SIZE.1 as f32), &selected_paint);
        }

        let text_paint = Paint::default();

        // Draw rows
        for j in 1..((size.height / CELL_SIZE.1 as i32) as usize) {
            let y = (j * CELL_SIZE.1) as f32;

            let text = j.to_string();
            let (_, bounds) = font.measure_str(text.as_str(), None);
            canvas.draw_str(text.as_str(), ((lines_col_width-bounds.width())/2.0, (y + CELL_SIZE.1 as f32) - bounds.height() / 2.0), &font, &text_paint);
        }

        // Draw cols
        for i in 0..((size.width / CELL_SIZE.0 as i32) as usize) {
            let x = (i * CELL_SIZE.0) as f32;
            let text = col_to_letters(i+1);
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
            let x = (state.selected.col * CELL_SIZE.0 as u32) as f32;
            let y = (state.selected.row * CELL_SIZE.1 as u32) as f32;
            let mut paint = Paint::default();
            paint.set_stroke_width(2.0);
            paint.set_style(PaintStyle::Stroke);

            canvas.draw_rect(Rect::new(x+1.0, y+1.0, (x+CELL_SIZE.0 as f32 - 1.0) as f32, (y+CELL_SIZE.1 as f32 - 1.0) as f32), &paint);
        }

        canvas.restore();
    }


}

fn render_input(canvas: &mut skia_safe::canvas::Canvas, size: &ISize) {
    let mut paint = Paint::default();
    
    paint.set_stroke_width(2.0);
                        
    paint.set_style(PaintStyle::Stroke);

    canvas.draw_rect(Rect::new(1.0, 1.0, (size.width-1) as f32, (size.height-1) as f32), &paint);
    // let mut path = Path::new();

    // path.move_to((1.0, 1.0));
    // path.line_to(((size.width-1) as f32, 1.0));
    // path.line_to(((size.width-1) as f32, (size.height-1) as f32));
    // path.line_to((1.0, (size.height-1) as f32));
    // path.close();

    // canvas.draw_path(&path, &paint);
}

pub fn render(canvas: &mut skia_safe::canvas::Canvas, state: &SheetState) {
    let full_size = canvas.image_info().dimensions();

    let input_size = ISize{width: full_size.width, height: 40};
    let grid_size = ISize{width: full_size.width, height: full_size.height-input_size.height};

    canvas.reset_matrix();
    //canvas.clip_rect(Rect::from_isize(input_size), None, None);
    render_input(canvas, &input_size);

    //canvas.reset_matrix();
    //canvas.clip_rect(Rect::new(0.0, input_size.height as f32, full_size.width as f32, full_size.height as f32), None, None);
    canvas.save();
    canvas.translate((0.0, input_size.height as f32));
    render_grid(canvas, &grid_size, &state);
    canvas.restore();

    
}

/*
use std::cmp::min;

const PI: f32 = std::f32::consts::PI;
const DEGREES_IN_RADIANS: f32 = PI / 180.0;
const PEN_SIZE: f32 = 1.0;

fn point_in_circle(center: (f32, f32), radius: f32, radians: f32) -> (f32, f32) {
    (
        center.0 + radius * radians.cos(),
        center.1 - radius * radians.sin(),
    )
}

fn chain_ring(
    canvas: &mut skia_safe::canvas::Canvas,
    center: (i32, i32),
    radius: i32,
    rotation: f32,
    teeth_count: i32,
) {
    canvas.save();
    canvas.translate(Point::from(center));
    canvas.save();
    canvas.rotate(rotation, None);

    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_stroke_width(PEN_SIZE.max(canvas.image_info().dimensions().width as f32 / 360.0));

    let center = (0, 0);
    let c = (center.0 as f32, center.1 as f32);
    let outer_radius = radius as f32;
    let inner_radius = outer_radius * 0.73;
    let ridge_radius = outer_radius * 0.85;
    let teeth_length = (outer_radius - ridge_radius) * 0.8;

    let delta = 2.0 * PI / (teeth_count as f32);
    let teeth_bottom_gap = 0.2 * delta;

    let mut alpha = PI / 2.0;
    let mut path = Path::new();
    for i in 0..teeth_count {
        let mut a = alpha - delta / 2.0 + teeth_bottom_gap / 2.0;
        let v = point_in_circle(c, outer_radius - teeth_length, a);
        if i == 0 {
            path.move_to(v);
        } else {
            path.line_to(v);
        }
        let middle = a + (delta - teeth_bottom_gap) / 2.0;
        a += delta - teeth_bottom_gap;
        path.cubic_to(
            point_in_circle(c, outer_radius * 1.035, middle),
            point_in_circle(c, outer_radius * 1.035, middle),
            point_in_circle(c, outer_radius - teeth_length, a),
        );
        a += teeth_bottom_gap;
        path.line_to(point_in_circle(c, outer_radius - teeth_length, a));

        alpha += delta;
    }
    path.close();

    let delta = -2.0 * PI / 5.0;
    let teeth_bottom_gap = 0.70 * delta;

    alpha = PI / 2.0;
    for i in 0..5 {
        let mut a = alpha - delta / 2.0 + teeth_bottom_gap / 2.0;
        let v = point_in_circle(c, inner_radius, a);
        if i == 0 {
            path.move_to(v);
        } else {
            path.line_to(v);
        }
        let middle = a + (delta - teeth_bottom_gap) / 2.0;
        a += delta - teeth_bottom_gap;
        path.cubic_to(
            point_in_circle(c, inner_radius - teeth_length * 1.33, middle),
            point_in_circle(c, inner_radius - teeth_length * 1.33, middle),
            point_in_circle(c, inner_radius, a),
        );
        a += teeth_bottom_gap;
        path.cubic_to(
            point_in_circle(c, inner_radius * 1.05, a - teeth_bottom_gap * 0.67),
            point_in_circle(c, inner_radius * 1.05, a - teeth_bottom_gap * 0.34),
            point_in_circle(c, inner_radius, a),
        );

        alpha += delta;
    }
    path.close();

    let bolt_radius = inner_radius * 0.81 * (delta - teeth_bottom_gap) / delta / PI;
    alpha = PI / 2.0;
    for _i in 0..5 {
        let c = point_in_circle(c, inner_radius + bolt_radius * 0.33, alpha);
        let mut a = alpha;
        for j in 0..5 {
            if j == 0 {
                path.move_to(point_in_circle(c, bolt_radius, a));
            } else {
                path.cubic_to(
                    point_in_circle(c, bolt_radius * 1.14, a + PI / 3.0),
                    point_in_circle(c, bolt_radius * 1.14, a + PI / 6.0),
                    point_in_circle(c, bolt_radius, a),
                );
            }
            a -= PI / 2.0;
        }
        path.close();

        alpha += delta;
    }

    paint.set_style(PaintStyle::Fill);
    // Rust shade, from steel gray to rust color:
    paint.set_shader(gradient_shader::radial(
        (0.0, 0.04 * ridge_radius),
        ridge_radius,
        [Color::from(0xff_555555), Color::from(0xff_7b492d)].as_ref(),
        [0.8, 1.0].as_ref(),
        TileMode::Clamp,
        None,
        None,
    ));
    canvas.draw_path(&path, &paint);
    paint.set_shader(None); // Remove gradient.
    paint.set_style(PaintStyle::Stroke);
    paint.set_color(0xff_592e1f);
    canvas.draw_path(&path, &paint);

    canvas.restore();

    // Ridge around the chain ring, under the gear teeth:
    gradient(
        &mut paint,
        (0.0, -ridge_radius),
        (2.0 * ridge_radius, 2.0 * ridge_radius),
        (Color::from(0xff_592e1f), Color::from(0xff_885543)),
    );
    canvas.draw_circle(center, ridge_radius, &paint);

    canvas.restore();
}

#[allow(clippy::many_single_char_names)]
fn triangle(
    canvas: &mut skia_safe::canvas::Canvas,
    center: (i32, i32),
    radius: i32,
    degrees: f32,
    vertex: Option<i32>,
    color: Color,
    wankel: bool,
) {
    let c = (center.0 as f32, center.1 as f32);
    let r = radius as f32;
    let b = r * 0.9;
    let delta = 120.0 * DEGREES_IN_RADIANS;
    let side = r / ((PI - delta) / 2.0).cos() * 2.0;

    let mut alpha = degrees * DEGREES_IN_RADIANS;
    let mut path = Path::new();
    let mut paint = Paint::default();
    match vertex {
        Some(index) => {
            let a = (degrees + (120 * index) as f32) * DEGREES_IN_RADIANS;
            let center = point_in_circle(c, r, a);
            let radii = match index {
                0 | 2 => {
                    if wankel {
                        (0.36 * side, 0.404 * side)
                    } else {
                        (0.30 * side, 0.60 * side)
                    }
                }
                1 => {
                    if wankel {
                        (0.404 * side, 0.50 * side)
                    } else {
                        (0.420 * side, 0.50 * side)
                    }
                }
                i => panic!("Invalid vertex index {} for triangle.", i),
            };
            gradient(&mut paint, center, radii, (color, Color::from(0x00_0000ff)))
        }
        None => {
            paint.set_anti_alias(true);
            paint.set_stroke_width(
                PEN_SIZE.max(canvas.image_info().dimensions().width as f32 / 360.0),
            );
            paint.set_style(PaintStyle::Stroke);
            paint.set_stroke_join(PaintJoin::Bevel);
            // Highlight reflection on the top triangle edge:
            paint.set_shader(gradient_shader::radial(
                (c.0, c.1 - 0.5 * r),
                0.5 * r,
                [Color::from(0xff_ffffff), color].as_ref(),
                None,
                TileMode::Clamp,
                None,
                None,
            ));
        }
    };
    for i in 0..4 {
        let v = point_in_circle(c, r, alpha);
        if i == 0 {
            path.move_to(v);
        } else if wankel {
            path.cubic_to(
                point_in_circle(c, b, alpha - 2.0 * delta / 3.0),
                point_in_circle(c, b, alpha - delta / 3.0),
                v,
            );
        } else {
            path.line_to(v);
        }
        alpha += delta;
    }
    path.close();
    canvas.draw_path(&path, &paint);
}

fn gradient(paint: &mut Paint, center: (f32, f32), radii: (f32, f32), colors: (Color, Color)) {
    let mut matrix = Matrix::scale((1.0, radii.1 / radii.0));
    matrix.post_translate((center.0, center.1));
    paint.set_shader(gradient_shader::radial(
        (0.0, 0.0),
        radii.0,
        [colors.0, colors.1].as_ref(),
        None,
        TileMode::Clamp,
        None,
        &matrix,
    ));
}
*/