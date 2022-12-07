import requests
import time
from threading import Thread

# the base URL of the database
base_url = "http://localhost:8080"

# the number of requests to make
num_requests = 1000

# the number of parallel threads to use
num_threads = 100

# a function to make a PUT request
def put_request():
    url = base_url + "/"
    data = {"test_key": "test_value"}
    headers = {"Content-Type": "application/json"}
    _ = requests.put(url, json=data, headers=headers)
    
def make_test_key():
    url = base_url + "/" + "test_key"
    data = {"test_key": "test_value"}
    headers = {"Content-Type": "application/json"}
    _ = requests.put(url, json=data, headers=headers)

# a function to make a GET request
def get_request():
    url = base_url + "/" + "test_key"
    _ = requests.get(url)

# a function to make a PATCH request
def patch_request():
    url = base_url + "/test_key"
    data = {"new_test_key": "new_test_value"}
    headers = {"Content-Type": "application/json"}
    _ = requests.patch(url, json=data, headers=headers)

# a function to make a DELETE request
def delete_request():
    url = base_url + "/test_key"
    _ = requests.delete(url)

# a function to make a GET /list request
def list_request():
    url = base_url + "/list/?skip=5&limit=100"
    _ = requests.get(url)

if __name__ == "__main__":
    
    # create a list of functions to call
    request_functions = [make_test_key, put_request, get_request, patch_request, delete_request, list_request]

    # start the timer
    start_time = time.time()

    # make the requests in parallel
    for i in range(num_requests):
        # choose a random request function
        func = request_functions[i % len(request_functions)]

        # start a new thread to make the request
        thread = Thread(target=func)
        thread.start()

    # stop the timer
    end_time = time.time()

    # calculate the total time taken
    total_time = end_time - start_time

    # print the results
    print(f"Made {num_requests} requests in {total_time} seconds using {num_threads} threads.")
