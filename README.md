# warehouse [![Build Status](https://www.travis-ci.org/PierreZ/warehouse.svg?branch=master)](https://www.travis-ci.org/PierreZ/warehouse)
Handling packages inventory on remote servers

# Features

* ansible-style: no agent, only ssh commands
* backed by ElasticSearch: make your own queries!

# Usage

```bash

# generate a key and put it on the server
ssh-keygen -t ed25519 -C "toto@example.com"

# configure warehouse client
vim config.toml

# run warehouse
cargo run -- -c config.toml scan 213.32.78.58:22
```

## Example of queries

```
GET /_search
{
    "query": {
        "bool": {
            "must": [
                {
                    "match_phrase": {
                        "name": "linux-base"
                    }
                },
                {
                    "prefix": {
                        "version": "4"
                    }
                }
            ]
        }
    }
}
```