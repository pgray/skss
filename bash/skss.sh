#!/usr/bin/env bash
USER=${USER:-pgray}
HOST=${HOST:-github.com}
POSTUSERPATTERN=${POSTUSERPATTERN:-.keys}
PROGRAM=skss
USERAUTHFILE="/home/$USER/.ssh/authorized_keys"

TMPFILE="/var/tmp/$PROGRAM/${USER}${POSTUSERPATTERN}"
URL="https://${HOST}/${USER}${POSTUSERPATTERN}"
TMPVAL=
[[ -f $TMPFILE ]] && TMPVAL="$(curl -Ss $URL)" || curl -Ss $URL > /var/tmp/$PROGRAM/${USER}${POSTUSERPATTERN}

[[ -d /home/pgray/.ssh ]] && [[ "$(sha256sum $TMPFILE)" != "$(sha256sum $USERAUTHFILE)"]] && mv $TMPFILE $USERAUTHFILE
