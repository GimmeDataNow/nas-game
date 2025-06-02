/* @refresh reload */
import { render } from "solid-js/web";
import { Router, Route, Navigate } from "@solidjs/router";
import { appLocalDataDir, join } from '@tauri-apps/api/path';
import { mkdir, BaseDirectory } from '@tauri-apps/plugin-fs';

import Overview from "./Overview.jsx";
import Library from "./Library.jsx";
import Screenshots from "./Screenshots.jsx";
import Settings from "./Settings.jsx";


async function createImagesFolder() {
  await mkdir('client', { baseDir: BaseDirectory.AppLocalData });
  await mkdir('client/images', { baseDir: BaseDirectory.AppLocalData });
}

createImagesFolder()


render(
  () => (
    <Router>
      <Route path="/" component={() => <Navigate href="/overview" />} />
      <Route path="/overview" component={Overview}/>
      <Route path="/library" component={Library}/>
      <Route path="/screenshots" component={Screenshots}/>
      <Route path="/settings" component={Settings}/>
    </Router>
  ),
  document.getElementById("root")
);
