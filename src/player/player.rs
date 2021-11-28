use std::time::Duration;
use crate::{Frame, Invaders};
use crate::render::{Drawable, NUM_COLS, NUM_ROWS};
use crate::player::shot::Shot;

const MAX_SHOTS: usize = 2;
const PLAYER_SPEED: usize = 1;

pub struct Player {
	x: usize,
	y: usize,
	shots: Vec<Shot>
}

impl Player {
	pub fn new() -> Self {
		Self {
			x: NUM_COLS / 2,
			y: NUM_ROWS - 1,
			shots: Vec::new()
		}
	}

	pub fn move_left(&mut self) {
		if self.x > 0 {
			self.x -= PLAYER_SPEED;
		}
	}

	pub fn move_right(&mut self) {
		if self.x < NUM_COLS - 1 {
			self.x += PLAYER_SPEED;
		}
	}

	pub fn shoot(&mut self) -> bool {
		if self.shots.len() < MAX_SHOTS {
			self.shots.push(Shot::new(self.x, self.y - 1));
			return true;
		}
		false
	}

	pub fn update(&mut self, delta: Duration) {
		for shot in self.shots.iter_mut() {
			shot.update(delta);
		}
		self.shots.retain(|shot| !shot.dead());
	}

	pub fn detect_hits(&mut self, invaders: &mut Invaders) -> bool {
		let mut hit = false;

		for shot in self.shots.iter_mut() {
			if !shot.exploding {
				if invaders.kill_invader_at(shot.x, shot.y) {
					hit = true;
					shot.explode();
				}
			}
		}

		hit
	}
}

impl Drawable for Player {
	fn draw(&self, frame: &mut Frame) {
		frame[self.x][self.y] = "A";

		for shot in self.shots.iter() {
			shot.draw(frame);
		}
	}
}