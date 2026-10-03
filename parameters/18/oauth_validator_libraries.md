---
name: "oauth_validator_libraries"
version: "18"
type: "string"
category: "Connections and Authentication / Authentication"
short_desc: "Lists libraries that may be called to validate OAuth v2 bearer tokens."
context: "sighup"
default: ""
url: "https://www.postgresql.org/docs/18/runtime-config-connection.html#GUC-OAUTH-VALIDATOR-LIBRARIES"
---

The library/libraries to use for validating OAuth connection tokens. If only one validator library is provided, it will be used by default for any OAuth connections; otherwise, all [`oauth` HBA entries](https://www.postgresql.org/docs/18/auth-oauth.html) must explicitly set a `validator` chosen from this list. If set to an empty string (the default), OAuth connections will be refused. This parameter can only be set in the `postgresql.conf` file.

Validator modules must be implemented/obtained separately; PostgreSQL does not ship with any default implementations. For more information on implementing OAuth validators, see [OAuth Validator Modules](https://www.postgresql.org/docs/18/oauth-validators.html).
