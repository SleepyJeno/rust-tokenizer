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
* (Optional) retrieve your tokenized string from the database with `psql`
```
export 
export PGPASSWORD=<your_pg_password>
psql -h localhost -p 5443 -u postgres -c "SELECT * FROM tokens WHERE token_id = <your_token_id>"
```
* Send a detokinization query using `curl`
```
curl -v -H "Content-Type: application/json" -X POST http://<url>:<port>/detokenize -d '{"input": "your_tokenized_string_here"}'
```

## Tokenization algorithm:
* A random hash is generated for every input and is stored along with the tokenized string