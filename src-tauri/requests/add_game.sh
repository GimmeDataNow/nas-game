curl -H 'Content-Type: application/json' \
      -d '[
  {
    "launcher": [
      {
        "name": "Steam",
        "game_id": "123456"
      },
      {
        "name": "Epic Games",
        "game_id": "abcde"
      }
    ],
    "steam_grid_id": "78910"
  },
  {
    "launcher": [
      {
        "name": "GOG Galaxy",
        "game_id": "gog-001"
      }
    ],
    "steam_grid_id": null
  },
  {
    "launcher": [],
    "steam_grid_id": "55555"
  }
]' \
      -X POST \
      http://127.0.0.1:53317/games
