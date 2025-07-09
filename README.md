## 🛰️ DroneSim – A Minimal Drone Telemetry Simulator in Rust

**DroneSim** is a Rust-based project that simulates telemetry data from a drone and transmits it over TCP. A separate receiver module listens on port `9000` and logs incoming data. The output is visualized using JupyterLab and Python to explore telemetry patterns such as position, speed, and heading over time.

---

### 📦 Features

* 📡 **Telemetry Generation**: Simulated data includes time, position, velocity, heading.
* 🔌 **TCP Communication**: Sends telemetry over TCP to a receiver.
* 📥 **Receiver Module**: Listens for data and logs it into a `.csv` file.
* 📊 **Visualization**: Python/JupyterLab scripts plot the telemetry data.
* ⚙️ **Rust-Based**: Safe, high-performance system implementation.

---

### 🛠️ Project Structure

```
.
├── src/
│   ├── main.rs                # Simulates drone and sends telemetry
│   ├── lib.rs                 # Shared module (optional)
│   └── bin/
│       ├── receiver.rs        # Receives telemetry over TCP and logs it
│       ├── visualize_telemetry.py  # Python script for data visualization
│       └── telemetry.csv      # Sample telemetry log
├── Cargo.toml
├── README.md
```

---

### 🚀 Getting Started

#### 1. Build the project

```bash
cargo build
```

#### 2. Start the receiver (in one terminal)

```bash
cargo run --bin receiver
```

#### 3. Start the drone simulator (in another terminal)

```bash
cargo run
```

Telemetry data will be written to `telemetry.csv`.

---

### 📈 Visualizing the Telemetry

Open the Jupyter Notebook or use the provided Python script:

```bash
python3 src/bin/visualize_telemetry.ipynb
```
This will produce a .html file to view the simulated drone path.

Or inside JupyterLab:

```bash
jupyter lab
```


### 📸 Preview

`docs/telemetry_plot.png`)

---

### 📜 License

MIT License. Feel free to fork, extend, and modify!

---

### 👨‍💻 Author

Mahammad Bin-Carlton – [Portfolio](https://mahammadbincarlton.com)
GitHub: [@Mahammad44](https://github.com/Mahammad44)

---



