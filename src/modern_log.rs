use crate::constants::PROJECT_NAME;
use owo_colors::OwoColorize;
use std::fmt::Display;
use std::fmt::Write;
use std::io;
use std::io::Write as OtherWrite;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub fn status(head: impl Display, msg: impl Display) {
    write_status(head, msg);
    println!();
}

pub fn write_status(head: impl Display, msg: impl Display) {
    print!("{}{:>12} {}", project_header(), head, msg);
}

pub fn format_status(head: impl Display, msg: impl Display) -> String {
    format!("{}{:>12} {}", project_header(), head, msg)
}

pub fn recv_input() -> anyhow::Result<String> {
    let mut s = String::new();
    io::stdout().flush()?;
    io::stdin().read_line(&mut s)?;

    Ok(s.trim_end().to_owned())
}

pub fn project_header() -> String {
    format!("{} | ", PROJECT_NAME.bright_black().dimmed()).to_owned()
}

pub fn apply_project_header(msg: &str) -> String {
    (project_header() + msg).to_owned()
}

pub fn assume_terminal_size() -> u16 {
    terminal_size::terminal_size().unwrap().0.0
}

pub fn prefix_lines(s: &str, prefix: &str) -> String {
    s.lines()
        .map(|line| format!("{prefix}{line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn gradient_string(text: &str, phase: f32) -> String {
    const BASE: [(u8, u8, u8); 1] = [(125, 125, 125)];

    const LIGHT: [(u8, u8, u8); 1] = [(255, 255, 255)];

    let half_width: f32 = text.len() as f32 / 3.5;

    let graphemes: Vec<&str> = text.graphemes(true).collect();
    let total_width: usize = graphemes.iter().map(|g| UnicodeWidthStr::width(*g)).sum();

    if total_width == 0 {
        return String::new();
    }

    let mix = |a: (u8, u8, u8), b: (u8, u8, u8), t: f32| {
        let channel = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;

        (channel(a.0, b.0), channel(a.1, b.1), channel(a.2, b.2))
    };

    let sample = |palette: &[(u8, u8, u8)], t: f32| {
        assert!(!palette.is_empty(), "palette must not be empty");

        if palette.len() == 1 {
            return palette[0];
        }

        let x = t.clamp(0.0, 1.0) * (palette.len() - 1) as f32;
        let i = (x.floor() as usize).min(palette.len() - 2);

        mix(palette[i], palette[i + 1], x - i as f32)
    };

    let phase = phase.rem_euclid(1.0);
    let sweep_length = total_width as f32 + half_width * 2.0;
    let highlight_center = -half_width + phase * sweep_length;

    let mut output = String::with_capacity(text.len() + graphemes.len() * 24);
    let mut offset = 0usize;

    for grapheme in &graphemes {
        let width = UnicodeWidthStr::width(*grapheme);
        let center = offset as f32 + width as f32 / 2.0;

        let gradient_t = if total_width <= 1 {
            0.5
        } else {
            ((center - 0.5) / (total_width as f32 - 1.0)).clamp(0.0, 1.0)
        };

        let base = sample(&BASE, gradient_t);
        let light = sample(&LIGHT, gradient_t);

        let distance = (center - highlight_center).abs();
        let x = ((half_width - distance) / half_width).clamp(0.0, 1.0);
        let strength = x * x * (3.0 - 2.0 * x);

        let (r, g, b) = mix(base, light, strength);

        write!(&mut output, "\x1b[38;2;{r};{g};{b}m{grapheme}")
            .expect("writing to String cannot fail");

        offset += width;
    }

    output.push_str("\x1b[0m");
    output
}

use indicatif::ProgressBar;
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

pub trait GradientTick {
    fn enable_gradient_tick(
        &self,
        text: impl Into<String>,
        interval: Duration,
    ) -> GradientTickHandle;
}

pub struct GradientTickHandle {
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl GradientTickHandle {
    pub fn stop(mut self) {
        self.stop.store(true, Ordering::Relaxed);

        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

impl Drop for GradientTickHandle {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);

        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

impl GradientTick for ProgressBar {
    fn enable_gradient_tick(
        &self,
        text: impl Into<String>,
        interval: Duration,
    ) -> GradientTickHandle {
        let pb = self.clone();
        let text = text.into();
        let stop = Arc::new(AtomicBool::new(false));
        let stop_thread = Arc::clone(&stop);

        let thread = thread::spawn(move || {
            let mut phase = 0.0_f32;

            while !stop_thread.load(Ordering::Relaxed) && !pb.is_finished() {
                pb.set_message(gradient_string(&text, phase));
                pb.tick();
                phase = (phase + 0.015).rem_euclid(1.0);

                thread::sleep(interval);
            }
        });

        GradientTickHandle {
            stop,
            thread: Some(thread),
        }
    }
}
