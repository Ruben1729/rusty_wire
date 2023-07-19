use rusty_wire::engine::Engine;

fn main() {
    let mut engine: Engine<100, 100> = Engine::new(10);
    engine.start();
}
