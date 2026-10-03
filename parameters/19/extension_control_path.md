---
name: "extension_control_path"
version: "19"
type: "string"
category: "Client Connection Defaults / Other Defaults"
short_desc: "Sets the path for extension control files."
extra_desc: "The remaining extension script and secondary control files are then loaded from the same directory where the primary control file was found."
context: "superuser"
default: "$system"
url: "https://www.postgresql.org/docs/19/runtime-config-client.html#GUC-EXTENSION-CONTROL-PATH"
---

A path to search for extensions, specifically extension control files (`name.control`). The remaining extension script and secondary control files are then loaded from the same directory where the primary control file was found. See [Extension Files](https://www.postgresql.org/docs/19/extend-extensions.html#EXTEND-EXTENSIONS-FILES) for details.

The value for `extension_control_path` must be a list of absolute directory paths separated by colons (or semi-colons on Windows). If a list element starts with the special string `$system`, the compiled-in PostgreSQL extension directory is substituted for `$system`; this is where the extensions provided by the standard PostgreSQL distribution are installed. (Use `pg_config --sharedir` to find out the name of this directory.) For example:

```
extension_control_path = '/usr/local/share/postgresql:/home/my_project/share:$system'
```

or, in a Windows environment:

```
extension_control_path = 'C:\tools\postgresql;H:\my_project\share;$system'
```

Note that the specified paths elements are expected to have a subdirectory `extension` which will contain the `.control` and `.sql` files; the `extension` suffix is automatically appended to each path element.

The default value for this parameter is `'$system'`. If the value is set to an empty string, the default `'$system'` is also assumed.

If extensions with equal names are present in multiple directories in the configured path, only the instance found first in the path will be used.

This parameter can be changed at run time by superusers and users with the appropriate `SET` privilege, but a setting done that way will only persist until the end of the client connection, so this method should be reserved for development purposes. The recommended way to set this parameter is in the `postgresql.conf` configuration file.

Note that if you set this parameter to be able to load extensions from nonstandard locations, you will most likely also need to set [`dynamic_library_path`](https://www.postgresql.org/docs/19/runtime-config-client.html#GUC-DYNAMIC-LIBRARY-PATH) to a correspondent location, for example,

```
extension_control_path = '/usr/local/share/postgresql:$system'
dynamic_library_path = '/usr/local/lib/postgresql:$libdir'
```
