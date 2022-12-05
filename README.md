# DistKV - The persistant in-memory key value store

This is a simple key-value store implemented in Rust using the [Actix web framework](https://actix.rs).
> **Warning**
> Do NOT use this in production and do NOT ask for support as this was a proof of concept.

## Running the Project
To run the project, you will need to have the [Rust programming language](https://www.rust-lang.org/) and the [Cargo package manager](https://doc.rust-lang.org/cargo/) installed on your system.

Once you have Rust and Cargo installed, you can clone this repository and run the following command in the project directory to build and run the project:

```bash
cargo run
```
This will compile the project and start the server on http://127.0.0.1:8080.

## Using the Requests
Once the server is running, you can use the following requests to interact with the key-value store:

`GET /`

This request will return a simple message indicating that the server is running.

`GET /{key}`

This request will return the value associated with the given key in the key-value store. If the key does not exist, it will return a 404 error.

`PUT /`

This request will insert the given value into the key-value store and will generate a new key.

`PUT /{key}`

This request will insert the given key and value into the key-value store.

`DELETE /{key}`

This request will delete the given key and its associated value from the key-value store. If the key does not exist, it will return a 404 error.

`GET /list/`

This request will return a list of all keys in the key-value store.

## Example Usage
Here are some examples of how you can use these requests to interact with the key-value store:


```bash
# Check if the server is running
curl http://127.0.0.1:8080/

# Insert a key-value pair
curl -X PUT http://127.0.0.1:8080/new-post -d '{"title": "Cool Post", "content": "my cool post"}' -H "Content-Type: application/json"

# Insert a value
curl -X PUT http://127.0.0.1:8080/ -d '{"title": "Cooler Post", "content": "my cooler post"}' -H "Content-Type: application/json"

# Get the value associated with a key
curl http://127.0.0.1:8080/new-post

# Delete a key-value pair
curl -X DELETE http://127.0.0.1:8080/new-post

# Get a list of all keys in the key-value store
curl http://127.0.0.1:8080/list/?skip=0&limit=1000
```