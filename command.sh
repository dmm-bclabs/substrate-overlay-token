#!/bin/bash

help () {
  echo "It receive only below argument." 1>&2
  echo "----" 1>&2
  echo "build" 1>&2
  echo "log parent|child|bridge" 1>&2
  echo "run" 1>&2
  echo "stop" 1>&2
  exit 1
}

container_name_error () {
  echo "Container name only receive below name." 1>&2
  echo "----" 1>&2
  echo "parent" 1>&2
  echo "child" 1>&2
  echo "bridge" 1>&2
  exit 1
}
if [ $# -gt 2 ]; then
    help
fi

if [ $1 = "build" ]
then
    cd substrate-overlay-token \
    && ./scripts/build.sh \
    && cargo build --release
elif [ $1 = "run" ]
then
    if [ ! -e parent.sh ] || [ ! -e child.sh ] || [ ! -e docker-compose.yaml ] || [ ! -e default.conf ]
    then
        cp -r substrate-overlay-token/docker/. .
    fi
    sudo docker-compose up -d
elif [ $1 = "log" ]
then
    if [ $2 = "parent" ] || [ $2 = "child" ] || [ $2 = "bridge" ]
    then
        sudo docker-compose logs -f --tail=10 $2
    else
        container_name_error
    fi
elif [ $1 = "stop" ]
then
    sudo docker-compose stop
else 
    help
fi

