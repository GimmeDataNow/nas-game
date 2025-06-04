curl -H 'Content-Type: application/json' \
      -d '{
  "games": [
    "The Legend of Zelda: Breath of the Wild",
    "God of War",
    "Elden Ring",
    "Hades",
    "Stardew Valley",
    "Red Dead Redemption 2",
    "The Witcher 3: Wild Hunt",
    "Hollow Knight",
    "Celeste",
    "Death Stranding",
    "Dark Souls III",
    "Bloodborne",
    "Sekiro: Shadows Die Twice",
    "Cyberpunk 2077",
    "Resident Evil Village",
    "Final Fantasy VII Remake",
    "Spider-Man: Miles Morales",
    "Ghost of Tsushima",
    "Disco Elysium",
    "Persona 5 Royal",
    "Slay the Spire",
    "Dead Cells",
    "Outer Wilds",
    "Return of the Obra Dinn",
    "It Takes Two"
  ]
}' \
      -X POST \
      http://127.0.0.1:53317/download_images
