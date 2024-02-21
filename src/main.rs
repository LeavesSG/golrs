use crate::gol::game::GameOfLife;

mod countable;
mod gol;
mod lattice;
mod types;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test() {
    let game = GameOfLife::new(10, 10);
    for _ in 0..10 {
        game.on_frame();
    }
    println!("{:?}", game);
}
