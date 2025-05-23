/* @refresh reload */
import "./App.css";
import "./components/LibrarySearch.css";
import TopMenu from "./components/TopMenu.jsx";
import Search from './assets/search.svg?url';
import Minimize from './assets/minimize.svg?url';
import Maximize from './assets/maximize.svg?url';
import Plus from './assets/plus.svg?url';
import Grid2x2 from './assets/grid-2x2.svg?url';
import List from './assets/list.svg?url';
import Slider from './assets/sliders-horizontal.svg?url';
import LibraryItem from './components/LibraryItem.jsx';

import { createSignal, onMount } from "solid-js";
import { games, setGames, loadGamesFromFile, saveGamesToFile } from "./store/GamesStore.jsx";

function App() {

  // create a signal
  // this signal will be used to update the filter each time something is typed into the search field
  const [search, setSearch] = createSignal("");

  // on mount load the games and update the signal
  onMount(async () => {
    const loadedGames = await loadGamesFromFile("./games.json");
    setGames(loadedGames); 
  });

  // filter the games
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
          <LibraryItem title={game.title} state={game.state} status={game.status} time={game.time} imgPath={game.img}/>
        ))}
      </div>
    </main>
  );
}

export default App;
