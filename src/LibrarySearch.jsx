/* @refresh reload */
import "./LibrarySearch.css";
import Search from './assets/search.svg';
import Minimize from './assets/minimize.svg';
import Maximize from './assets/maximize.svg';
import Plus from './assets/plus.svg';
import Grid2x2 from './assets/grid-2x2.svg';
import List from './assets/list.svg';
import Slider from './assets/sliders-horizontal.svg';

function App() {
  return (
    <div class="lib-search-filtering">
      <div class="lib-search-container">
        <div class="search-container">
          <img src={Search} class="search-icon"/>
          <input type="text" placeholder="Search..." class="search-input" />
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
  );
}

export default App;
