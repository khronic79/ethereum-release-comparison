# Reth JSON-RPC: схема для валидирующего прокси — результаты и план

| Параметр | Значение |
|---|---|
| Скоуп | namespace `eth`, `net`, `web3`, `txpool` |
| Валидация | запросы + ответы |
| Формат | JSON Schema Draft 2020-12, один документ |
| Режим | strict (`additionalProperties: false`) |
| Версии Reth | v2.3.0 → v2.7.0 |
| Источники | код Reth (теги `v2.3.0`, `v2.7.0`); `ethereum/execution-apis` (OpenRPC/YAML) |

Документ описывает **фактически выполненную** работу (инвентаризация из кода + генерация схем + проверка на примерах спеки), а не пересказ release notes.

---

## 1. Инвентарь методов (из кода, не из релиз-ноутов)

Методы извлечены из трейтов `#[rpc(namespace = ...)]` в `crates/rpc/rpc-api` и `crates/rpc/rpc-eth-api` обоих тегов.

| namespace | v2.3.0 | v2.7.0 |
|---|---|---|
| eth | 76 | 77 |
| net | 3 | 3 |
| web3 | 2 | 2 |
| txpool | 4 | 4 |
| **Итого** | **85** | **86** |

**Единственное добавление между версиями в этом скоупе:** `eth_getMultiProof` (v2.7.0). Остальные различия — поведенческие/форматные (см. `docs/reth-v2.3.0-vs-v2.7.0-rpc-comparison.md`).

## 2. Покрытие официальной спекой

Сопоставление с методами, реально объявленными в `execution-apis` (`src/**/*.yaml`):

| namespace | всего | есть в спеке | Reth-only |
|---|---|---|---|
| eth | 77 | 44 | 33 |
| net | 3 | 3 | 0 |
| web3 | 2 | 0 | 2 |
| txpool | 4 | 3 | 1 |
| **Итого** | **86** | **50 (58%)** | **36 (42%)** |

**Reth-only методы (36), требуют авторской схемы:**

```
eth_callBundle, eth_callMany, eth_cancelBundle, eth_cancelPrivateTransaction,
eth_getAccount, eth_getAccountInfo, eth_getBlockAccessListByBlockHash,
eth_getBlockAccessListByBlockNumber, eth_getBlockAccessListRaw,
eth_getHeaderByHash, eth_getHeaderByNumber, eth_getMultiProof (v2.7.0+),
eth_getRawTransactionByBlockHashAndIndex, eth_getRawTransactionByBlockNumberAndIndex,
eth_getRawTransactionByHash, eth_getTransactionBySenderAndNonce,
eth_getUncleByBlockHashAndIndex, eth_getUncleByBlockNumberAndIndex,
eth_getUncleCountByBlockHash, eth_getUncleCountByBlockNumber, eth_getWork,
eth_hashrate, eth_mining, eth_pendingTransactions, eth_protocolVersion,
eth_sendBundle, eth_sendPrivateRawTransaction, eth_sendPrivateTransaction,
eth_sendRawTransactionConditional, eth_sendRawTransactionSync, eth_signTypedData,
eth_submitHashrate, eth_submitWork,
web3_clientVersion, web3_sha3, txpool_inspect
```

Машиночитаемая матрица: `schemas/coverage.json`.

---

## 3. Что уже сгенерировано

| Файл | Содержимое |
|---|---|
| `schemas/reth-eth-net-txpool.strict.schema.json` | 52 метода (44 eth + subscribe/unsubscribe + 3 net + 3 txpool), strict, refs раскрыты в `$defs` |
| `schemas/reth-eth-net-txpool.loose.schema.json` | то же без `additionalProperties: false` |
| `schemas/coverage.json` | метод → {in_spec, spec_file, reth_only, v2.3.0, v2.7.0} |
| `schemas/method_inventory.json` | полный инвентарь по версиям |

**Конвенция использования в прокси** (в одном документе):

```jsonc
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "methods": {
    "eth_getBalance": {
      "source": "execution-apis:eth/state.yaml",
      "params": { "anyOf": [ <позиционный массив>, <именованный объект> ] },
      "result": { "$ref": "#/$defs/uint" }
    }
  },
  "$defs": { /* все компоненты спеки */ }
}
```

Алгоритм прокси: взять `request.method` → проверить `request.params` по `methods[method].params`, а `response.result` по `methods[method].result`. Ошибки валидировать по `$defs/JSONRPCError`.

Ограничение текущего артефакта: покрыты только 50 стандартных методов; 36 Reth-only ещё не добавлены. Версии v2.3.0/v2.7.0 пока общие (стандартный набор одинаков), отличия зафиксированы в `coverage.json`.

---

## 4. Проверка на примерах официальной спеки (факт)

Схема прогнана против **собственных примеров** `execution-apis` (54 примера). Схема валидна как Draft 2020-12, но примеры проходят не все:

| Режим | Ответы (response) | Запросы (request) |
|---|---|---|
| strict | 20 провалов | 4 провала |
| loose | 13 провалов | 5 провалов |

**Категории провалов (loose):** `oneOf` 6, `anyOf` 5, `pattern` 5, `required` 1, `additionalProperties` 1.

**Разбор причин:**

1. **`pattern` (5) — баг примеров, не схемы.** Примеры хэшей транзакций используют 63 hex-символа вместо 64 (напр. `0xe670ec...7331`). Схема (`^0x[0-9a-f]{64}$`) корректна.
2. **`oneOf` (union-типы) — фундаментальная проблема ответов.** `Transaction`, `txpool_content`, `feeHistory` описаны в спеке как `oneOf` вариантов. Примеры не проходят `oneOf`: варианты не взаимоисключающие (или неполные). Это проявляется и в strict, и в loose → **официальные схемы ответов для union-типов непригодны как строгий валидатор без доработки**.
3. **`anyOf` в запросах (фильтры/`eth_createAccessList`/`eth_subscribe`)** — артефакты примеров: неканоничные фильтр-id (`0x01` вместо `0x1`), отсутствие опционального фильтра у `transactionReceipts`. Схема параметров при этом рабочая.
4. **`additionalProperties` (7) — прямое следствие strict.** Закрытие объектов отвергает поля, которые спека допускает в примерах (вложенные транзакции в txpool).

**Вывод:** strict для **запросов** безопасен; strict для **ответов** даёт ложные срабатывания на union/эволюционирующих типах.

---

## 5. Рекомендация по строгости

Дифференцированная политика вместо единого strict:

| Категория | Политика |
|---|---|
| Параметры запросов | `additionalProperties: false` (strict) — ок |
| Ответы-скаляры и стабильные объекты (балансы, quantity, receipt, log, proof) | strict — ок |
| union/эволюционирующие ответы (`eth_getTransaction*`, `eth_getBlock*`, `txpool_*`, `eth_feeHistory`) | заменить `oneOf` на `anyOf` **или** заменить спеку на один Reth-специфичный объект (Reth отдаёт одну форму) |

Для Reth предпочтительнее второе: у конкретной версии один serde-выход, поэтому точнее описать **один** `Transaction`/`Block`, а не union — это и строже, и без ложных срабатываний.

---

## 6. Дальнейшие шаги

1. **Стандартные методы:** пересобрать union-ответы по Reth-факту (из Rust-типов / реальных ответов), сохранив strict.
2. **Reth-only методы (36):** сгенерировать схемы из сигнатур Rust (типы параметров/результата + `serde`-атрибуты) — начать с наиболее используемых: `eth_getAccount*`, `eth_getStorageValues`, `eth_getHeaderBy*`, `web3_*`, `txpool_inspect`, `eth_getBlockAccessList*`, `eth_sendRawTransactionSync`, `eth_getMultiProof`.
3. **Сверить с живым узлом:** снять реальные ответы с v2.3.0 и v2.7.0, прогнать через схемы, устранить расхождения (это же даст точный diff версий).
4. **Версионирование:** выпускать `reth-v2.3.0.schema.json` и `reth-v2.7.0.schema.json`.

---

## 7. Воспроизводимость

- Reth: `git clone --filter=blob:none --sparse`; sparse-path `crates/rpc/{rpc-api,rpc-eth-api,rpc-builder,rpc-server-types}`; теги `v2.3.0`, `v2.7.0`.
- Спекa: `git clone --depth 1 https://github.com/ethereum/execution-apis`. Официальные YAML-исходники завендорены в `schemas/execution-apis/src/` (коммит и дата — `schemas/execution-apis/UPSTREAM_COMMIT.txt`).
- Генератор: парсит `src/schemas/*.yaml` → `$defs`, методы `src/**/*.yaml` → `methods`; `$ref` `#/components/schemas/X` → `#/$defs/X`.
- Валидация: `jsonschema` (Draft202012Validator) на примерах методов.
