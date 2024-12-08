import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [version, setVersion] = createSignal("Loading...");

  // Fetch GStreamer version from the backend
  const fetchVersion = async () => {
    try {
      const response = await invoke<string>("get_gstreamer_version");
      setVersion(response);
    } catch (error) {
      setVersion("Failed to fetch version: " + error);
    }
  };

  // Fetch version when the app loads
  fetchVersion();

  return (
    <div style={{ padding: "20px", "font-family": "Arial, sans-serif" }}>
      <h1>GStreamer + Tauri + Solid.js</h1>
      <p>{version()}</p>
    </div>
  );
}

export default App;
