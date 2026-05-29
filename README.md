Орехов Артем, группа 221331, Лабораторная работа 11, вариант 9, сложность средняя

# Лабораторная работа №11: Docker

Выполнены задания средней сложности:

1. Написан `Dockerfile` для Python-приложения.
2. Написан `Dockerfile` для Rust-приложения.
3. Ограничены ресурсы контейнеров: CPU, память и число процессов.

## Структура

- `python-app/app.py` — Python-приложение для подсчета простых чисел.
- `python-app/Dockerfile` — Dockerfile для Python-приложения.
- `rust-app/src/main.rs` — Rust-приложение для подсчета простых чисел.
- `rust-app/Dockerfile` — многоступенчатый Dockerfile для Rust-приложения.
- `docker-compose.yml` — запуск контейнеров с ограничениями ресурсов.

Оба приложения решают одну и ту же вычислительную задачу, что удобно для сравнения работы контейнеров.

## Запуск через Docker Compose

Собрать образы:

```powershell
docker compose build
```

Запустить Python-приложение:

```powershell
docker compose run --rm python-app
```

Запустить Rust-приложение:

```powershell
docker compose run --rm rust-app
```

Запустить оба контейнера:

```powershell
docker compose up --build
```

## Ограничения ресурсов

В `docker-compose.yml` заданы лимиты:

- `python-app`: `0.50` CPU, `128m` памяти, `64` процесса.
- `rust-app`: `0.50` CPU, `64m` памяти, `64` процесса.

Используются параметры:

- `cpus` — ограничение доли процессора.
- `mem_limit` — максимальный объем оперативной памяти.
- `memswap_limit` — запрет использовать swap сверх лимита памяти.
- `pids_limit` — ограничение количества процессов внутри контейнера.

## Запуск без Compose

Python:

```powershell
docker build -t lab11-python-app ./python-app
docker run --rm --cpus="0.50" --memory="128m" --memory-swap="128m" --pids-limit=64 lab11-python-app
```

Rust:

```powershell
docker build -t lab11-rust-app ./rust-app
docker run --rm --cpus="0.50" --memory="64m" --memory-swap="64m" --pids-limit=64 lab11-rust-app
```

## Изменение нагрузки

Количество проверяемых чисел задается переменной окружения `LIMIT`.

Пример:

```powershell
docker compose run --rm -e LIMIT=200000 rust-app
```
