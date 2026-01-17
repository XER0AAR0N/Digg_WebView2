// Digglit created by Aaron Doe (XER0AAR0N)
//
// This is an unofficial client and is not affiliated with
// the official Digg development team in ANY way

#![windows_subsystem = "windows"]

use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Icon, WindowBuilder},
};

use wry::WebViewBuilder;

fn load_icon() -> Icon {
    let buffer = include_bytes!("digglit.ico");
    let image = image::load_from_memory(buffer)
        .expect("Failed to load icon")
        .to_rgba8();
    let (width, height) = image.dimensions();

    Icon::from_rgba(image.into_raw(), width, height)
        .expect("Failed to create window icon")
}

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Digglit")
        .with_inner_size(LogicalSize::new(1200.0, 1024.0))
        .with_window_icon(Some(load_icon()))
        .build(&event_loop)
        .expect("Failed to create window");

// Client spoofer is needed in order for Digg site not to throw an error whilst loading

    let _webview = WebViewBuilder::new(&window)
        .with_user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
             AppleWebKit/537.36 (KHTML, like Gecko) \
             Chrome/120.0.0.0 Safari/537.36",
        )
        .with_url("https://digg.com")
        .build()
        .expect("Failed to create WebView");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = event {
            *control_flow = ControlFlow::Exit;
        }
    });
}
