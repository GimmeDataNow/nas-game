/* @refresh reload */
import "./App.css";
import LibrarySearch from "./LibrarySearch.jsx";
import TopMenu from "./TopMenu.jsx";

function App() {

  return (
    <main class="container">
      <TopMenu/>
      <LibrarySearch/>
      This is the library      
    </main>
  );
}

export default App;
