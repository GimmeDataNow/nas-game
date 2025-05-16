import { createSignal } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/core";
import "./TopMenu.css";

function App() {
  return (
    <div class="topmenu">
      <div class="navigation">
         <a href="/overview"><button>Overview</button></a>
         <a href="/library"><button>Library</button></a>
         <a href="/screenshots"><button>screenshots</button></a>
         <a href="/settings"><button>Settings</button></a>
      </div>
      <div class="misc-links">
        wow
      </div>
    </div>
  );
}

export default App;
