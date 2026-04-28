//! QuantumSpaceOS Wayland GUI — Main Entry Point
//!
//! A minimal, low-power Wayland client for space mission visualization:
//! - 3D orbital view
//! - Real-time telemetry streaming
//! - Quantum state visualizer
//!
//! Build:
//!   cargo build --release
//!
//! Run:
//!   ./target/release/quantumspace-gui

mod orbital_view;
mod telemetry;
mod quantum_visualizer;

use anyhow::Result;
use clap::Parser;
use winit::{
    application_loop::{ApplicationLoop, ControlFlow},
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopProxy},
    window::{Window, WindowBuilder, WindowId},
};
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::PhysicalSize;

/// Command-line arguments
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Telemetry source URL (e.g., http://localhost:8080)
    #[arg(short, long, default_value = "http://localhost:8080")]
    telemetry_url: String,

    /// Start in fullscreen
    #[arg(short, long)]
    fullscreen: bool,

    /// Window width
    #[arg(short, long, default_value_t = 1280)]
    width: u32,

    /// Window height
    #[arg(short, long, default_value_t = 720)]
    height: u32,
}

/// Application state
struct App {
    window: Window,
    pixels: Pixels,
    orbital_view: orbital_view::OrbitalView,
    telemetry_view: telemetry::TelemetryView,
    quantum_view: quantum_visualizer::QuantumVisualizer,
    telemetry_url: String,
}

impl App {
    async fn new(
        window: Window,
        pixels: Pixels,
        args: &Args,
    ) -> Result<Self> {
        let size = pixels.surface_size();

        Ok(Self {
            orbital_view: orbital_view::OrbitalView::new(size.width, size.height),
            telemetry_view: telemetry::TelemetryView::new(),
            quantum_view: quantum_visualizer::QuantumVisualizer::new(),
            telemetry_url: args.telemetry_url.clone(),
            window,
            pixels,
        })
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.pixels.resize_buffer(width, height);
        self.orbital_view.resize(width, height);
    }

    fn render(&mut self) -> Result<(), anyhow::Error> {
        let frame = self.pixels.frame_mut();

        // Clear background (space black)
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0x07, 0x0A, 0x14, 0xFF]); // #070A14
        }

        // Render orbital view (bottom layer)
        self.orbital_view.render(frame)?;

        // Render quantum visualization (top-left overlay)
        self.quantum_view.render(frame)?;

        // Render telemetry overlay (right panel)
        self.telemetry_view.render(frame)?;

        self.pixels.render()?;
        Ok(())
    }

    fn update(&mut self) -> Result<(), anyhow::Error> {
        // Update orbital view from flight-sim state (placeholder)
        self.orbital_view.update()?;

        // Fetch telemetry from API
        // TODO: implement async fetch
        self.telemetry_view.update()?;

        // Update quantum visualizer from core
        self.quantum_view.update()?;

        Ok(())
    }
}

// Custom event for telemetry updates
#[derive(Debug, Clone)]
enum AppEvent {
    TelemetryUpdate(telemetry::TelemetryData),
}

impl ApplicationLoop for App {
    fn event(&mut self, event: Event<AppEvent>, _elapsed: Instant, control_flow: &mut ControlFlow) {
        match event {
            Event::NewEvents(_) => {
                // Process events
            }
            Event::MainEventsCleared => {
                // Update simulation state
                if let Err(e) = self.update() {
                    eprintln!("Update error: {:?}", e);
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                // Request redraw
                self.window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Render frame
                if let Err(e) = self.render() {
                    eprintln!("Render error: {:?}", e);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(physical_size) => {
                    self.resize(physical_size.width, physical_size.height);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create event loop
    let event_loop = EventLoop::<AppEvent>::with_user_event();

    // Build window
    let window = WindowBuilder::new()
        .with_title("QuantumSpaceOS — Mission Control")
        .with_inner_size(PhysicalSize::new(args.width, args.height))
        .build(&event_loop)?;

    // Create pixel buffer
    let surface_texture = SurfaceTexture::new(args.width, args.height, &window);
    let pixels = Pixels::new(args.width, args.height, surface_texture)?;

    // Initialize application
    let mut app = App::new(window, pixels, &args).await?;

    // Enter event loop
    event_loop.run_app(&mut app);

    Ok(())
}
