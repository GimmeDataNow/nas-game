import { createSignal } from "solid-js";

const [games, setGames] = createSignal([]);
export { games, setGames };


export function saveGamesToFile() {
  const data = JSON.stringify(games(), null, 2);
  const blob = new Blob([data], { type: "application/json" });
  const url = URL.createObjectURL(blob);
  
  const a = document.createElement("a");
  a.href = url;
  a.download = "games.json";
  a.click();
  URL.revokeObjectURL(url);
}

export function loadGamesFromFile(file) {
  const reader = new FileReader();
  reader.onload = (e) => {
    try {
      const parsed = JSON.parse(e.target.result);
      if (Array.isArray(parsed)) {
        setGames(parsed);
      } else {
        console.error("Invalid file format");
      }
    } catch (err) {
      console.error("Failed to parse JSON:", err);
    }
  };
  reader.readAsText(file);
}
