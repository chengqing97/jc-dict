# CLI 简单粗暴实用小词典

————词典来源————

线下词典: ECDICT

线上词典: 有道词典

人声发音: Cambridge Dictionary

## 安装

```
git clone https://github.com/chengqing97/jc-dict.git && ./jc-dict/install.sh
```

## 使用

### 快速搜索:

```
jc [要查询的内容]
```

### 互动模式:

```
jc
```

在互动模式中搜索后可发送:

- '1' 播放英式发音
- '2' 播放美式发音
- 'i' 在有道词典搜索

## 卸载

```
sudo rm /bin/jc && sudo rm -rf /opt/jc-dict
```

## Issues

- [ ] It becomes almost unusable when getting voice takes long time
- [ ] voice playback has fixed length
- [ ] Cannot cancel requests
- [ ] git clone won't download the database zip file
- [ ] install script needs to be able to handle error
- [ ] need to make sure dependecies are installed: libasound2, unzip

## Dev

- [x] Clear terminal when program starts
