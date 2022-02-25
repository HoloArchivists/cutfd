cutfd
=====
A tool to find cuts in audio using MASS.

Requires having both the cut version and the original
[![asciicast](https://asciinema.org/a/4uCPMdDb1WWMW8tB5Jy6HISzY.png)](https://asciinema.org/a/4uCPMdDb1WWMW8tB5Jy6HISzY)

## Getting started
### Convert to the required format
The software currently only works with wav f32 one channel.

```bash
ffmpeg -i file.ext -acodec pcm_f32le -ac 1 -ar 44100 file.wav
```
### Finding the cuts
```bash
cutfd --original file.wav --copy file_copy.wav
# output:
# Cut at 1:12:21.133
# Cut end at 1:12:36.783
```
## Installation
Download from the releases: https://github.com/Ryu1845/cutfd/releases

If you're on Linux
- Make it executable  `chmod +x /path/to/cutfd`
- (Optional) copy it to a directory in your PATH

If you're on windows, just download it and use the path of the file instead of `cutfd` in the examples.