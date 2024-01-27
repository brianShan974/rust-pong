use rand::rngs::ThreadRng;
use rand::thread_rng;

// use crate::game::paddle;

use super::ball::Ball;
use super::operation::Operation;
use super::paddle::{Paddle, Sides};
use super::scene::Scene;

enum GameState {
    Paused,
    Running,
}

pub struct Game {
    state: GameState,
    scene: Scene,
    scores: (u32, u32),
    rng: ThreadRng,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: GameState::Paused,
            scene: Scene::default(),
            scores: (0, 0),
            rng: thread_rng(),
        }
    }

    pub fn set_state(&mut self, state: GameState) {
        self.state = state;
    }

    pub fn set_scene(&mut self, scene: Scene) {
        self.scene = scene;
    }

    pub fn set_scores(&mut self, scores: (u32, u32)) {
        self.scores = scores;
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.scene.has_no_balls() {
            return Err(String::from(
                "Error: you cannot start a game without a ball.",
            ));
        } else if self.scene.has_no_left_paddles() {
            return Err(String::from(
                "Error: you cannot start a game without a left paddle.",
            ));
        } else if self.scene.has_no_right_paddles() {
            return Err(String::from(
                "Error: you cannot start a game without a right paddle.",
            ));
        }

        self.state = GameState::Running;
        Ok(())
    }

    pub fn update(&mut self, ops: Vec<Operation>) -> Option<Sides> {
        if let GameState::Running = self.state {
            let winner = self.scene.update_scene(ops);
            if let Some(winner) = winner {
                self.state = GameState::Paused;
                if let Sides::Left = winner {
                    self.scores.0 += 1;
                } else {
                    self.scores.1 += 1;
                }
            }
            winner
        } else {
            None
        }
    }

    pub fn get_left_paddle_count(&self) -> usize {
        self.get_left_paddles().len()
    }

    pub fn get_right_paddle_count(&self) -> usize {
        self.get_right_paddles().len()
    }

    pub fn get_left_paddles(&self) -> &Vec<Paddle> {
        self.scene.get_left_paddles()
    }

    pub fn get_right_paddles(&self) -> &Vec<Paddle> {
        self.scene.get_right_paddles()
    }

    pub fn get_balls(&self) -> &Vec<Ball> {
        self.scene.get_balls()
    }

    pub fn start_default_game_with_2_balls(&mut self) {
        self.scene = Scene::construct_default_scene_with_2_balls(&mut self.rng);
        self.start().unwrap();
    }

    // pub fn randomize(
    //     &mut self,
    //     left_paddle_count: usize,
    //     right_paddle_count: usize,
    //     ball_count: usize,
    // ) {
    //     self.scene.clear_paddles();
    //     self.scene.clear_balls();
    //
    //     todo!("Implement randomize method for game.");
    // }
}
