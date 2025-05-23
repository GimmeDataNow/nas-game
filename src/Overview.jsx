/* @refresh reload */
import { createSignal } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import TopMenu from "./components/TopMenu.jsx";

function App() {

  return (
    <main class="container">
      <TopMenu/>
      This is the overview and the default page      
    </main>
  );
}

export default App;
