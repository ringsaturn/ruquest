from typing import TypedDict

class Response(TypedDict):
    status_code: str
    content: str


def get(url: str) -> Response: ...

def batch_get(ursl: list[str]) -> list[Response]: ...
