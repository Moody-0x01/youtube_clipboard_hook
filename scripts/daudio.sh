#!/bin/bash


# yt-dlp -x --audio-format mp3 --embed-thumbnail --add-metadata $1
if [[ $1 ]]; then
	echo "Downloading: $1"
	set -ex
	yt-dlp -x --cookies-from-browser firefox --audio-format mp3 --audio-quality 0 --embed-thumbnail --add-metadata --embed-chapters --parse-metadata "title:%(artist)s - %(title)s" $1
else
	echo "Usage: $0 <YT-URL>"
fi
