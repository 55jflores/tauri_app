import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [image_vec, setImgvec] = useState("none");

  async function apod() {
    // Calling rust function from react
    setImgvec(await invoke("grab_pic"));
  }
  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <div className="row">
        <div>
          <button type="button" onClick={() => apod()}>
            NASA pic of the day
          </button>
        </div>

      </div>

      {image_vec !== "none" &&
        <div>
          <p>{image_vec[0]}</p>
          <img 
          src={image_vec[1]}
          alt="new"
          width={512}
          height={512}
          />
          <p>{image_vec[2]}</p>
        </div>
      }

    </div>
  );
}

export default App;
