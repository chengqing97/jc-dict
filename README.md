# wd-dict

CLI 有道词典

> Currently only works on Linux

## Installation

```
curl -L -o wd https://github.com/chengqing97/wd-dict/raw/main/wd && chmod +x wd && sudo mv wd /bin/wd && printf "wd-dict has been successfully installed! \nType 'wd' to start.\n"
```

## Usage

```
wd hello
```

or interactive mode

```
wd
```
Send '1' or '2' after searching something in interactive mode to play pronunciation.

## To-dos

- [x] Navigate around typed words
- [x] Pronunciation playback
- [x] Chinese to English lookup

## Known issues
- <s>Crash when trying to play pronunciation that it can't find.</s> Fixed


## Uninstall

```
sudo rm /bin/wd
```
