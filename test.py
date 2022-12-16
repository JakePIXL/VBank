import requests
import time
from threading import Thread

# the base URL of the database
base_url = "http://localhost:8080/testing"

# the number of parallel threads to use
num_threads = 100

test_data = {
    "title": "Title",
    "description": "Test Description",
    "price": 100.00,
    "quantity": 100,
    "category": "test_category",
    "image": "test_image",
    "rating": 5.0,
}

updated_test_data = {
    "title": "New Title",
    "description": "New Test Description",
    "price": 95.99,
    "quantity": 25,
    "category": "test_category",
    "image": "new_test_image",
    "rating": 5.5,
}
headers = {"Content-Type": "application/json"}

# a function to make a PUT request
def put_request():
    url = f"{base_url}/"
    _ = requests.put(url, json=test_data, headers=headers)
    
def make_test_key():
    url = f"{base_url}/test_key"
    _ = requests.put(url, json=test_data, headers=headers)
    
def make_second_test_key():
    url = f"{base_url}/test_key_two"
    _ = requests.put(url, json=updated_test_data, headers=headers)

# a function to make a GET request
def get_request():
    url = f"{base_url}/test_key"
    _ = requests.get(url)

# a function to make a PATCH request
def patch_request():
    url = f"{base_url}/test_key"
    _ = requests.patch(url, json=updated_test_data, headers=headers)

# a function to make a DELETE request
def delete_request():
    url = f"{base_url}/test_key"
    _ = requests.delete(url)

# a function to make a GET /list request
def list_request():
    url = f"{base_url}/list/?skip=5&limit=100"
    _ = requests.get(url)

def run_threaded_requests():
    
    # create a list of functions to call
    request_functions = [make_test_key, make_second_test_key, put_request, get_request, patch_request, delete_request, list_request]

    # make the requests in parallel
    for num_requests in range(1, 900, 100):
        # reset the timer
        start_time = time.time()
        
        # make the requests
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


if __name__ == "__main__":
    run_threaded_requests()