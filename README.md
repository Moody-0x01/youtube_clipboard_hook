# Desc
- An open source command line tool to download data from the net.
- examples:
```
[*] - Youtube links
[*] - Media links (Any link that has a valid media format at the end like exm: .mp3, .pdf, .m4a, .wav...)
```

# Deps
- Cargo (To build the project lol)
- ffmpeg (For better quality generally)
- libx11 (to be able to link with so u can monitor ur clipboard)
- yt-dlp (for downloading youtube link)
- wget (for media links)

# Setup
1) Build
```sh
$> cargo build
```

2) Run
```sh
$> cargo run
```
- When u do a run, the command will start listening for links that u copy in ur clipboard, if it catches a link that can be downloaded as media it just downloads it and logs that it is doing so in ur terminal...
# Usage
```
Usage: {} [DOWNLOAD_PATH] [...Options], program_name
Options:
   -h  | --help: Displays this menu
   -fs | --fmts: specifies which format to care about when getting links to download
      Example: {} . -fs \'.mp3 .mp4 .wav\', program_name
      Explaination: it will only download the formats given
-y  | --use_youtube: Use `yt-dlp` to download youtube videos
-w  | --use_wget: Use `wget` to download content
-q  | --quiet: be quiet lol
```
