#!/bin/sh
set -e
mkdir -p assets
curl -L https://github.com/bevyengine/bevy/raw/main/assets/bevy_bird.png -o assets/bevy_bird.png
curl -L https://github.com/bevyengine/bevy/raw/main/assets/bevy_icon.png -o assets/bevy_icon.png
curl -L https://github.com/bevyengine/bevy/raw/main/assets/crosshair.png -o assets/crosshair.png
