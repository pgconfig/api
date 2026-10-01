# Another API Options

## All rules in all environments

**URL:** [/v1/tuning/get-config-all-environments](https://api.pgconfig.org/v1/tuning/get-config-all-environments)

Show all rules for all environments. This endpoint is currently used by the site.

```json
...
"data": [
    {
    "configuration": [..],
    "environment": "WEB"
    },
    {
    "configuration": [..],
    "environment": "OLTP"
    },
    {
    "configuration": [..],
    "environment": "DW"
    },
    {
    "configuration": [..],
    "environment": "Mixed"
    },
    {
    "configuration": [..],
    "environment": "Desktop"
    }
]
...
```

## List Environments

**URL:** [/v1/tuning/list-environments`](https://api.pgconfig.org/v1/tuning/list-environments)

Show all environments.

```json
...
"data": [
    "WEB",
    "OLTP",
    "DW",
    "Mixed",
    "Desktop"
],
...
```

## PGBadger Configuration

**URL:** [/v1/generators/pgbadger/get-config`](https://api.pgconfig.org/v1/generators/pgbadger/get-config)

Show the pgbadger configurations (accepts the <code>format</code> parameter)


```json
...
"data": [
    {
    "category": "log_config",
    "description": "Logging configuration for pgbadger",
    "parameters": [
        {
        "config_value": "on",
        "name": "logging_collector"
        },
        {
        "config_value": "on",
        "name": "log_checkpoints"
        },
        {
        "config_value": "on",
        "name": "log_connections"
        },
        {
        "config_value": "on",
        "name": "log_disconnections"
        },
        {
        "config_value": "on",
        "name": "log_lock_waits"
        },
        {
        "config_value": "0",
        "name": "log_temp_files"
        },
        {
        "config_value": "C",
        "format": "String",
        "name": "lc_messages"
        },
        {
        "comment": "Adjust the minimum time to collect data",
        "config_value": "10s",
        "format": "Time",
        "name": "log_min_duration_statement"
        },
        {
        "config_value": "0",
        "name": "log_autovacuum_min_duration"
        }
    ]
},
...
```