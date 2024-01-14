# Native Rust tokenisation backend

## Key Functionality:
* Tokenize given input using a secure hash algorithm
* De-tokenize the token back into original input
* Input string limit - 128 bytes

## Requirements:
* Postgresql database engine version >= 12
* Docker

## Usage:
* A one-click setup is available with Docker: 
```
docker compose up
```
* Send a tokenization query using `curl`
```
 curl -v -H "Content-Type: application/json" -X POST  http://<url>:<port>/tokenize -d '{"input": "your_string_here"}'   
```

## Tokenization algorithm:
* A random hash is generated for every input and is stored along with the tokenized string