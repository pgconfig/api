---
name: "gin_fuzzy_search_limit"
version: "9.4"
type: "integer"
category: "Client Connection Defaults / Other Defaults"
short_desc: "Sets the maximum allowed result for exact search by GIN."
context: "user"
default: "0"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/9.4/runtime-config-client.html#GUC-GIN-FUZZY-SEARCH-LIMIT"
---

Soft upper limit of the size of the set returned by GIN index scans. For more information see [GIN Tips and Tricks](https://www.postgresql.org/docs/9.4/gin-tips.html).
