use std::error::Error;
use std::{io, thread};
use core::time::Duration;
use std::sync::mpsc;
use std::time::Instant;
use crossterm::event;
use crossterm::event::{ Event, KeyCode };
use crossterm::{ExecutableCommand, terminal};
use crossterm::cursor::{Hide, Show};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use rusty_audio::Audio;
use invaders::{frame, Player, render};
use invaders::frame::{Drawable, new_frame};

fn main() -> Result<(), Box<dyn Error>>{
	let mut audio = Audio::new();
	audio.add("explode", "audio/explode.wav");
	audio.add("lose", "audio/lose.wav");
	audio.add("move", "audio/move.wav");
	audio.add("pew", "audio/pew.wav");
	audio.add("startup", "audio/startup.wav");
	audio.add("win", "audio/win.wav");
	audio.play("startup");

	// Terminal
	let mut stdout = io::stdout();
	terminal::enable_raw_mode()?;
	stdout.execute(EnterAlternateScreen)?;
	stdout.execute(Hide);

	// Render loop thread
	let (render_tx, render_rx) = mpsc::channel();
	let render_handle = thread::spawn(move || {
		let mut last_frame = frame::new_frame();
		let mut stdout = io::stdout();
		render::render(&mut stdout, &last_frame, &last_frame, true);

		loop {
			let curr_frame = match render_rx.recv() {
				Ok(x) => x,
				Err(_) => break
			};
			render::render(&mut stdout, &last_frame, &curr_frame, false);
			last_frame = curr_frame;
		}
	});

	let mut player = Player::new();
	let mut instant = Instant::now();

	// Game loop thread
	'gameloop: loop {
		// Per-frame init
		let delta = instant.elapsed();
		instant = Instant::now();
		let mut curr_frame = new_frame();

		// Input
		while event::poll(Duration::default())? {
			if let Event::Key(e) = event::read()? {
				match e.code {
					KeyCode::Left => player.move_left(),
					KeyCode::Right => player.move_right(),
					KeyCode::Char(' ') => {
						if player.shoot() {
							audio.play("pew");
						}
					}
					KeyCode::Esc | KeyCode::Char('q') => {
						audio.play("lose");
						break 'gameloop;
					},
					_ => {}
				}
			}
		}

		// Updates
		player.update(delta);

		// Draw & render
		player.draw(&mut curr_frame);
		let _ = render_tx.send(curr_frame);
		thread::sleep(Duration::from_millis(10));  // 100 fps
	}

	// Clean up
	drop(render_tx);
	render_handle.join().unwrap();

	audio.wait();

	stdout.execute(Show);
	stdout.execute(LeaveAlternateScreen);
	terminal::disable_raw_mode();

	Ok(())
}
