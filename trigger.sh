#!/bin/bash

set -ex

giturl=git@github.com:certik/fpm.git
branch=repr3

for i in {1..15}
do
    echo "Trigger" >> README.md
    git commit -a -m "CI trigger"
    git push ${giturl} ${branch}
done
