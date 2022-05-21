#!/bin/bash


# 1. download zip
# /jc-dict
# ----bin/jc-dict
# ----database/stardict.db
# 2. extract zip to home dir
# 3. copy executable to bin
# 4. done

sudo cp jc-dict/bin/jc-dict /bin/jc
rm -rf jc-dict
echo "CLI简单粗暴实用小词典安装完成！"
echo "输入'jc'开始实用"