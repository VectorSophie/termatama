use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use termatama::state::Snapshot;
use termatama::sys::Button;
use termatama::tui::TuiRenderer;

#[derive(Clone)]
struct Keybind {
    left: char,
    middle: char,
    right: char,
}

struct Config {
    rom_path: PathBuf,
    keybind: Keybind,
    speed: f64,
    headless: bool,
}

fn parse_args() -> Config {
    let mut rom_path = PathBuf::from("roms\\tama.b");
    let mut keybind = Keybind {
        left: 'z',
        middle: 'x',
        right: 'c',
    };
    let mut speed = 1.0_f64;
    let mut headless = false;

    for arg in std::env::args().skip(1) {
        if let Some(rest) = arg.strip_prefix("--keybind=") {
            for part in rest.split(|c| c == ',' || c == ' ') {
                if let Some((k, v)) = part.split_once('=') {
                    let k_upper = k.trim().to_ascii_uppercase();
                    if let Some(ch) = v.trim().chars().next() {
                        let ch_lower = ch.to_ascii_lowercase();
                        match k_upper.as_str() {
                            "A" => keybind.left = ch_lower,
                            "B" => keybind.middle = ch_lower,
                            "C" => keybind.right = ch_lower,
                            _ => {}
                        }
                    }
                }
            }
            continue;
        }

        if let Some(rest) = arg.strip_prefix("--speed=") {
            if let Ok(v) = rest.parse::<f64>() {
                if v > 0.0 {
                    speed = v;
                }
            }
            continue;
        }

        if arg == "--headless" {
            headless = true;
            continue;
        }

        rom_path = PathBuf::from(arg);
    }

    Config {
        rom_path,
        keybind,
        speed,
        headless,
    }
}

fn main() -> std::io::Result<()> {
    let config = parse_args();

    let mut engine = match termatama::load_engine_from_file(&config.rom_path) {
        Ok(engine) => engine,
        Err(err) => {
            eprintln!(
                "failed to initialize engine from {}: {err}",
                config.rom_path.display()
            );
            std::process::exit(1);
        }
    };

    let save_path = PathBuf::from("termatama.state");
    if save_path.exists() {
        if let Ok(bytes) = std::fs::read(&save_path) {
            if let Ok(snapshot) = bincode::deserialize::<Snapshot>(&bytes) {
                engine.load_snapshot(&snapshot);
                println!("Loaded state from {}", save_path.display());
            }
        }
    }

    let mut renderer = if config.headless {
        terminal::enable_raw_mode()?;
        None
    } else {
        Some(TuiRenderer::new()?)
    };

    let mut accumulator = Duration::ZERO;
    let logic_step = Duration::from_secs_f64(1.0 / config.speed.max(0.01));
    let mut last_time = Instant::now();
    let logic_batch = ((1000.0 * config.speed).round() as usize).max(1);
    let frame_batch = ((100.0 * config.speed).round() as usize).max(1);

    'main: loop {
        let now = Instant::now();
        let delta = now - last_time;
        last_time = now;
        accumulator += delta;

        while accumulator >= logic_step {
            engine.tick_many(logic_batch);
            accumulator -= logic_step;
        }

        if event::poll(Duration::from_millis(1))? {
            if let Event::Key(key) = event::read()? {
                let pressed = match key.kind {
                    KeyEventKind::Press | KeyEventKind::Repeat => true,
                    KeyEventKind::Release => false,
                };

                if key.modifiers.contains(event::KeyModifiers::CONTROL) {
                    if let KeyCode::Char('c') | KeyCode::Char('C') = key.code {
                        break 'main;
                    }
                }

                match key.code {
                    KeyCode::Char(ch) => {
                        let ch_lower = ch.to_ascii_lowercase();
                        if ch_lower == config.keybind.left {
                            engine.set_button(Button::Left, pressed);
                        } else if ch_lower == config.keybind.middle {
                            engine.set_button(Button::Middle, pressed);
                        } else if ch_lower == config.keybind.right {
                            engine.set_button(Button::Right, pressed);
                        }
                    }
                    KeyCode::Esc => break 'main,
                    _ => {}
                }
            }
        }

        engine.tick_many(frame_batch);

        if let Some(r) = renderer.as_mut() {
            let lcd = engine.get_lcd();
            r.render(&lcd)?;
        }

        std::thread::sleep(Duration::from_millis(10));
    }

    let snapshot = engine.save_snapshot();
    if let Ok(bytes) = bincode::serialize(&snapshot) {
        if let Err(err) = std::fs::write(&save_path, bytes) {
            eprintln!("failed to write state to {}: {err}", save_path.display());
        } else {
            println!("Saved state to {}", save_path.display());
        }
    }

    if renderer.is_none() {
        let _ = terminal::disable_raw_mode();
    }

    Ok(())
}
