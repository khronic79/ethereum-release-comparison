# Расписание хардфорков Ethereum (mainnet и Sepolia)

Источник: `ethereum/go-ethereum`, файлы `params/chainspecs/mainnet.json` и `params/chainspecs/sepolia.json`
(`master` @ `32fea34c44eb9f280b2b8bdbbf344c8ff5ef63cf`). После Merge форки активируются по timestamp.

Формат: `дата UTC (unix timestamp)`.

| Форк | mainnet | Sepolia |
|---|---|---|
| Shapella | 2023-04-12 22:27 (1681338455) | 2023-02-28 04:04 (1677557088) |
| Dencun (Cancun) | 2024-03-13 13:55 (1710338135) | 2024-01-30 22:51 (1706655072) |
| Pectra | 2025-05-07 10:05 (1746612311) | 2025-03-05 07:29 (1741159776) |
| **Fusaka (Fulu + Osaka)** | **2025-12-03 21:49 (1764798551)** | 2025-10-14 07:36 (1760427360) |
| BPO1 | 2025-12-09 14:21 (1765290071) | 2025-10-21 03:26 (1761017184) |
| BPO2 | 2026-01-07 01:01 (1767747671) | 2025-10-27 23:16 (1761607008) |
| **Amsterdam** | — (не запланирован) | **2026-10-06 13:53 (1791294816)** |

## Примечания

- **Крайний полный хардфорк mainnet — Fusaka** (Fulu на CL + Osaka на EL), 3 декабря 2025.
- После Fusaka были только **Blob-Parameter-Only (BPO)** форки, меняющие лишь `target`/`max` блобов:
  BPO1 — target 10 / max 15, BPO2 — target 14 / max 21. Последнее по времени сетевое изменение — BPO2 (7 января 2026).
- **Amsterdam/Glamsterdam на mainnet ещё не активирован** — в mainnet-чейнспеке `amsterdamTime` отсутствует.
- На **Sepolia Amsterdam активирован 6 октября 2026** (1791294816), поэтому Reth v2.7.0 требует обновления до этой даты.
- Тестнеты идут впереди mainnet: Sepolia получает форки на неделях–месяцах раньше.

## Как обновить таблицу

```bash
curl -sSL https://raw.githubusercontent.com/ethereum/go-ethereum/master/params/chainspecs/mainnet.json
curl -sSL https://raw.githubusercontent.com/ethereum/go-ethereum/master/params/chainspecs/sepolia.json
```

Поля времени: `shanghaiTime`, `cancunTime`, `pragueTime`, `osakaTime`, `bpo1Time`, `bpo2Time`, `amsterdamTime`
(значение — unix timestamp; `null`/отсутствие = форк не назначен; `0` = форк уже активен с генезиса).
