/* @refresh reload */
import { createSignal, onMount } from "solid-js";
import fallbackImg from './assets/Hollow_Knight_cover_art.webp';

function Card(props) {
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
        <div class="library-item-pop-over-game-title">Hollow Knight</div>
        <div class="library-item-pop-over-play-and-stats">
          <div class="">State</div>
          <div class="">Not installed</div>
          <div class="">Playtime</div>
          <div class="">x hours played</div>
          <div class="">Status</div>
          <div class="">Not completed</div>
          <div class="library-item-pop-over-buttons">
            <button>external link</button>
            <button>play</button>
          </div>
        </div>
      </div>
    </div>
  );
}

export default Card;
