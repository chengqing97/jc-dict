#!/bin/bash

curl -L -o wd https://github.com/chengqing97/wd-dict/raw/main/wd
chmod +x wd
sudo mv wd /bin/wd
echo "wd-dict has been successfully installed!"
echo "Type 'wd' to start."