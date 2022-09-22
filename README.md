# skss
simple key sync service

## summary
  The simple key sync service periodically syncs your ssh keys from a reliable and public source. It can be used as a cron currently. Feel free to submit a PR for other well known ssh key hosts.

### supported providers:
- github
- gitlab
- sourcehut

## install
install the binary
```
cargo build --release
# as root
cp target/release/skss /usr/local/bin/
```

for cron
```
# add a crontab entry like
0 * * * * skss --users pgray --host sourcehut
```

for systemd
```
# /etc/systemd/system/skss.timer
[Unit]
Description=Runs skss once a day

[Timer]
OnCalendar=daily

# /etc/systemd/system/skss.service
[Unit]
Description=skss cron

[Service]
Type=simple
ExecStart=/usr/local/bin/skss --users pgray --host sourcehut

[Install]
WantedBy=multi-user.target
```
