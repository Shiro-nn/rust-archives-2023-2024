# rust‑archives‑2023‑2024 🗄️✨

[![GitHub stars](https://img.shields.io/github/stars/Shiro-nn/rust-archives-2023-2024?style=social)](https://github.com/Shiro-nn/rust-archives-2023-2024/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/Shiro-nn/rust-archives-2023-2024?style=social)](https://github.com/Shiro-nn/rust-archives-2023-2024/network/members)
[![GitHub issues](https://img.shields.io/github/issues/Shiro-nn/rust-archives-2023-2024)](https://github.com/Shiro-nn/rust-archives-2023-2024/issues)
[![GitHub last commit](https://img.shields.io/github/last-commit/Shiro-nn/rust-archives-2023-2024)](https://github.com/Shiro-nn/rust-archives-2023-2024/commits)
[![License: MIT](https://img.shields.io/github/license/Shiro-nn/rust-archives-2023-2024)](LICENSE)
[![Status: Archived](https://img.shields.io/badge/status-archived-lightgrey.svg)](https://github.com/Shiro-nn/rust-archives-2023-2024)

![Repo Stats](https://github-readme-stats.vercel.app/api/pin/?username=Shiro-nn\&repo=rust-archives-2023-2024)

> **rust‑archives‑2023‑2024** — это просто «склад» экспериментов на Rust, которые рождались в свободное время с марта 2023 по июль 2024 года. Каждая папка — отдельный pet‑project, а вместе их 12 штук. Полезного применения им не нашлось, поэтому весь зоопарк переехал в один архивный репозиторий. Код остаётся открытым и может пригодиться в качестве примеров, но **новых фич и багфиксов больше не планируется**.

---

## 📂 Содержимое

| Директория          | Краткое описание                                                                                     |
| ------------------- | ---------------------------------------------------------------------------------------------------- |
| `assembler_call`    | Мини‑пример `global_asm!`: Rust вызывает две функции на ассемблере и печатает 1337/666.              |
| `cheat-finder`      | Консольная утилита, ищущая на Windows следы читов (Prefetch, логи, Steam, VAC‑статус).               |
| `cheat-finder-old`  | Первая версия `cheat-finder`; оставлена для истории.                                                 |
| `encrypt-test`      | Тест шифрования встроенных ресурсов через [`include_crypt`](https://crates.io/crates/include_crypt). |
| `hook-tests`        | Черновики hook’ов WinAPI и inline‑patchинга функций.                                                 |
| `pdtest`            | Простые прототипы детектирования процессов/DLL‑инжекта.                                              |
| `randomtest`        | Случайные эксперименты без конкретной цели — «песочница» автора.                                     |
| `readlines`         | Бенчмарки различных способов чтения строк из больших файлов.                                         |
| `speedtest`         | Замеры производительности CPU/IO/Memory на чистом Rust.                                              |
| `tgtest`            | Черновик Telegram‑бота на базе `teloxide`.                                                           |
| `tool-with-service` | Пример CLI‑инструмента с установкой себя как Windows‑службы.                                         |
| `winlangtest`       | Тесты локализации и ресурсов WinAPI (языковые dll/rc‑файлы).                                         |

> Кроме директорий, в корне лежат отдельные скрипты (`core.rs`, `get_sessions.rs`, `test.rs`) — это мелкие проверки и демонстрации, не попавшие в папки.

---

## 🛠️ Как запускать

Каждый подпроект — **изолированный** Cargo‑крейт. Формально они не объединены в workspace, поэтому запуск выглядит так:

```bash
# Пример: собрать и запустить cheat-finder
git clone https://github.com/Shiro-nn/rust-archives-2023-2024.git
cd rust-archives-2023-2024/cheat-finder
cargo run --release        # или cargo build --release
```

> ⚠️ **Windows‑специфичные проекты** (например, `cheat-finder`, `tool-with-service`, `hook-tests`) требуют toolchain `x86_64-pc-windows-msvc` и иногда запуска из‑под администратора.

### Системные требования

* **Rust 1.75+** (edition 2021);
* Для некоторых проектов: `clang` / Windows SDK / `teloxide` токен.

---

## 🤝 Вклад

Репозиторий помечен как **архив**: pull‑request’ы принимаются только по мелким фиксам (broken build, опечатки). Если захотите доработать одну из идей — feel free to fork!

---

## ⚖️ Лицензия

Проекты распространяются под лицензией **MIT** — можно копировать, изменять и использовать без ограничений, но **без гарантий**.

> Спасибо, что заглянули! Пускай эти кусочки кода послужат полезными примерами или вдохновением для ваших проектов.
