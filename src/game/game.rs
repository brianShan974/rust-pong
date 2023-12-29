enum GameState {
    Pausing,
    Running,
}

struct Game {
    state: GameState,
    scene: Scene,
    scores: (u32, u32),
}

impl Game {
    pub fn new() -> Self {

    }
}
