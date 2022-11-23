from typing import TypedDict

class Response(TypedDict):
    status: str
    content: str


def get(url: str) -> Response: ...

def batch_get(ursl: list[str]): ...
