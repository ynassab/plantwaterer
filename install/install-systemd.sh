#!/bin/sh

UNIT_DIR=/etc/systemd/system

sudo cp systemd/plantwaterer.service "$UNIT_DIR/"
sudo cp systemd/plantwaterer.timer "$UNIT_DIR/"

systemctl daemon-reload
systemctl enable plantwaterer.timer
systemctl start plantwaterer.timer

