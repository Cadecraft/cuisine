# cuisine

A simple backend for storing my list of favorite foods. Not much else

## Features

This is a very simple key-value store, with the use case of frequent fetches and infrequent updates.

The data is persistent (writes to disk), and the server caches results for performance.

### API
```
# -> gets the value of the key at test
GET /data key=test

# -> puts the value at the key
PUT /data key=test value=abc password=mypassword

# Empty string -> deletes the key
PUT /data key=test value= password=mypassword

# Error: key must be underscores, lowercase letters, or numbers
PUT /data key=!* value=abc auth=mypassword

# Nonexistent or invalid key -> empty string
GET /data key=!*
```

## Development
Simply add a `.env` file based on `.env.example` (a utility is provided for hashing your password), then run:
```sh
cargo run
```

## Deployment
This can be deployed anywhere that offers persistent file storage, e.g. Railway volumes. Update the `KV_PATH` environment variable accordingly
