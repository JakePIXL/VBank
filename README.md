# VBank - The persistant in-memory key value store

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


### Testing Results

hard limit of requests it can take with an item of

data:
`JrhG1ePu|{"key":"value"}`

output:
```
Made 1 requests in 0.00011587142944335938 seconds using 100 threads.
Made 101 requests in 0.0737149715423584 seconds using 100 threads.
Made 201 requests in 0.1371469497680664 seconds using 100 threads.
Made 301 requests in 0.20068883895874023 seconds using 100 threads.
Made 401 requests in 0.2744600772857666 seconds using 100 threads.
Made 501 requests in 0.3543989658355713 seconds using 100 threads.
Made 601 requests in 0.4169590473175049 seconds using 100 threads.
Made 701 requests in 0.5468730926513672 seconds using 100 threads.
Made 801 requests in 0.5518231391906738 seconds using 100 threads.
Made 901 requests in 0.7367067337036133 seconds using 100 threads.
```

system info:
```
System: Mac os 13.0.1

Ram: 16gb

Platform: M1
```