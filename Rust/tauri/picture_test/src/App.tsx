import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
import { join, appDataDir } from '@tauri-apps/api/path';
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [someLogo, setSomeLogo] = useState<string | null>(null);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  let chosenLogo = someLogo
    ? someLogo
    : "/tauri.svg";

  async function changeLogo() {
    console.log("appDataDir: ", await appDataDir())
    // const finalFile = await join(await appDataDir(), 'pictureTest', 'favicon.png')
    const finalFile = await join(await appDataDir(), 'pictureTest', 'ferris.png')
    console.log("finalFile: ", finalFile)
    return convertFileSrc(finalFile);
  }

  changeLogo()
    .then(value => {
      if (!someLogo) {
        setSomeLogo(value);
      }
    });

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src={chosenLogo} className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>

      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
