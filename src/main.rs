use rusty_wire::engine::Engine;

fn main() {
    let mut engine: Engine<400, 400> = Engine::new(1280, 720);
    engine.start();
}
