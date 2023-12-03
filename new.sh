#!/bin/bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

USAGE_STR="usage: $0 -d <folder_name>"

#opts
FOLDER_NAME=""

while getopts "hd:" arg; do
  case ${arg} in
    h)
      echo -e ${USAGE_STR} && exit
      ;;
    d)
      FOLDER_NAME=${OPTARG}
      ;;
  esac
done

if [ "${FOLDER_NAME}" = "" ]; then
  echo "error: no folder name" && echo -e ${USAGE_STR} && exit 1
fi

if [ -d ${FOLDER_NAME} ]; then
  echo "error: folder exists" && exit 1
fi

if ! mkdir ${FOLDER_NAME}; then
  echo "error: mkdir ${FOLDER_NAME} failed" && exit 1
fi

if ! cd ${FOLDER_NAME}; then
  echo "error: cd ${FOLDER_NAME} failed" && exit 1
fi

if ! cargo init; then
  echo "error: cargo init failed" && exit 1
fi
