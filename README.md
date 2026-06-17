#  Rust Algorith Sorting Visualizer 

A real-time sorting algorithm visualizer built with Rust and [egui](https://github.com/emilk/egui) via [eframe](https://github.com/emilk/egui/tree/master/crates/eframe). Watch classic (and chaotic) sorting algorithms come to life with animated bar charts.

---

## Preview

Bars represent randomly generated values. As the selected algorithm runs, bars are rearranged in real time until the array is fully sorted.

---

## Features

-  **Live bar chart visualization** of the array being sorted
-  **Start / Pause** control to step through sorting at any time
-  **Reset** to generate a new random array
-  **Adjustable speed** via a Steps/Frame slider
-  **Multiple algorithms** selectable from a dropdown

---

## Algorithms

| Algorithm | Description | Time Complexity |
|---|---|---|
| **Bubble Sort** | Repeatedly swaps adjacent elements that are out of order | O(n²) |
| **Insertion Sort** | Builds a sorted portion one element at a time | O(n²) |
| **Bogo Sort** | Randomly shuffles until sorted — for fun only! | O((n+1)!) avg |

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable, 1.70+)

### Run

```bash
git clone https://github.com/your-username/rust-sorting-visualizer
cd rust-sorting-visualizer
cargo run --release
```

> `--release` is recommended for smooth rendering performance.

### Dependencies

```toml
[dependencies]
eframe = "0.27"
rand = "0.8"
```

---

## Usage

1. **Select an algorithm** from the dropdown menu
2. Click **Start** to begin sorting
3. Click **Pause** to freeze at any point
4. Drag the **Steps/Frame** slider to control animation speed
5. Click **Reset** to shuffle a new random array and start over

>  **Bogo Sort** is included for amusement — it may run for a very long time on arrays larger than ~8 elements.

---

## Project Structure

```
src/
└── main.rs   # All app logic, rendering, and algorithm implementations
```

---

## License

MIT
