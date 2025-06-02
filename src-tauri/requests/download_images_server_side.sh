curl -H 'Content-Type: application/json' \
      -d '{
  "games": [
    "The Legend of Zelda: Breath of the Wild",
    "God of War",
    "Elden Ring",
    "Hades",
    "Stardew Valley"
  ]
}' \
      -X POST \
      http://127.0.0.1:53317/download_images
