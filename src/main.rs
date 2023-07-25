use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use std::sync::mpsc;
use std::thread;

// Custom events enumeration
enum CustomEvent {
    CloseRequested,

}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // Create a channel to send custom events from the dedicated thread to the main event loop
    let (sender, receiver) = mpsc::channel();

    // Spawn a dedicated thread for generating events
    thread::spawn(move || {
        // Mock event generator (replace this with your actual event source)
        // Here, we generate three CloseRequested events to close the window after some delay
        println!("hello");
        for _ in 0..3 {
            thread::sleep(std::time::Duration::from_secs(2));
            sender.send(CustomEvent::CloseRequested).unwrap();
        }
    });

    event_loop.run(move |event, _, control_flow| {
        println!("world");
        *control_flow = ControlFlow::Wait;

        // Process events received from the channel
        while let Ok(custom_event) = receiver.try_recv() {
            match custom_event {
                CustomEvent::CloseRequested => {
                    println!("Received CloseRequested event from channel.");
                    // Add your window close handling logic here
                    // For now, we'll just exit the application when the window is closed
                    *control_flow = ControlFlow::ExitWithCode(4);
                }
            }
        }
    });
}
