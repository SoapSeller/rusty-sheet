use druid::widget::{Button, Flex, TextBox};

use druid::kurbo::BezPath;
use druid::piet::{FontFamily, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use druid::{
    Affine, AppLauncher, Color, FontDescriptor, LocalizedString, Point, Rect, TextLayout, PlatformError,
    WindowDesc, WidgetExt,
};


pub fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder);
    let data = "hi".to_string();
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
}

fn ui_builder() -> impl Widget<String> {
    // The label text will be computed dynamically based on the current locale and count
    // let text =
    //     LocalizedString::new("hello-counter").with_arg("count", |data: &String, _env| (*data).into());
    let textbox = TextBox::multiline()
        .with_placeholder("Hi?").fix_width(300.0);

    let grid = GridView {};

    Flex::row().with_flex_child(grid, 1.0).with_child(textbox)
        
}

const CELL_SIZE: (f64, f64) = (80.0, 20.0);

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

struct GridView;

impl Widget<String> for GridView {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut String, _env: &Env) {}

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &String,
        _env: &Env,
    ) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &String, _data: &String, _env: &Env) {}


    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &String,
        _env: &Env,
    ) -> Size {
        // BoxConstraints are passed by the parent widget.
        // This method can return any Size within those constraints:
        // bc.constrain(my_size)
        //
        // To check if a dimension is infinite or not (e.g. scrolling):
        // bc.is_width_bounded() / bc.is_height_bounded()
        //
        // bx.max() returns the maximum size of the widget. Be careful
        // using this, since always make sure the widget is bounded.
        // If bx.max() is used in a scrolling widget things will probably
        // not work correctly.
        if bc.is_width_bounded() && bc.is_height_bounded() {
            bc.max()
        } else {
            let size = Size::new(100.0, 100.0);
            bc.constrain(size)
        }
    }

    // The paint method gets called last, after an event flow.
    // It goes event -> update -> layout -> paint, and each method can influence the next.
    // Basically, anything that changes the appearance of a widget causes a paint.
    fn paint(&mut self, ctx: &mut PaintCtx, data: &String, env: &Env) {
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::WHITE);


        // Headers
        {
            let header_bg_color = Color::GRAY;
            let header_fg_color = Color::BLACK;

            let mut header_layout = TextLayout::<String>::from_text("0000");
            header_layout.set_font(FontDescriptor::new(FontFamily::MONOSPACE).with_size(12.0));
            header_layout.set_text_color(header_fg_color);
            header_layout.rebuild_if_needed(ctx.text(), env);

            let lines_col_width = header_layout.size().width * 1.2;
        


            let rect = Rect::from_origin_size((0.0, 0.0), (size.width, CELL_SIZE.1));
            ctx.fill(rect, &header_bg_color);

            let rect = Rect::from_origin_size((0.0, 0.0), (lines_col_width, size.height));
            ctx.fill(rect, &header_bg_color);


            // Draw rows
            for j in 1..((size.height as usize / CELL_SIZE.1 as usize) as usize) {
                let y = (j * CELL_SIZE.1 as usize) as f64;

                //let text = (j + state.view_offset.row as usize).to_string();
                let text = (j).to_string();
                header_layout.set_text(text);
                header_layout.rebuild_if_needed(ctx.text(), env);

                let bounds = header_layout.size();
                header_layout.draw(ctx, ((lines_col_width-bounds.width)/2.0, y + ((CELL_SIZE.1 - bounds.height) / 2.0)));
                
            }

            // Draw cols
            for i in 0..((size.width as usize / CELL_SIZE.0 as usize) as usize) {
                let x = (i * CELL_SIZE.0 as usize) as f64;
                //let text = col_to_letters(i+1 + state.view_offset.col as usize);
                let text = col_to_letters(i+1);
                header_layout.set_text(text);
                header_layout.rebuild_if_needed(ctx.text(), env);

                let bounds = header_layout.size();
                header_layout.draw(ctx, (lines_col_width + x + (CELL_SIZE.0 - bounds.width)/2.0, (CELL_SIZE.1 - bounds.height) / 2.0));
            }
        }
        
        // Main grid
        {
            let mut path = BezPath::new();
            

        }
    }
}
