# Word Bomb External
this a completely external cheat for [wordbomb](https://www.roblox.com/games/2653064683). 

**this *does* work with byfron**

## How does this work?
it uses OCR to detect the letters on the screen and then uses a dictionary to find the best word. it then uses inputfaker to send the word to the game.

## How to use?
### Requirements
- tesseract you must install from scoop.
- [python 3+](https://www.python.org/downloads/)
- a recent version of [rust](https://rustup.rs/)
- InputFaker (you can download with [ds4windows](https://github.com/Ryochan7/DS4Windows/releases) and when installing make sure to install inputfaker)
### Steps

1. git clone the repo `git clone https://github.com/pozm/wordbomb-external`
2. pip install the requirements `pip install -r requirements.txt`. 
3. build or run using `cargo build --release` or `cargo run --release` respectively.
