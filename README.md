# RustyWire

RustyWire is a wireworld simulation written in Rust. It provides an interactive environment where you can simulate and explore the behavior of electronic circuits using the WireWorld cellular automaton.

Keep in mind the code could use some improvement. I built this project as a learning experience to get familiar with Rust and its features.

### WireWorld

WireWorld is a cellular automaton that models the behavior of electronic circuits. It consists of cells arranged on a grid, with each cell in one of four possible states: empty, electron head, electron tail, or conductor.

In the WireWorld simulation, electrons flow through conductors, interact with other components, and exhibit interesting patterns and behaviors. It serves as a simple yet powerful tool for visualizing and understanding the behavior of digital logic circuits.

### Controls

- **Left Click**: Place a cell.
- **Right Click**: Erase.
- **Scroll**: Change brush.

### Getting Started

Follow these steps to run RustyWire:

Clone the RustyWire repository:

```shell
git clone https://github.com/Ruben1729/rusty_wire
```
Navigate to the project directory:

```shell
cd RustyWire
```
Build and run the project using Cargo:

```shell
cargo run
```
The simulation will start, and you can interact with it using the provided keyboard controls.

### Future Plans
I'm happy with the current state of the project, although I would like to eventually add the brushes for other logic .

### References
- [WireWorld - Wikipedia](https://en.wikipedia.org/wiki/Wireworld)
- [WireWorld - Bitstorm](https://www.quinapalus.com/wi-index.html)

