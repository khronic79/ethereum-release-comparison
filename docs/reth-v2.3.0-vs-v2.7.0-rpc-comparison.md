# Reth v2.3.0 → v2.7.0: спецификация изменений для потребителей JSON-RPC

| Параметр | Значение |
|---|---|
| Документ | Сравнительное исследование версий Reth для потребителей JSON-RPC |
| Текущая версия узла | Reth **v2.3.0** (10.06.2026) |
| Целевая версия | Reth **v2.7.0** (28.09.2026, latest на дату отчёта) |
| Аудитория | Команды-потребители JSON-RPC API (индексаторы, эксплореры, кошельки, боты, MEV/builder-инфраструктура, внутренние сервисы) |
| Статус | Research / для внутреннего согласования перед апгрейдом |
| Источники | GitHub Releases `paradigmxyz/reth`, официальная документация `reth.rs/jsonrpc` |

> **Назначение.** Помочь потребителям заранее выявить изменения, способные сломать или изменить поведение их клиентов, и спланировать правки. Документ описывает только те изменения, которые видны снаружи через JSON-RPC и влияют на ответы/поведение методов. Внутренние SDK-изменения Rust здесь не рассматриваются (см. раздел «За рамками»).

---

## 1. Краткое резюме для потребителей (TL;DR)

Апгрейд **в целом обратносовместим** для стандартных Ethereum-методов (`eth_*`, `web3_*`, `net_*`, `txpool_*`, `debug_*`, `trace_*`). Поверхность API преимущественно **расширяется** новыми методами. Однако есть несколько **поведенческих breaking changes**, которые обязан проверить каждый потребитель:

1. **`testing` больше не входит в `all`** (v2.7.0). Если вы полагались на `--http.api all` для доступа к `testing_*`, теперь namespace нужно указывать явно. *(Если `testing` вы не используете — не влияет.)*
2. **`trace_callMany` и `trace_filter` без указания блока/границ теперь дефолтят на `latest`** (v2.7.0), а не на pending/историю. Клиентам, которым нужен pending или диапазон, надо указывать его явно.
3. **`eth_estimateGas` теперь учитывает `--rpc.gascap`** (v2.7.0): при заданном на узле gascap оценка газа будет ограничена сверху. Если узел раньше не имел gascap — поведение не изменится; если имел — оценки могут стать меньше.
4. **`trace_get` изменён** (v2.7.0): выбор результата по пути в дереве трасс; повтор несуществующей транзакции возвращает `null` (а не ошибку).
5. **`eth_getMultiProof` всегда возвращает proof V2** (v2.6.0) — изменился формат ответа для downstream-интеграций.
6. **`eth_getLogs` по очищенным (pruned) receipts возвращает ошибку**, а не неполный результат (v2.6.0).
7. **Включён partial persistence / state masking по умолчанию** (v2.6.0). Критично для сервисов, читающих БД Reth **вне процесса узла**: рекомендовано отключить (`--engine.num-state-masking-blocks 0`). Для JSON-RPC-потребителей влияет косвенно через доступность исторического состояния.
8. **Исправления `eth_getProof` для несуществующих аккаунтов** (v2.4.0): пустые proof нормализуются в пустые массивы (geth-совместимо). Если клиент ожидал прежний не-geth формат — проверить.

Всё остальное — аддитивные методы и оптимизации производительности/кэшей, не меняющие контракт.

---

## 2. Область исследования и методология

**В скоупе:**
- Изменения списка методов, namespace'ов и их доступности.
- Изменения форматов запросов/ответов и кодов ошибок.
- Изменения поведения по умолчанию параметров.
- Изменения конфигурации RPC (`--http.api`, `--rpc.gascap`, кэши, compression), влияющие на потребителей.
- Изменения Engine API и authrpc, значимые для инфраструктуры (consensus clients, builders).
- Аппаратные/операционные изменения (persistence, pruning), влияющие на доступность данных через RPC.

**За рамками (упоминается только для контекста):**
- Изменения публичных Rust SDK-типов и трейтов (custom RPC builders, ExEx, custom payload/consensus). Для потребителей чистого JSON-RPC нерелевантно.
- Внутренние оптимизации без изменения контракта.

**Метод:** анализ release notes каждой промежуточной версии (v2.4.0, v2.4.1, v2.5.0, v2.5.1, v2.5.2, v2.6.0, v2.7.0) и официальной документации JSON-RPC для v2.7.0. Каждое изменение привязано к номеру PR в приложении.

**Ограничение:** release notes — не формальная спецификация. Перед релизом в прод рекомендуется прогнать собственный regression-набор методов (см. §8) на тестовом инстансе v2.7.0.

---

## 3. Хронология версий между v2.3.0 и v2.7.0

| Версия | Дата | Класс | Обновление обязательно? | Ключевое для RPC |
|---|---|---|---|---|
| v2.3.0 | 10.06.2026 | Базовая | — | Уже содержит `eth_pendingTransactions`, `eth_baseFee`, debug account-state, `eth_simulate` state-root, `eth_getProof` geth-zero-hashes |
| v2.4.0 | 14.07.2026 | Feature | Высокий приоритет | `eth_sendRawTransactionSync`, `engine_hasBlobs`, `debug_clearTxpool`, `admin_banPeer/unbanPeer`, правки `eth_estimateGas`/`eth_getProof`/fee-history/simulate; снят `--engine.legacy-state-root` |
| v2.4.1 | 17.07.2026 | Patch | Высокий | Только обновление alloy v1.6.1 |
| v2.5.0 | 12.08.2026 | Feature | Средний | `eth_getMultiProof`, `debug_getRawBlockAccessList`, правки `eth_simulate`/`testing_buildBlockV1`/error-codes |
| v2.5.1 | 20.08.2026 | Patch | Средний | RPC-изменений нет |
| v2.5.2 | 02.09.2026 | Patch | Высокий | RPC-изменений нет |
| v2.6.0 | 17.09.2026 | Feature | **Низкий** (для не-builder) | `debug_traceChain`, Engine API V6/V5, `eth_getMultiProof` V2, `eth_getLogs` pruned→error, partial persistence по умолчанию, HTTP compression |
| v2.7.0 | 28.09.2026 | Feature | **Низкий** | `testing` вне `all`, дефолты `trace_callMany`/`trace_filter`, `eth_estimateGas` + gascap, `trace_get` fix, Otterscan fix, `eth_config` (Glamsterdam) |

**Rollback-совместимость:**
- После v2.7.x откат на **v2.6.0** (в пределах 2.6-линии данных).
- После v2.5.x откат на **v2.4.1**.
- После v2.3.x откат на **v2.2.0**.
- ⚠️ Прямой откат v2.7.0 → v2.3.0 по формату БД/состояния может быть небезопасен. Откатывать нужно пошагово по линии совместимости.

---

## 4. Критические изменения функционала узла (влияют на доступность данных через RPC)

### 4.1. Partial persistence (state masking) включён по умолчанию — v2.6.0 ⚠️
**Что:** Начиная с v2.6.0 Reth по умолчанию не пишет в БД часть state/trie-ключей, если более поздние блоки их перезаписывают («state masking»). Новые дефолты:
- `--engine.persistence-threshold 50` (было 7)
- `--engine.num-state-masking-blocks 30` (включено по умолчанию)
- `--engine.memory-block-buffer-target 5`
- backpressure 16 → 100 блоков

**Кому важно:** любым сервисам, которые читают **БД Reth напрямую вне процесса узла** (не через RPC). Официальная рекомендация: отключить partial persistence, выставив `--engine.num-state-masking-blocks 0`.

**Действие для потребителей RPC:** уточнить у команды узла, включён ли partial persistence. Если ваши сервисы читают БД напрямую — потребовать отключения.

### 4.2. Снят legacy state-root runtime — v2.4.0
`--engine.legacy-state-root` скрыт и лишь предупреждает при передаче. Использовать `--engine.state-root-fallback` при необходимости синхронного расчёта state-root. На JSON-RPC-контракт не влияет, но на латентность `eth_call`/`eth_estimateGas` на «холодном» состоянии — может.

### 4.3. Engine API: REST-SSZ включён по умолчанию — v2.6.0
На аутентифицированном Engine API сервере по умолчанию установлены REST-SSZ маршруты. Потребители Engine API (consensus clients, кастомные узлы) должны это учитывать. Добавлены `engine_newPayloadV6`, `engine_forkchoiceUpdatedV5`, `engine_getInclusionListV1` (FOCIL), `engine_hasBlobs`.

### 4.4. Хардфорки и расписание
- v2.6.0: поддержка Amsterdam / Bogota, EIP-7928 BAL, `engine_newPayloadV6`/`forkchoiceUpdatedV5`.
- v2.7.0: подготовка Sepolia к Glamsterdam; активация Amsterdam на Sepolia **6 октября 2026 13:53:36 UTC** (операторам Sepolia — обновиться до этой даты). Тестируется переход fork ID и `eth_config`.
- **Следствие для клиентов:** после активации хардфорка в ответах могут появляться новые поля (payload/BAL-поля, `eth_config`), а семантика `eth_estimateGas`/fee-history меняется (см. §6). Индексирующим сервисам нужно уметь принимать новые поля блоков/транзакций.

### 4.5. Производительность и кэши (без изменения контракта)
- v2.4.0: mean `newPayload` latency −6.2% относительно v2.3.0.
- v2.5.0: −5–10% mean block-processing latency относительно v2.4.1.
- v2.6.0: исторические RPC-запросы (trace/state) быстрее за счёт переиспользования RocksDB snapshots/итераторов; `eth_getLogs` — оконные сканы и bloom-фильтрация; `trace_get` — ленивая конвертация; fee-history — общие записи.
- v2.7.0: **понижены дефолты RPC-кэшей**: блоки 5 000 → 1 000, receipts блоков 2 000 → 500; добавлены лимиты байт на кэш и idle-eviction.
  - Влияние: возможен рост latency на «горячих» повторяющихся запросах, если полагались на большой кэш. Настраивается через `--rpc-cache.*`.
- Новые флаги кэша BAL для трейсинга: `--rpc-cache.cache-computed-bals`, `--rpc-cache.prewarm-bals[=BLOCKS]`, `--rpc-cache.max-bals-bytes`, `--rpc-cache.idle-timeout`.

### 4.6. HTTP compression — v2.6.0
Добавлена декомпрессия входящих HTTP-запросов и allowlist `--http.compression` для алгоритмов сжатия ответов. Клиенты могут использовать gzip/br (в зависимости от конфигурации). Обратносовместимо.

---

## 5. Изменения JSON-RPC по версиям (полный перечень)

### 5.1. v2.4.0 (14.07.2026)

| Тип | Метод / элемент | Изменение | PR |
|---|---|---|---|
| Добавлено | `eth_sendRawTransactionSync` | Поддержка таймаута (EIP-7966-подобное поведение) | #25682 |
| Добавлено | `engine_hasBlobs` | Проверка доступности blob'ов без загрузки payload/proof | #25589 |
| Добавлено | `debug_clearTxpool` | Очистка txpool (geth-совместимо, для тестов/отладки) | #25588 |
| Добавлено | `admin_banPeer`, `admin_unbanPeer` | Управление peer'ами | #25140 |
| Добавлено | `EthSubscriptions` (logs/headers) | Трейт для подписок | #22155 |
| Добавлено | testing helpers | Хелперы commit/build block | #25070, #25750 |
| Поведение | `eth_sendRawTransaction` | Сабмиты трактуются как **локальные** транзакции | #25412 |
| Поведение | `eth_estimateGas` | Используется эффективный gas-cap из revm для границ оценки | #25612 |
| Поведение | fee history | Rewards взвешиваются по газу receipt (семантика Amsterdam) | #26067 |
| Формат | `eth_getProof` | Пустые account/storage proof несуществующих аккаунтов → пустые массивы (geth EIP-1186) | #24719 |
| Исправление | `eth_simulate` | Nonce-ошибки; выравнивание ошибок missing-block | #25079, #25074 |
| Формат | `eth_fillTransaction` | Дефолт `maxFeePerGas` = `2 * base_fee + tip` (как в geth) | #25258 |
| Поведение | `trace_filter` | Буферизация replay блоков | #25133 |
| Поведение | capability retention | Сериализуется как quantity | #25324 |
| Конфиг | `--engine.legacy-state-root` | Скрыт, только warning; использовать `--engine.state-root-fallback` | #26069 |
| Engine API | SSZ payload/forkchoice containers | Добавлены | #26133, #26271, #25422 |
| Engine API | `flashbots_validateBuilderSubmissionV6` | Валидация BAL | #24297, #25268 |

### 5.2. v2.5.0 (12.08.2026)

| Тип | Метод / элемент | Изменение | PR |
|---|---|---|---|
| Добавлено | `eth_getMultiProof` | Консолидированные EIP-1186 proof для нескольких account/storage целей | #26555 |
| Добавлено | `debug_getRawBlockAccessList` | RLP-encoded EIP-7928 BAL | #26438 |
| Исправление | `testing_buildBlockV1` | Учитывает `targetGasLimit` | #26632 |
| Поведение | `eth_simulate` | Ограничение неявных gas-лимитов | #26502 |
| Формат ошибок | debug trace | Выровнены коды ошибок | #26479 |
| Добавлено | network log conversion | Конверсия log-ответов для downstream-сетей | #26491 |
| Добавлено | execution-witness | Любой `BlockId` | #26572 |
| Добавлено | `reth-rpc-api-testing-util` | **Удалён** (Rust crate, не JSON-RPC) | #26393 |

### 5.3. v2.6.0 (17.09.2026)

| Тип | Метод / элемент | Изменение | PR |
|---|---|---|---|
| Добавлено | `debug_traceChain` (subscription) | Geth-совместимое последовательное трассирование | #26582 |
| **Breaking** | `eth_getMultiProof` | Всегда возвращает **proof V2**, включая child branch nodes после extensions. Сигнатура (2 аргумента) не менялась | #26851 |
| Добавлено | `engine_newPayloadV6` | Bogota/Amsterdam | #26686, #26691, #26706, #26711 |
| Добавлено | `engine_forkchoiceUpdatedV5` | Bogota/Amsterdam | #26686, #26691, #26706, #26711 |
| Добавлено | `engine_getInclusionListV1` | FOCIL | #26682, #26737 |
| Добавлено | `eth_config` | Amsterdam system contracts | #27173, #26696, #26705 |
| Правка поведения | `eth_getLogs` | При очищенных receipts возвращается **ошибка** вместо неполных результатов | #27002 |
| Правка поведения | `eth_estimateGas` | Basic-transfer shortcut возвращает фактически использованный газ (фикс переоценки до +75% после Amsterdam) | #26875 |
| Правка формата | `eth_callBundle` | Base fee следующего блока | #27265 |
| Правка поведения | `mev_simBundle` | Симуляция в блоке после `parentBlock` | #27266 |
| Правка поведения | `eth_*` nonce | `null` для nonce-gaps, потреблённых EIP-7702 auth | #27275 |
| Правка безопасности | `eth_*` chainId | Caller-supplied `chainId`, не совпадающий с нодой, **отклоняется** | #26782 |
| Перф | `eth_getLogs`, `trace_get`, fee-history | Оконные сканы, bloom-фильтрация, ленивая конверсия, shared entries | #26946, #27000, #27031, #26942 |
| Перф | trace/state RPC | Переиспользование RocksDB snapshots/итераторов | #27210 |
| Конфиг | `--rpc-cache.cache-computed-bals`, `--rpc-cache.prewarm-bals[=BLOCKS]` | Кэширование/прогрев BAL | #24097 |
| Конфиг | `--http.compression` | Allowlist алгоритмов сжатия ответов + декомпрессия запросов | #20277, #26668 |
| Конфиг | `--authrpc.jwtsecret-hex` | Inline JWT secret | #27034 |
| Engine API | REST-SSZ по умолчанию; retrieval payload-body by hash/range; payload-witness endpoint | #26254, #26394, #26406, #27028 |
| Engine API | raw block tx lookup | На authenticated Engine API | #26760 |
| Tracing | EIP-8037 state-gas | С сохранением pre-Amsterdam вывода | #26804 |

### 5.4. v2.7.0 (28.09.2026)

| Тип | Метод / элемент | Изменение | PR |
|---|---|---|---|
| **Breaking** | namespace `testing` | **Требует явного выбора**. `all` больше **не включает** `testing` | #27398 |
| **Breaking** | `trace_callMany` | Без указания блока → дефолт **`latest`** (для pending указывать явно) | #27410 |
| **Breaking** | `trace_filter` | Без границ → дефолт **`latest`** | #27411 |
| **Breaking** | `eth_estimateGas` | Учитывает `--rpc.gascap` | #27384 |
| Правка поведения | `trace_get` | Выбор результата по пути в дереве трасс; повтор несуществующей tx → `null` | #27366, #27364 |
| Правка формата | Otterscan (`ots_*`) | Исправлены block и transaction ответы | #27217 |
| Конфиг | дефолты RPC-кэшей | blocks 5 000→1 000; receipts 2 000→500; лимиты байт + idle eviction | #27447, #27310, #27351, #27329 |
| Конфиг | `--rpc-cache.max-bals-bytes`, `--rpc-cache.idle-timeout` | Новые ограничители BAL-кэша | #27351 |
| Hardfork | `eth_config` transition | Sepolia Amsterdam/Glamsterdam | #27350 |

> **Примечание по группировке PR v2.7.0.** Release notes приводят четыре PR (#27398, #27410, #27411, #27384) для двух пунктов (namespace selection; trace-дефолты; `eth_estimateGas`/gascap). Точное соответствие один-к-одному в тексте не указано — в таблице приведена наиболее вероятная привязка.

---

## 6. Разбор критичных для клиентов изменений (с примерами)

### 6.1. Namespace `testing` исключён из `all` (v2.7.0) — BREAKING

**Было:** `--http.api all` / `--ws.api all` включал `testing`.
**Стало (документация v2.7.0):** `all` включает
`admin,debug,eth,net,trace,txpool,web3,rpc,reth,ots,flashbots,miner,mev`.
`testing` **исключён** и должен указываться явно. IPC по-прежнему даёт все namespace'ы кроме `testing`.

```bash
# Если нужен testing_*:
reth node --http --http.api eth,testing
```

**Действие:**
- Если вы не вызываете `testing_*` — правок не требуется.
- Если вызываете `testing_buildBlockV1`/`testing_*` через endpoint с `all` — добавить `testing` в явный список API на соответствующем транспорте (HTTP/WS). Для IPC поведение — «все кроме testing».

### 6.2. Дефолты `trace_callMany` и `trace_filter` (v2.7.0) — BREAKING

**Было:** при отсутствии блока/границ поведение могло использовать pending/историю.
**Стало:** по умолчанию `latest`.

**Действие:** если запросу нужен **pending state** или **исторический диапазон**, указывайте блок/границы явно.

### 6.3. `eth_estimateGas` и `--rpc.gascap` (v2.7.0)

Если на узле задан `--rpc.gascap`, значения `eth_estimateGas` теперь ограничиваются этим cap'ом. В связке с фиксом v2.6.0 (фактически использованный газ для basic transfer, устранение переоценки до +75% после Amsterdam) оценки могут заметно уменьшиться. **Перепроверьте собственные safety-margin'ы** вокруг `eth_estimateGas`.

### 6.4. `trace_get` (v2.7.0)

Выбор результата — по пути в дереве трасс; повтор несуществующей транзакции → `null` (не ошибка). Клиенты, ожидавшие ошибку, должны обрабатывать `null`.

### 6.5. `eth_getProof` для несуществующих аккаунтов (v2.4.0)

Пустые account/storage proof нормализуются в **пустые массивы** (geth-совместимый EIP-1186). Если парсер клиента ожидал прежний формат — проверить.

### 6.6. `eth_getMultiProof` (v2.5.0 → v2.6.0)

Добавлен в v2.5.0 как консолидированный EIP-1186 proof; в v2.6.0 **всегда возвращает proof V2** (включая child branch nodes после extensions). Сигнатура (2 аргумента) сохранена, изменился формат тела ответа. Обновить десериализацию.

### 6.7. `eth_getLogs` и pruned receipts (v2.6.0)

Запрос логов, чьи receipts уже удалены pruning'ом, теперь возвращает **ошибку**, а не неполный результат. Индексаторы должны либо запрашивать доступный диапазон, либо корректно обрабатывать ошибку. Связанное: v2.5.0 включает номера блоков в ошибки pruned-history.

### 6.8. chainId-проверка (v2.6.0)

Caller-supplied `chainId`, не совпадающий с нодой, **отклоняется** (ранее мог молча переподписать и отправить на другую сеть). Всегда передавайте корректный `chainId`.

### 6.9. Otterscan `ots_*` (v2.7.0)

Исправлены block- и transaction-ответы. Если использовали `ots_getBlockDetails`, `ots_getTransaction*` и обходили баги/workaround'ы — перепроверить.

### 6.10. MEV/builder методы (v2.6.0)

- `eth_callBundle`: base fee берётся из следующего блока.
- `mev_simBundle`: симуляция выполняется в блоке после `parentBlock`.
- Возврат `null` при nonce-gap, потреблённом EIP-7702 authorization.

---

## 7. Матрица «изменение → версия → действие потребителя»

| # | Изменение | Версия | Влияние | Обязательное действие |
|---|---|---|---|---|
| 1 | `testing` вне `all` | 2.7.0 | Тесты/локальная разработка | Указать `testing` явно, если используется |
| 2 | `trace_callMany`/`trace_filter` → `latest` | 2.7.0 | Трассировка | Явно указывать блок/границы |
| 3 | `eth_estimateGas` + gascap | 2.7.0 | Оценка газа | Проверить margin'ы; согласовать gascap с командой узла |
| 4 | `trace_get` fix / `null` | 2.7.0 | Трассировка | Обрабатывать `null` |
| 5 | `ots_*` fix | 2.7.0 | Эксплореры | Перепроверить парсинг |
| 6 | RPC-кэш дефолты ↓ | 2.7.0 | Latency | При необходимости настроить `--rpc-cache.*` |
| 7 | `eth_getMultiProof` V2 | 2.6.0 | Proof-потребители | Обновить десериализацию |
| 8 | `eth_getLogs` pruned → error | 2.6.0 | Индексаторы | Обрабатывать ошибку / ограничить диапазон |
| 9 | chainId mismatch → reject | 2.6.0 | Отправка tx | Всегда корректный `chainId` |
| 10 | partial persistence default | 2.6.0 | Чтение БД вне узла | Отключить masking, если читаете БД напрямую |
| 11 | Engine API V6/V5, FOCIL, REST-SSZ | 2.6.0 | Consensus/builders | Проверить совместимость CL |
| 12 | EIP-8037 state-gas tracing | 2.6.0 | Трассировка | Проверить новые поля вывода |
| 13 | `--http.compression` | 2.6.0 | HTTP-клиенты | Опционально; контракт не меняется |
| 14 | `eth_getProof` empty arrays | 2.4.0 | Proof-потребители | Обновить парсер при необходимости |
| 15 | `eth_sendRawTransaction` local | 2.4.0 | Отправка tx | Учесть семантику local (fee/min priority) |
| 16 | `eth_fillTransaction` fee default | 2.4.0 | Кошельки | Сверить с ожиданиями |
| 17 | fee-history weighting (Amsterdam) | 2.4.0 | Аналитика комиссий | Проверить после хардфорка |
| 18 | `engine_hasBlobs`, `debug_clearTxpool`, `admin_banPeer/unbanPeer` | 2.4.0 | Инфраструктура | Использовать при необходимости |
| 19 | `eth_getMultiProof`, `debug_getRawBlockAccessList` | 2.5.0 | Новые возможности | Доступны для внедрения |
| 20 | Снят `--engine.legacy-state-root` | 2.4.0 | Оператор узла | Убрать флаг / `--engine.state-root-fallback` |
| 21 | Хардфорк Amsterdam/Bogota | 2.6–2.7 | Все | Принимать новые поля; учесть дату Sepolia |

---

## 8. Чек-лист миграции и тестирования для потребителей

**Перед апгрейдом:**
- [ ] Собрать инвентарь используемых RPC-методов и namespace'ов по каждому сервису.
- [ ] Определить, кто использует `testing_*`, `trace_callMany`, `trace_filter`, `eth_estimateGas`, `eth_getProof`, `eth_getMultiProof`, `ots_*`, `eth_getLogs`.
- [ ] Уточнить у команды узла: задан ли `--rpc.gascap`; включён ли partial persistence; какие namespace'ы и на каких транспортах включены (`--http.api`, `--ws.api`, IPC).
- [ ] Уточнить, читает ли какой-либо сервис БД Reth напрямую (не через RPC).

**Изменения в клиентах:**
- [ ] Добавить `testing` в явный список, если он нужен (HTTP/WS).
- [ ] Явно задавать блок/границы в `trace_callMany`/`trace_filter`.
- [ ] Обрабатывать `null` в `trace_get`.
- [ ] Обрабатывать ошибку `eth_getLogs` на pruned-диапазонах.
- [ ] Обновить десериализацию `eth_getProof` (пустые массивы) и `eth_getMultiProof` (V2).
- [ ] Проверить парсинг `ots_*`.
- [ ] Убедиться, что `chainId` всегда корректен.
- [ ] Перепроверить safety-margin'ы после `eth_estimateGas` + gascap.

**Тест-план (на staging-инстансе v2.7.0, желательно с pruning и gascap как в прод):**
- [ ] Snapshot-response diff для каждого используемого метода: v2.3.0 vs v2.7.0 на одинаковых запросах (JSON-структура, типы, nullability, error codes).
- [ ] Нагрузочный тест `eth_getLogs`, `trace_*`, `eth_call` — оценить влияние новых дефолтов кэшей.
- [ ] Проверка поведения на границах: несуществующий аккаунт (`eth_getProof`), отсутствующая tx (`trace_get`), pruned-диапазон (`eth_getLogs`), pending vs latest (`trace_callMany`).
- [ ] Проверка после активации хардфорка: новые поля блоков/транзакций, `eth_config`, изменения fee/estimate.
- [ ] Smoke-тест Engine API (для CL/builder), если применимо.

**Развёртывание:**
- [ ] Canary-инстанс v2.7.0 → переключение части трафика → полный переход.
- [ ] План отката: v2.7.x → v2.6.0 → v2.5.x (по линиям совместимости), а не напрямую на v2.3.0.

---

## 9. Совместимость версий зависимостей (для контекста)

| Версия | revm | alloy-core | alloy-crates | alloy-evm | revm-inspectors |
|---|---|---|---|---|---|
| v2.3.0 | 40.0.3 | 1.6.0 | 2.0.5 | 0.36.0 | 0.40.1 |
| v2.4.0 | 41.0.0 | 1.6.0 | 2.1.1 | 0.37.0 | 0.41.0 |
| v2.5.0 | 42.0.1 | 1.6.1 | 2.3.0 | 0.38.0 | 0.42.0 |

---

## 10. Приложение: ссылки на upstream

- Релизы: <https://github.com/paradigmxyz/reth/releases>
  - v2.3.0: <https://github.com/paradigmxyz/reth/releases/tag/v2.3.0>
  - v2.4.0: <https://github.com/paradigmxyz/reth/releases/tag/v2.4.0>
  - v2.5.0: <https://github.com/paradigmxyz/reth/releases/tag/v2.5.0>
  - v2.6.0: <https://github.com/paradigmxyz/reth/releases/tag/v2.6.0>
  - v2.7.0: <https://github.com/paradigmxyz/reth/releases/tag/v2.7.0>
- Документация JSON-RPC: <https://reth.rs/jsonrpc/intro>
- Update priorities: <https://reth.rs/installation/priorities>

**Ключевые PR, упомянутые в документе:**
`#27398`, `#27410`, `#27411`, `#27384` (v2.7.0 RPC behavior); `#27217` (Otterscan); `#27366`, `#27364` (`trace_get`); `#27351`, `#27447`, `#27310`, `#27329` (кэши); `#27350` (`eth_config`); `#26851` (`eth_getMultiProof` V2); `#27002` (`eth_getLogs` pruned); `#26875` (`eth_estimateGas`); `#26782` (chainId); `#27265`, `#27266`, `#27275` (bundle/mev/simulate); `#26582` (`debug_traceChain`); `#26686`/`#26691`/`#26706`/`#26711` (Engine V6/V5); `#26682`/`#26737` (FOCIL); `#26254` (REST-SSZ); `#25682` (`eth_sendRawTransactionSync`); `#25589` (`engine_hasBlobs`); `#25588` (`debug_clearTxpool`); `#25140` (`admin_banPeer`); `#26555` (`eth_getMultiProof`); `#26438` (`debug_getRawBlockAccessList`); `#25412` (local tx); `#25612`/`#26875` (`eth_estimateGas`); `#24719` (`eth_getProof`); `#25258` (`eth_fillTransaction`); `#26069` (legacy state root); `#27220`/`#26800` (partial persistence).

---

*Документ основан на публичных release notes и документации Reth. Перед принятием решений в проде рекомендуется подтвердить поведение на тестовом инстансе целевой версии.*
