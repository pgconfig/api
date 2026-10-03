---
name: "log_hostname"
version: "14"
type: "boolean"
category: "Reporting and Logging / What to Log"
short_desc: "Logs the host name in the connection logs."
extra_desc: "By default, connection logs only show the IP address of the connecting host. If you want them to show the host name you can turn this on, but depending on your host name resolution setup it might impose a non-negligible performance penalty."
context: "sighup"
default: "off"
url: "https://www.postgresql.org/docs/14/runtime-config-logging.html#GUC-LOG-HOSTNAME"
---

By default, connection log messages only show the IP address of the connecting host. Turning this parameter on causes logging of the host name as well. Note that depending on your host name resolution setup this might impose a non-negligible performance penalty. This parameter can only be set in the `postgresql.conf` file or on the server command line.
