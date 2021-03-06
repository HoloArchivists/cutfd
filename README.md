cutfd
=====
A tool to find cuts in audio using MASS.

Requires having both the cut version and the original
[![asciicast](https://asciinema.org/a/g3lP8QjImxZpIjiAMKGnaCxoB.svg)](https://asciinema.org/a/g3lP8QjImxZpIjiAMKGnaCxoB?autoplay=1)

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
# ✔  Searching for cuts
# ┌ 1:49:59.15
# └ 1:52:22.314
# ┌ 2:29:59.78
# └ 2:30:16.878
```
The software has a default window of 10 minutes.
You can change that window with the `--window` option. 
The smaller the window the faster the search. 
However, if the window is too small the program will just crash. 
(with an error message like so `thread 'main' panicked at 'slice index starts at 18446744073709551615 but ends at 0'`)

If you know the files only have one cut, there is the `--one-cut` option. It's extremely fast too (<50ms for a 3h file I tested)

## Installation
Download from the releases: https://github.com/Ryu1845/cutfd/releases

If you're on Linux
- Make it executable  `chmod +x /path/to/cutfd`
- (Optional) copy it to a directory in your PATH

If you're on windows, just download it and use the path of the file instead of `cutfd` in the examples.

## Roadmap
- [ ] convert input files automatically
- [ ] detect silences
- [ ] write cut parts to separate files
