from locust import HttpUser, TaskSet, task, between
import json
import os

# Load request.json once at module level
REQUEST_JSON_PATH = os.path.join(os.path.dirname(__file__), "request.json")
with open(REQUEST_JSON_PATH, "r", encoding="utf-8") as f:
    REQUEST_PAYLOAD = json.load(f)

class UserBehavior(TaskSet):
    @task(1)
    def parse_test(self):
        self.client.post(
            "/v0/parse",
            json=REQUEST_PAYLOAD,
            headers={"Content-Type": "application/json"}
        )


class User(HttpUser):
    host = "http://localhost:9000"
    tasks = [UserBehavior]
    wait_time = between(0, 10)  # seconds