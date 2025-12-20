# cuisine

A simple backend for serving my list of favorite foods. Not much else

## Features

This is a very simple key-value store.

The data is persistent (writes to disk), and the server caches results for performance.

### API
```
# Gets the key at test
GET /data key=test

# Puts the value at the key
PUT /data key=test value=abc auth=mypassword

# Empty string deletes the key
PUT /data key=test value= auth=mypassword

# Error: key must be underscores, lowercase letters, or numbers
PUT /data key=!* value=abc auth=mypassword

# Nonexistent or invalid key gives empty string
GET /data key=!*
```

## Development
Simply add a `.env` file based on `.env.example`, then run:
```sh
cargo run
```

## Deployment
This can be deployed anywhere that offers persistent file storage, e.g. Railway volumes
