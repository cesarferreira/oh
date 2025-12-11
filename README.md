
<h1 align="center">oh</h1>
<p align="center">Quickly <i>open</i> any app in the folder you're currently in (<i><strong>O</strong>pen <strong>H</strong>ere</i>)</p>
<p align="center">
  <a href="https://crates.io/crates/open-here"><img src="https://img.shields.io/crates/v/open-here.svg" alt="Crates.io"></a>
  <a href="https://crates.io/crates/open-here"><img src="https://img.shields.io/crates/d/open-here.svg" alt="Downloads"></a>
  <a href="https://github.com/cesarferreira/oh/blob/master/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"></a>
</p>

<p align="center">
  <img src="extras/screenshot.png" width="100%" />
</p>

## Install

```sh
cargo install open-here
```

## Usage

Run `oh` in any directory and use the fuzzy finder to select an application. The selected app will open with the current directory as its target.

```
$ oh
```

Perfect for quickly opening the current folder in apps like:
- **Android Studio** – open a project
- **VS Code** – open a workspace  
- **Finder** – browse files
- **Sourcetree** – open a git repo

## How it works

1. Scans `/Applications` for installed apps
2. Presents a fuzzy-searchable list
3. Opens the selected app with the current directory (`.`) as the argument

> **Note:** macOS only

## Created by
[Cesar Ferreira](https://cesarferreira.com)

## License
MIT © [Cesar Ferreira](http://cesarferreira.com)
