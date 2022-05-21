#!/bin/bash

sudo cp jc-dict/bin/jc-dict /bin/jc
sudo unzip jc-dict/database/ecdict-sqlite-28.zip -d opt/jc-dict/database
rm -rf jc-dict
echo "CLI简单粗暴实用小词典安装完成！"
echo "输入'jc'开始实用"