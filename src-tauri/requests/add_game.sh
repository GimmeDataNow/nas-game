curl -H 'Content-Type: application/json' \
      -d '[{ "launcher": "Steam", "id": "1"}, { "launcher": "Steam", "id": "2"}, { "launcher": "Gog", "id": "wow"}, { "launcher": "Steam", "id": "2"}]' \
      -X POST \
      http://127.0.0.1:53317/games
