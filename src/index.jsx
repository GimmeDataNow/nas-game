/* @refresh reload */
import { render } from "solid-js/web";
import { Router, Route } from "@solidjs/router";
import App from "./App";
import Overview from "./Overview.jsx";
import Library from "./Library.jsx";
import Screenshots from "./Screenshots.jsx";
import Settings from "./Settings.jsx";


render(
  () => (
    <Router>
      <Route path="/" component={Overview}/>
      <Route path="/overview" component={Overview}/>
      <Route path="/library" component={Library}/>
      <Route path="/screenshots" component={Screenshots}/>
      <Route path="/settings" component={Settings}/>

    </Router>
  ),
  document.getElementById("root")
);
