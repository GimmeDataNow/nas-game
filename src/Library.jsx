/* @refresh reload */
import "./App.css";
import "./components/LibrarySearch.css";
import TopMenu from "./components/TopMenu.jsx";
import Search from './assets/search.svg';
import Minimize from './assets/minimize.svg';
import Maximize from './assets/maximize.svg';
import Plus from './assets/plus.svg';
import Grid2x2 from './assets/grid-2x2.svg';
import List from './assets/list.svg';
import Slider from './assets/sliders-horizontal.svg';
import LibraryItem from './components/LibraryItem.jsx';

import { createSignal } from "solid-js";

function App() {

  const [games] = createSignal([
    { title: "Hollow Knight", state: "Installed", status: "Completed", time: 10000 },
    { title: "Celeste", state: "Not installed", status: "Not completed", time: 1500 },
    { title: "Stardew Valley", state: "Installed", status: "In progress", time: 300 },
    { title: "Outer Wilds", state: "Not installed", status: "Completed", time: 2000 },
    // Add more games as needed need to load this somehow
  ]);

  // create a signal
  // this signal will be used to update the filter each time something is typed into the search field
  const [search, setSearch] = createSignal("");

  const filteredGames = () => {
    const query = search().toLowerCase().trim();
    return games().filter(game => game.title.toLowerCase().includes(query));
  };

  return (
    <main class="container">
      <TopMenu/>
      <div class="lib-search-filtering">
        <div class="lib-search-container">
          <div class="search-container">
            <img src={Search} class="search-icon"/>
            <input type="text" placeholder="Search..." class="search-input" value={search()} onInput={(e) => setSearch(e.currentTarget.value)} />
          </div>
          <button><img src={Plus} class="icon"/></button>
        </div>
        <div class="filter-grouping">
          <button>Clear all</button>
          <button><img src={Slider} class="icon"/>Filters (x)</button>
          <button><img src={Grid2x2} class="icon"/></button>
          <button><img src={List} class="icon"/></button>
          |
          <div class="library-slider">
            <img src={Minimize} class="icon"/>
            <input type="range" min="1" max="100" value="50" class="slider" id="myRange"/>
            <img src={Maximize} class="icon"/>
          </div>
        </div>
      </div>            
      <div class="library-list">
        {filteredGames().map(game => (
          <LibraryItem title={game.title} state={game.state} status={game.status} time={game.time}/>
        ))}
      </div>
    </main>
  );
}

export default App;
