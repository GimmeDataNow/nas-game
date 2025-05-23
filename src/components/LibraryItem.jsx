/* @refresh reload */
import { createSignal, onMount } from "solid-js";
const images = import.meta.glob('./../assets/*.{webp,png,jpg,jpeg}', { import: 'default' });

import fallbackImg from './../assets/Hollow_Knight_cover_art.webp';
import ExternalLink  from "./../assets/external-link.svg"; 
import Play from "./../assets/play.svg"; 

export async function getImagePath(filename) {
  for (const path in images) {
    if (path.endsWith(filename)) {
      console.log(`Attempting to import image: ${path}`);
      try {
        const mod = await images[path]();
        console.log(`Resolved ${filename} to ${mod}`);
        return mod;
      } catch (err) {
        console.error(`Error loading image: ${filename}`, err);
      }
    }
  }
  console.warn(`Image not found for filename: ${filename}`);
  return null;
}


function Card(props) {
  const title = () => props.title ?? "Default Title";
  const state = () => props.state ?? "Not installed";
  const time = () => props.time ?? 0;
  const status = () => props.status ?? "Not completed";

  const [bgImage, setBgImage] = createSignal(fallbackImg);

  onMount(async () => {
    if (props.imgPath) {
    const resolvedPath = await getImagePath(props.imgPath);

      if (resolvedPath) {
        const img = new Image();
        img.src = resolvedPath;
        img.onload = () => setBgImage(resolvedPath);
        img.onerror = () => setBgImage(fallbackImg);
      } else {
        setBgImage(fallbackImg);
      }
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
