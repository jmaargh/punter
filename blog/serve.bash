#!/bin/bash

cd `dirname "${BASH_SOURCE[0]}"`
hugo server -D --bind 0.0.0.0
