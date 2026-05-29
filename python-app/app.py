import os
import time


def is_prime(number: int) -> bool:
    if number < 2:
        return False
    if number == 2:
        return True
    if number % 2 == 0:
        return False

    divisor = 3
    while divisor * divisor <= number:
        if number % divisor == 0:
            return False
        divisor += 2

    return True


def count_primes(limit: int) -> int:
    return sum(1 for number in range(2, limit + 1) if is_prime(number))


def main() -> None:
    limit = int(os.getenv("LIMIT", "100000"))
    started_at = time.perf_counter()
    result = count_primes(limit)
    elapsed = time.perf_counter() - started_at

    print("Python-приложение в Docker")
    print(f"Задача: подсчет простых чисел от 2 до {limit}")
    print(f"Результат: {result}")
    print(f"Время выполнения: {elapsed:.6f} сек.")


if __name__ == "__main__":
    main()
