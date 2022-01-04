mod renderer;
mod sheet;
mod sheet_state;

use std::time::Instant;

use sheet_state::*;

use crate::sheet::Cell;

const DEBOUNCE_MILLIS: u128 = 120;

fn debounce<F>(mut func: F)  -> impl FnMut(&mut SheetState) where F: FnMut(&mut SheetState) {
    let mut last = Box::new(Instant::now());
    move |state| {
        if last.elapsed().as_millis() > DEBOUNCE_MILLIS
        {
            func(state);
            last = Box::new(Instant::now());
        }
    }
}

fn main() {
    use gl::types::*;
    use glutin::{
        event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        GlProfile,
    };
    use skia_safe::{
        gpu::{gl::FramebufferInfo, BackendRenderTarget, SurfaceOrigin},
        Color, ColorType, Surface,
    };

    type WindowedContext = glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;

    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("Rusty Sheet");

    let cb = glutin::ContextBuilder::new()
         .with_depth_buffer(0)
         .with_stencil_buffer(8)
         .with_pixel_format(24, 8)
         .with_gl_profile(GlProfile::Core);

    let windowed_context = cb.build_windowed(wb, &el).unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    let pixel_format = windowed_context.get_pixel_format();

    println!(
        "Pixel format of the window's GL context: {:?}",
        pixel_format
    );

    gl::load_with(|s| windowed_context.get_proc_address(s));

    let mut gr_context = skia_safe::gpu::DirectContext::new_gl(None, None).unwrap();

    let fb_info = {
        let mut fboid: GLint = 0;
        unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

        FramebufferInfo {
            fboid: fboid.try_into().unwrap(),
            format: skia_safe::gpu::gl::Format::RGBA8.into(),
        }
    };

    windowed_context
        .window()
        .set_inner_size(glutin::dpi::Size::new(glutin::dpi::LogicalSize::new(
            1024.0, 1024.0,
        )));

    fn create_surface(
        windowed_context: &WindowedContext,
        fb_info: &FramebufferInfo,
        gr_context: &mut skia_safe::gpu::DirectContext,
    ) -> skia_safe::Surface {
        let pixel_format = windowed_context.get_pixel_format();
        let size = windowed_context.window().inner_size();
        let backend_render_target = BackendRenderTarget::new_gl(
            (
                size.width.try_into().unwrap(),
                size.height.try_into().unwrap(),
            ),
            pixel_format.multisampling.map(|s| s.try_into().unwrap()),
            pixel_format.stencil_bits.try_into().unwrap(),
            *fb_info,
        );
        Surface::from_backend_render_target(
            gr_context,
            &backend_render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        )
        .unwrap()
    }

    let surface = create_surface(&windowed_context, &fb_info, &mut gr_context);

    // Guarantee the drop order inside the FnMut closure. `WindowedContext` _must_ be dropped after
    // `DirectContext`.
    //
    // https://github.com/rust-skia/rust-skia/issues/476
    struct Env {
        surface: Surface,
        gr_context: skia_safe::gpu::DirectContext,
        windowed_context: WindowedContext,
    }

    let mut env = Env {
        surface,
        gr_context,
        windowed_context,
    };

    let mut state = SheetState::new();

    let handle_move = move |state: &mut SheetState| {
        state.text = state.sheet.get_text(&state.selected);
    };

    let mut handle_left = debounce(move |state| { state.selected.col = state.selected.col.saturating_sub(1); handle_move(state); });
    let mut handle_right = debounce(move |state| { state.selected.col += 1; handle_move(state); });
    let mut handle_up = debounce(move |state| { state.selected.row = state.selected.row.saturating_sub(1); handle_move(state); });
    let mut handle_down = debounce(move |state| { state.selected.row += 1; handle_move(state); });

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        #[allow(deprecated)]
        match event {
            Event::LoopDestroyed => {},
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    env.surface =
                        create_surface(&env.windowed_context, &fb_info, &mut env.gr_context);
                    env.windowed_context.resize(physical_size)
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::ReceivedCharacter(char) => {
                    match char {
                        '\u{8}' => { state.text.pop(); },
                        '\r' => { state.sheet.insert(state.selected.clone(), Cell{value: state.text.clone()}) },
                        _ => { state.text.push(char); },
                    }
                    //println!("Got char: {:?}", char);
                    env.windowed_context.window().request_redraw();
                },
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode,
                            modifiers,
                            ..
                        },
                    ..
                } => {
                    if modifiers.logo() {
                        if let Some(VirtualKeyCode::Q) = virtual_keycode {
                            *control_flow = ControlFlow::Exit;
                            println!("Quit nicely...");
                        }
                    }
                    
                    match virtual_keycode {
                        // TBD: Add debouncer per key
                        Some(VirtualKeyCode::Left) => { handle_left(&mut state); },
                        Some(VirtualKeyCode::Right) => { handle_right(&mut state); },
                        Some(VirtualKeyCode::Up) => { handle_up(&mut state); },
                        Some(VirtualKeyCode::Down) => { handle_down(&mut state); },
                        _ => (),
                    }
                    env.windowed_context.window().request_redraw();
                },
                WindowEvent::CursorMoved {..} => {
                    env.windowed_context.window().request_redraw();
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                {
                    let canvas = env.surface.canvas();
                    canvas.clear(Color::WHITE);
                    renderer::render(canvas, &mut state);
                }
                env.surface.canvas().flush();
                env.windowed_context.swap_buffers().unwrap();
            },
            _ => (),
        }
    });}
    