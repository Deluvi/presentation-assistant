# Snips RevealJS and Key-based Presentation Assistant 👩‍🏫 👨‍🏫

Action code and template for my [RevealJS](https://github.com/hakimel/reveal.js) presentation assistant and action code for any slideshow software using keyboard inputs. Use your voice to control your slideshow!

## Setup 💻

Install the associated Snips app on your assistant :

- 🇫🇷 French : [Diaporama](https://console.snips.ai/store/fr/skill_gw50Xzv0X4Q)
- 🇬🇧 English : Coming Soon™

Install the Snips platform and your assistant on your device. Start the action code and open your Reveal JS presentation on a webserver or your favorite slideshow software.

## Build 🛠

The action code consists of a Rust crate. To build a Rust crate, install Rust with Cargo, change your current directory to the root of the repository and type :

```
cargo build --release
```

## Run 🏃‍♂️

If you want to run your action code, type:

```
cargo run --release
```

### Notes for the RevealJS action code ❗️

Because of security restrictions on your browser, you should run your RevealJS presentation on a webserver to be able to access the action code. If you have Python 3 on your computer for example, set your current directory to the RevealJS folder and use the following command to host a webserver:

```
python3 -m http.server
```

The webserver should be accessible on the 8000 port. Just access it on your browser using the address `localhost:8000`.

## How (RevealJS)? 🕹

The special RevealJS template included in the `template` folder includes some Javascript code to open a websocket. The action code includes a websocket server responsible to relay any intents related to the presentation app to the RevealJS app which will react accordingly.

## How (Key-based inputs)? ⌨️

The action code will use the left and right arrow keys to get to the next or previous slide (those keys should be commonly supported by any presentation software). The slideshow software should be in focus for this method to work.
