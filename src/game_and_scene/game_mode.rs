// a default game is a game with 2 paddles on each side and 2 balls
// a custom game can be customized, but it is not yet implemented
/// There are 2 game modes in total, one is default and the other is custom.
#[allow(dead_code)]
pub enum GameMode {
    /// The default game is defined as a game such that:
    /// - there are 2 paddles on each side, so 4 paddles in total;
    /// - there are 2 balls in total;
    /// - the paddles and balls have default sizes and default initial positions;
    /// - the balls are launched from the center of the screen.
    Default,
    /// Custom games allow you to modify any of the properties mentioned above.
    Custom,
}
