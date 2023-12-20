#!/bin/sh

# tinygo
wget https://github.com/tinygo-org/tinygo/releases/download/v0.26.0/tinygo_0.26.0_amd64.deb
dpkg -i tinygo_0.26.0_amd64.deb
export PATH=$PATH:/usr/local/bin
rm tinygo_0.26.0_amd64.deb

# go
wget https://go.dev/dl/go1.19.linux-amd64.tar.gz
tar -xf go1.19.linux-amd64.tar.gz
mv go /usr/local
export GOROOT=/usr/local/go
rm go1.19.linux-amd64.tar.gz
