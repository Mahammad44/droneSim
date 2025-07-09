import pandas as pd
import matplotlib.pyplot as plt

# Load CSV with no header
df = pd.read_csv("telemetry.csv", header=None)

# Assign all 13 columns
df.columns = [
    "drone_id", "timestamp", "latitude", "longitude",
    "altitude", "pitch", "roll", "yaw",
    "heading", "throttle", "speed", "mode", "armed"
]

# Convert timestamp column to numeric
df["timestamp"] = pd.to_numeric(df["timestamp"], errors="coerce")

# Drop rows where timestamp couldn't be converted
df = df.dropna(subset=["timestamp"])

# Convert to milliseconds and seconds
df["time_ms"] = df["timestamp"] - df["timestamp"].iloc[0]
df["time_s"] = df["time_ms"] / 1000.0

# Plotting
plt.figure(figsize=(12, 8))

plt.subplot(2, 2, 1)
plt.plot(df["time_s"], df["altitude"])
plt.title("Altitude over Time")
plt.xlabel("Time (s)")
plt.ylabel("Altitude (m)")

plt.subplot(2, 2, 2)
plt.plot(df["time_s"], df["pitch"])
plt.title("Pitch over Time")
plt.xlabel("Time (s)")
plt.ylabel("Pitch (°)")

plt.subplot(2, 2, 3)
plt.plot(df["time_s"], df["roll"])
plt.title("Roll over Time")
plt.xlabel("Time (s)")
plt.ylabel("Roll (°)")

plt.subplot(2, 2, 4)
plt.plot(df["time_s"], df["yaw"])
plt.title("Yaw over Time")
plt.xlabel("Time (s)")
plt.ylabel("Yaw (°)")

plt.tight_layout()
plt.show()

