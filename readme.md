# Word Bomb External
this a completely external cheat for [wordbomb](https://www.roblox.com/games/2653064683). 

**this *does* work with byfron**

## How does this work?
it uses OCR to detect the letters on the screen and then uses a dictionary to find the best word. it then uses inputfaker to send the word to the game.

## How to use?
### Requirements
- tesseract cli.
- InputFaker (you can download with [ds4windows](https://github.com/Ryochan7/DS4Windows/releases) and when installing make sure to install inputfaker)
### Requirements (build)
- [opencv](https://github.com/twistedfall/opencv-rust/blob/master/INSTALL.md)
- msvc compiler
### Steps (build)

1. cargo build --release
2. put all dlls in current working directory (when running).
3. profit.


### notice
1. Please note that the OCR / word detection is not perfect and normally slips up on I / L's due to the font which the game uses. Currently i don't know of a way to make it more reliable so it'll have to make do.
2. I use a HDR monitor. the colours hardcoded for me may be different. if the tool doesn't work try changing upper_range and lower_range in cv/partial.rs
