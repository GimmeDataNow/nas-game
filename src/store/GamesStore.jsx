import { createSignal } from "solid-js";
// stored in ~/.local/share/com.nas-game.app
import { readFile, create, writeTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';
import { appLocalDataDir } from '@tauri-apps/api/path';

const [games, setGames] = createSignal([]);
export { games, setGames };


export async function saveGamesToFile() {
  const data = JSON.stringify(games(), null, 2);
}

// this still load every time the page is visited
export async function loadGamesFromFile(file) {
  try {
    const contents = await readFile(file, { baseDir: BaseDirectory.AppLocalData });
    const decoder = new TextDecoder("utf-8");
    const text = decoder.decode(contents);
    const data = JSON.parse(text);
    return data;
  } catch (err) {
    console.error("Error loading games:", err);
    return [];
  }
}
