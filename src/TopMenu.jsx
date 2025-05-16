import { createSignal } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/core";
import { A } from "@solidjs/router";
import "./TopMenu.css";

function App() {
  return (
    <div class="topmenu">
      <div class="navigation">
         <A href="/overview"    class="main-pages" activeClass="active-link" end><button>Overview</button></A>
         <A href="/library"     class="main-pages" activeClass="active-link" end><button>Library</button></A>
         <A href="/screenshots" class="main-pages" activeClass="active-link" end><button>screenshots</button></A>
         <A href="/settings"    class="main-pages" activeClass="active-link" end><button>Settings</button></A>
      </div>
      <div class="misc-links">
        wow
      </div>
    </div>
  );
}

export default App;
