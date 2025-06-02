/* @refresh reload */
import { createSignal, onMount } from "solid-js";
import fallbackImg from './../assets/Hollow_Knight_cover_art.webp';
import ExternalLink  from "./../assets/external-link.svg"; 
import Play from "./../assets/play.svg"; 

import { appLocalDataDir, join } from '@tauri-apps/api/path';
import { readDir, readFile , BaseDirectory } from '@tauri-apps/plugin-fs';
import { exists } from "@tauri-apps/plugin-fs";



function getMimeType(filename) {
  const ext = filename.split('.').pop()?.toLowerCase();
  switch (ext) {
    case 'jpg':
    case 'jpeg':
      return 'image/jpeg';
    case 'png':
      return 'image/png';
    case 'webp':
      return 'image/webp';
    default:
      // octet-stream tell the browser that it is an unidentified mime type
      // browser usually downlaod this file type. This might be an issue later
      return 'application/octet-stream';
  }
}


export async function getImagePath(filename) {
  try {
    // get full path cause tauri is being annoying about it
    const fullPath = await join(await appLocalDataDir(), 'client','images' , filename);
    const fileBuffer = await readFile(fullPath);

  
    const blob = new Blob([new Uint8Array(fileBuffer)], { type: getMimeType(filename) });
    return URL.createObjectURL(blob);
  } catch (err) {
    console.error(`Failed to load image from AppLocalData: ${filename}`, err);
    return null;
  }
}

function Card(props) {
  const title = () => props.title ?? "Default Title";
  const state = () => props.state ?? "Not installed";
  const time = () => props.time ?? 0;
  const status = () => props.status ?? "Not completed";

  const [bgImage, setBgImage] = createSignal(fallbackImg);

  onMount(async () => {
      const resolvedPath = await getImagePath(props.title + '.webp');
      if (resolvedPath) {
        const img = new Image();
        img.src = resolvedPath;
        img.onload = () => setBgImage(resolvedPath);
        img.onerror = () => setBgImage(fallbackImg);
      } else {
        setBgImage(fallbackImg);
      }
  });

  return (
    <div
      class="library-item-wrapper"
      style={{
        "background-image": `url(${bgImage()})`,
        "background-size": "cover",
        "background-position": "center",
      }}
    >  
      <div class="library-item-pop-over-wrapper">
        <div class="library-item-pop-over-game-title">{title}</div>
        <div class="library-item-pop-over-play-and-stats">
          <div class="library-item-pop-over-stat-description">State</div>
          <div class="library-item-pop-over-stat">{state}</div>
          <div class="library-item-pop-over-stat-description">Playtime</div>
          <div class="library-item-pop-over-stat">{time} hours played</div>
          <div class="library-item-pop-over-stat-description">Status</div>
          <div class="library-item-pop-over-stat">{status}</div>
          <div class="library-item-pop-over-buttons">
            <button class="library-item-pop-over-external-link"><img src={ExternalLink} class="icon"/></button>
            <button class="library-item-pop-over-play"><img src={Play} class="icon"/></button>
          </div>
        </div>
      </div>
    </div>
  );
}

export default Card;
