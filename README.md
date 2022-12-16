# VBank - The persistant in-memory key value store

This is a simple key-value store implemented in Rust using the [Actix web framework](https://actix.rs).
> **Warning**
>
> Do NOT use this in production and do NOT ask for support as this was a proof of concept.
> Error when using, sometime only one thread gets an item.

## Running the Project
To run the project, you will need to have the [Rust programming language](https://www.rust-lang.org/) and the [Cargo package manager](https://doc.rust-lang.org/cargo/) installed on your system.

Once you have Rust and Cargo installed, you can clone this repository and run the following command in the project directory to build and run the project:

```bash
cargo run
```
This will compile the project and start the server on http://127.0.0.1:8080.

## Using the Requests
Once the server is running, you can use the following requests to interact with the key-value store:

> **Note**
>
> namespaces are only a placeholder, they will all lead to the same node (the only node)
> the node will write to the same file database.vbank


`GET /{namespace}/`

This request will return a simple message indicating that the server is running.

`GET /{namespace}/{key}`

This request will return the value associated with the given key in the key-value store. If the key does not exist, it will return a 404 error.

`PUT /{namespace}/`

This request will insert the given value into the key-value store and will generate a new key.

`PUT /{namespace}/{key}`

This request will insert the given key and value into the key-value store.

`DELETE /{namespace}/{key}`

This request will delete the given key and its associated value from the key-value store. If the key does not exist, it will return a 404 error.

`GET /{namespace}/list/`

This request will return a list of all keys in the key-value store.

## Example Usage
Here are some examples of how you can use these requests to interact with the key-value store:


```bash
# Check if the server is running
curl http://127.0.0.1:8080/posts/

# Insert a key-value pair
curl -X PUT http://127.0.0.1:8080/posts/new-post -d '{"title": "Cool Post", "content": "my cool post"}' -H "Content-Type: application/json"

# Insert a value
curl -X PUT http://127.0.0.1:8080/posts/ -d '{"title": "Cooler Post", "content": "my cooler post"}' -H "Content-Type: application/json"

# Get the value associated with a key
curl http://127.0.0.1:8080/posts/new-post

# Delete a key-value pair
curl -X DELETE http://127.0.0.1:8080/posts/new-post

# Get a list of all keys in the key-value store
curl http://127.0.0.1:8080/posts/list/?skip=0&limit=1000
```


### Testing Results

hard limit of requests it can take with an item of

test data:
```json
// Test Data
{
    "title": "Title",
    "description": "Test Description",
    "price": 100.00,
    "quantity": 100,
    "category": "test_category",
    "image": "test_image",
    "rating": 5.0,
}

// Updated Test Data
{
    "title": "New Title",
    "description": "New Test Description",
    "price": 95.99,
    "quantity": 25,
    "category": "test_category",
    "image": "new_test_image",
    "rating": 5.5,
}
```

output:
```
Made 1 requests in 0.00018310546875 seconds using 100 threads.
Made 101 requests in 0.07857799530029297 seconds using 100 threads.
Made 201 requests in 0.14663195610046387 seconds using 100 threads.
Made 301 requests in 0.22292494773864746 seconds using 100 threads.
Made 401 requests in 0.3342561721801758 seconds using 100 threads.
Made 501 requests in 0.39011693000793457 seconds using 100 threads.
Made 601 requests in 0.4560239315032959 seconds using 100 threads.
Made 701 requests in 0.5776610374450684 seconds using 100 threads.
Made 801 requests in 0.710881233215332 seconds using 100 threads.
```

system info:
```
System: Mac os 13.0.1

Ram: 16gb

Platform: M1
```