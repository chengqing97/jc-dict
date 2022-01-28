# wd-dict

CLI 有道词典

> Currently only works on Linux

## Installation

```
git clone https://github.com/chengqing97/wd-dict.git
sudo cp wd-dict/wd /bin/wd
rm -rf wd-dict #optional
```

## Usage

```
wd hello
```

or interactive mode

```
wd
```

## To-dos

- [x] Navigate around typed words
- [x] Pronunciation playback
- [ ] Pronunciation playback without printing out '1' or '2'
- [ ] Chinese to English lookup
- [ ] Offline dictionary
- [ ] Offline dictionary with flag to search online

## Known issues
- Crash when trying to play pronunciation that it can't find.

## Uninstall

```
sudo rm /bin/wd
```
