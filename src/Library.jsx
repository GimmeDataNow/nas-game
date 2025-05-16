/* @refresh reload */
import { createSignal } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import TopMenu from "./TopMenu.jsx";

function App() {

  return (
    <main class="container">
      <TopMenu/>
      This is the library      
    </main>
  );
}

export default App;
