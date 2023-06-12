# RustyWire

RustyWire is a wireworld simulation written in Rust. It provides an interactive environment where you can simulate and explore the behavior of electronic circuits using the WireWorld cellular automaton.

Keep in mind the code could use some improvement. I built this project as a learning experience to get familiar with Rust and its features.

### WireWorld

WireWorld is a cellular automaton that models the behavior of electronic circuits. It consists of cells arranged on a grid, with each cell in one of four possible states: empty, electron head, electron tail, or conductor.

In the WireWorld simulation, electrons flow through conductors, interact with other components, and exhibit interesting patterns and behaviors. It serves as a simple yet powerful tool for visualizing and understanding the behavior of digital logic circuits.

### Controls

- **Left Click**: Place a cell.
- **Right Click**: Erase.
- **Spacebar**: Start or pause the simulation.
- **Q**: Select Conductor as the cell.
- **W**: Select Electron Head as the cell.
- **E**: Select Electron Tail as the cell.
- **Esc**: Quit the simulation

### Getting Started

Follow these steps to run RustyWire:

Clone the RustyWire repository:

```shell
git clone https://github.com/your-username/RustyWire.git
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
I'm happy with the current state of the project. I have started implementing a search bar and clusters of cells but will not be finishing the feature anytime soon.

### References
[WireWorld - Wikipedia](https://en.wikipedia.org/wiki/Wireworld)
[WireWorld - Bitstorm](https://www.quinapalus.com/wi-index.html)

