/* @refresh reload */
import { A } from "@solidjs/router";
import Download from './assets/download.svg';
import Palette from './assets/palette.svg';
import Clock from './assets/clock.svg';
import Cloud from './assets/cloud.svg';
import Reload from './assets/refresh-cw.svg';
import Close from './assets/x.svg';
import Square from './assets/square.svg';
import Minus from './assets/minus.svg';

Reload
import "./TopMenu.css";

function App() {
  return (
    <div class="topmenu">
      <div class="navigation">
         <A href="/overview"    class="main-pages overview-button" activeClass="active-link" end><button>Overview</button></A>
         <A href="/library"     class="main-pages" activeClass="active-link" end><button>Library</button></A>
         <A href="/screenshots" class="main-pages" activeClass="active-link" end><button>Screenshots</button></A>
         <A href="/settings"    class="main-pages" activeClass="active-link" end><button>Settings</button></A>
      </div>
      <div class="misc-grouping">
        <div class="misc-links">
          <A href="/" end><img src={Download}/></A>
          <A href="/" end><img src={Palette}/></A>
          <A href="/" end><img src={Clock}/></A>
          <A href="/" end><img src={Cloud}/></A>
          <A href="/" end><img src={Reload}/></A>
        </div>
        <div class="window-management-buttons">
          <A href="/" end><img src={Minus}/></A>
          <A href="/" end><img src={Square}/></A>
          <A href="/" end><img src={Close}/></A>
        </div>
      </div>
    </div>
  );
}

export default App;
