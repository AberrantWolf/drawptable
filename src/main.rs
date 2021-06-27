mod rendering;
mod texture;

use futures::executor::block_on;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use rendering::RenderState;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut render_state = block_on(RenderState::new(&window));

    event_loop.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            render_state.update();
            match render_state.render() {
                Ok(_) => {}
                Err(wgpu::SwapChainError::Lost) => render_state.regen_swap_chain(),
                Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if !render_state.input(event) {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => match input {
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        _ => {}
                    },
                    WindowEvent::Resized(physical_size) => {
                        render_state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        render_state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    });
}
