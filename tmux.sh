#! /usr/bin/env bash

source .bashrc

SESH="nas-game-dev"

tmux has-session -t $SESH 2>/dev/null


if [ $? != 0 ]; then

  tmux new-session -d -s $SESH -n "edit-src"
  tmux send-keys -t $SESH:edit-src "cd ~/coding/rust/nas-game" C-m
  tmux send-keys -t $SESH:edit-src "nix-shell dev.nix" C-m C-l

  tmux new-window -t $SESH -n "edit-web"
  tmux send-keys -t $SESH:edit-web "cd ~/coding/rust/nas-game" C-m
  tmux send-keys -t $SESH:edit-web "nix-shell dev.nix" C-m C-l

  tmux new-window -t $SESH -n "pnpm"
  tmux send-keys -t $SESH:pnpm "cd ~/coding/rust/nas-game" C-m
  tmux send-keys -t $SESH:pnpm "nix-shell dev.nix" C-m C-l
  tmux send-keys -t $SESH:pnpm "pnpm run dev"

  tmux new-window -t $SESH -n "server"
  tmux send-keys -t $SESH:server "cd ~/coding/rust/nas-game" C-m
  tmux send-keys -t $SESH:server "nix-shell dev.nix" C-m
  tmux send-keys -t $SESH:server "cd ~/coding/rust/nas-game/src-tauri" C-m C-l
  tmux send-keys -t $SESH:server "cargo run -- server -ios"  

  tmux new-window -t $SESH -n "tauri"
  tmux send-keys -t $SESH:tauri "cd ~/coding/rust/nas-game" C-m
  tmux send-keys -t $SESH:tauri "nix-shell dev.nix" C-m
  tmux send-keys -t $SESH:tauri "cd ~/coding/rust/nas-game/src-tauri" C-m C-l
  tmux send-keys -t $SESH:tauri "cargo run -- client"

fi


tmux attach-session -t $SESH
