def safe_divide(a: int, b: int) -> int:
    if b == 0:
        raise ValueError("division by zero")
    return int(a / b)
