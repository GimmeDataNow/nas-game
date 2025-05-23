/* @refresh reload */
import { createSignal, onMount } from "solid-js";
import fallbackImg from './../assets/Hollow_Knight_cover_art.webp';
import ExternalLink  from "./../assets/external-link.svg"; 
import Play from "./../assets/play.svg"; 


function Card(props) {
  const title = () => props.title ?? "Default Title";
  const state = () => props.state ?? "Not installed";
  const time = () => props.time ?? 0;
  const status = () => props.status ?? "Not completed";

  const [bgImage, setBgImage] = createSignal(fallbackImg);

  onMount(() => {
    if (props.imgPath) {
      const img = new Image();
      img.src = props.imgPath;
      img.onload = () => setBgImage(props.imgPath);
      img.onerror = () => setBgImage(fallbackImg);
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
