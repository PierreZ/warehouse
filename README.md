# warehouse [![Build Status](https://www.travis-ci.org/PierreZ/warehouse.svg?branch=master)](https://www.travis-ci.org/PierreZ/warehouse)
Handling packages inventory on remote servers

## Queries

```
GET /_search
{
    "query": {
        "nested" : {
            "path" : "packages",
            "query" : {
                "bool" : {
                    "must" : [
                    { "match": {"packages.name": "linux"} }
                    ]
                }
            }
        }
    }
}

```