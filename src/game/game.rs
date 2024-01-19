use super::scene::Scene;

enum GameState {
    Paused,
    Running,
}

struct Game {
    state: GameState,
    scene: Scene,
    scores: (u32, u32),
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: GameState::Paused,
            scene: Scene::default(),
            scores: (0, 0),
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

    pub fn randomize(
        &mut self,
        left_paddle_count: usize,
        right_paddle_count: usize,
        ball_count: usize,
    ) {
        self.scene.clear_paddles();
        self.scene.clear_balls();
    }
}
