use rusty_wire::engine::Engine;

fn main() {
    let mut engine: Engine<50, 50> = Engine::new(10);
    engine.start();
}
