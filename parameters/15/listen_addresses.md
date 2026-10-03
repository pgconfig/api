---
name: "listen_addresses"
version: "15"
type: "string"
category: "Connections and Authentication / Connection Settings"
short_desc: "Sets the host name or IP address(es) to listen to."
context: "postmaster"
default: "localhost"
url: "https://www.postgresql.org/docs/15/runtime-config-connection.html#GUC-LISTEN-ADDRESSES"
---

Specifies the TCP/IP address(es) on which the server is to listen for connections from client applications. The value takes the form of a comma-separated list of host names and/or numeric IP addresses. The special entry `*` corresponds to all available IP interfaces. The entry `0.0.0.0` allows listening for all IPv4 addresses and `::` allows listening for all IPv6 addresses. If the list is empty, the server does not listen on any IP interface at all, in which case only Unix-domain sockets can be used to connect to it. If the list is not empty, the server will start if it can listen on at least one TCP/IP address. A warning will be emitted for any TCP/IP address which cannot be opened. The default value is `localhost`, which allows only local TCP/IP "loopback" connections to be made.

While client authentication ([Client Authentication](https://www.postgresql.org/docs/15/client-authentication.html)) allows fine-grained control over who can access the server, `listen_addresses` controls which interfaces accept connection attempts, which can help prevent repeated malicious connection requests on insecure network interfaces. This parameter can only be set at server start.
