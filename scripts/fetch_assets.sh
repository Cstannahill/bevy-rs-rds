#!/usr/bin/env bash
set -e
mkdir -p assets
curl -L -o assets/block.png https://raw.githubusercontent.com/bevyengine/bevy/main/assets/android-res/mipmap-mdpi/ic_launcher.png
curl -L -o assets/ground.png https://raw.githubusercontent.com/bevyengine/bevy/main/assets/branding/icon.png
curl -L -o assets/terrain.png https://raw.githubusercontent.com/bevyengine/bevy/main/assets/branding/bevy_logo_light.png

