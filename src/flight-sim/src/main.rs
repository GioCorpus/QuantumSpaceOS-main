//! Flight Simulator — Main Entry Point
//!
//! Usage:
//!   flight-sim --mode orbital --altitude 400
//!   flight-sim --mission mars-insertion --delta-v 2.5
//!   flight-sim --interactive
//!   flight-sim --mode atmospheric-entry --velocity 7.8

use clap::Parser;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod orbital_mechanics;
mod thrust_vectoring;
mod atmospheric_entry;
mod attitude_control;

use orbital_mechanics::*;
use thrust_vectoring::*;
use atmospheric_entry::*;
use attitude_control::*;

/// Flight simulation modes
#[derive(Debug, Clone, clap::ValueEnum)]
enum SimulationMode {
    /// Low Earth Orbit simulation
    Orbital,
    /// Mars orbital insertion
    MarsInsertion,
    /// Atmospheric reentry
    AtmosphericEntry,
    /// Lunar landing approach
    Lunar,
}

/// Interactive mode type
#[derive(Debug, Clone, clap::ValueEnum)]
enum InteractiveMode {
    /// Full mission control interface
    MissionControl,
    /// Simple cockpit HUD
    Cockpit,
    /// Orbital map view
    OrbitalMap,
}

/// Command-line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Simulation mode
    #[arg(short, long, value_enum)]
    mode: Option<SimulationMode>,

    /// Mission profile name
    #[arg(short, long)]
    mission: Option<String>,

    /// Orbit altitude in km (for orbital mode)
    #[arg(short, long, default_value_t = 400.0)]
    altitude: f64,

    /// Delta-V budget in km/s
    #[arg(short, long, default_value_t = 0.0)]
    delta_v: f64,

    /// Entry velocity in km/s
    #[arg(long, default_value_t = 7.8)]
    velocity: f64,

    /// Enable interactive mode
    #[arg(short, long)]
    interactive: bool,

    /// Select interactive mode variant
    #[arg(long, value_enum, default_value_t = InteractiveMode::MissionControl)]
    interactive_mode: InteractiveMode,

    /// Log level
    #[arg(short, long, default_value = "info")]
    log_level: String,
}

/// Main simulation state
struct Simulation {
    orbit: OrbitalState,
    thrust: ThrustController,
    attitude: AttitudeControl,
    running: bool,
}

impl Simulation {
    fn new() -> Self {
        Self {
            orbit: OrbitalState::new(),
            thrust: ThrustController::new(),
            attitude: AttitudeControl::new(),
            running: true,
        }
    }

    fn step(&mut self, dt: f64) {
        // Update orbital mechanics
        self.orbit.propagate(dt);

        // Apply thrust if active
        self.thrust.apply(&mut self.orbit, dt);

        // Stabilize attitude
        self.attitude.stabilize(&mut self.orbit, dt);
    }

    fn stop(&mut self) {
        self.running = false;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize logging
    let level = args.log_level.parse::<Level>().unwrap_or(Level::INFO);
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("QuantumSpaceOS Flight Simulator v0.1.0");
    info!("Starting simulation...");

    if args.interactive {
        run_interactive(args.interactive_mode)?;
    } else if let Some(mode) = args.mode {
        run_headless(mode, &args)?;
    } else if let Some(mission) = &args.mission {
        run_mission(mission, &args)?;
    } else {
        // Default: orbital simulation
        run_headless(SimulationMode::Orbital, &args)?;
    }

    Ok(())
}

/// Run headless (non-interactive) simulation
fn run_headless(mode: SimulationMode, args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    info!("Running {:?} mode", mode);

    let mut sim = Simulation::new();

    match mode {
        SimulationMode::Orbital => {
            sim.orbit.altitude = args.altitude * 1000.0; // km → m
            info!("Orbit altitude: {} km", args.altitude);
        }
        SimulationMode::MarsInsertion => {
            sim.orbit = OrbitalState::mars_insertion();
            info!("Mars orbital insertion sequence");
        }
        SimulationMode::AtmosphericEntry => {
            sim.orbit.velocity = args.velocity * 1000.0; // km/s → m/s
            info!("Entry velocity: {} km/s", args.velocity);
        }
        SimulationMode::Lunar => {
            sim.orbit = OrbitalState::lunar_transfer();
            info!("Lunar transfer orbit");
        }
    }

    // Run for fixed duration
    let sim_time = 3600.0; // 1 hour simulation
    let dt = 1.0; // 1 second steps
    let steps = (sim_time / dt) as usize;

    info!("Simulating {} steps...", steps);
    for i in 0..steps {
        sim.step(dt);

        if i % 600 == 0 {
            info!("t+{}s: alt={:.1}km, vel={:.1}m/s",
                i * (dt as usize),
                sim.orbit.altitude / 1000.0,
                sim.orbit.velocity);
        }
    }

    info!("Simulation complete");
    Ok(())
}

/// Run mission profile
fn run_mission(mission: &str, args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    info!("Running mission: {}", mission);

    match mission {
        "mars-insertion" => run_headless(SimulationMode::MarsInsertion, args)?,
        "leo" => run_headless(SimulationMode::Orbital, args)?,
        "reentry" => run_headless(SimulationMode::AtmosphericEntry, args)?,
        _ => return Err(format!("Unknown mission: {}", mission).into()),
    }

    Ok(())
}

/// Run interactive simulation with TUI
fn run_interactive(mode: InteractiveMode) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting interactive mode: {:?}", mode);

    // Placeholder: would launch TUI or GUI
    println!("Interactive mode not yet implemented");
    println!("Expected: full-screen TUI with orbital visualization, telemetry, and controls");

    Ok(())
}
