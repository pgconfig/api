---
name: "custom_variable_classes"
version: "9.1"
type: "string"
category: "Customized Options"
short_desc: "Sets the list of known custom variable classes."
context: "sighup"
url: "https://www.postgresql.org/docs/9.1/runtime-config-custom.html#GUC-CUSTOM-VARIABLE-CLASSES"
---

This variable specifies one or several class names to be used for custom variables, in the form of a comma-separated list. A custom variable is a variable not normally known to PostgreSQL proper but used by some add-on module. Such variables must have names consisting of a class name, a dot, and a variable name. `custom_variable_classes` specifies all the class names in use in a particular installation. This parameter can only be set in the `postgresql.conf` file or on the server command line.
