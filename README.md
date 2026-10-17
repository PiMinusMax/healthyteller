# Healthy Teller

Healthy Teller bewertet jedes Lebensmittel gleichzeitig gegen alle Erkrankungen, Unverträglichkeiten und Ziele einer Person und begründet jede Bewertung mit Messwert und Quelle.

Die App stellt keine Diagnose und ersetzt keine ärztliche Beratung oder Ernährungstherapie.

**Stand:** erste nutzbare Version. Profil anlegen, Produkte per Barcode-Nummer aus Open Food Facts laden, Allergene und eigene Arztvorgaben bewerten, Teller und Druckliste. Fachlich geprüfte Regeln und Referenzwerte fehlen noch, deshalb zeigt die App für Erkrankungen meist „unbekannt“. Details: [docs/adr/0002-app-aufbau.md](docs/adr/0002-app-aufbau.md).

## Dokumente

- [docs/konzept.md](docs/konzept.md): Konzept mit Bewertungsdimensionen, Datenquellen, Architektur, Funktionen und Recht
- [docs/auftrag.md](docs/auftrag.md): Startauftrag für KI-Agenten
- [AGENTS.md](AGENTS.md): Projektregeln für Menschen und KI-Agenten

## Grundsätze

- Kein Konto, kein eigener Server, kein Tracking. Gesundheitsdaten bleiben auf dem Gerät.
- Keine Fremddaten im Repo. Jede Installation lädt BLS, Open Food Facts, USDA usw. selbst von der Originalquelle.
- Rust-Kern mit Dioxus-Oberfläche: Web (PWA, auch für iOS), Android, Linux, Windows, macOS.

## Entwicklung

Am einfachsten im Dev Container (`.devcontainer/`), der alle Werkzeuge mitbringt. Ohne Container braucht es Rust (stable), [dx 0.7.10](https://github.com/DioxusLabs/dioxus/releases/tag/v0.7.10), [cargo-deny](https://github.com/EmbarkStudios/cargo-deny), Node 22 und für Desktop-Builds unter Linux `libwebkit2gtk-4.1-dev libgtk-3-dev libxdo-dev`.

```sh
# App starten (im Ordner crates/app)
dx serve --platform web        # öffnet http://127.0.0.1:8080
dx serve --platform desktop
dx serve --platform android    # braucht Android SDK und NDK, Emulator oder Gerät per USB

# Prüfungen wie in der CI
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
scripts/check-core-purity.sh
scripts/repo-guard.sh

# Ende-zu-Ende- und Barrierefreiheitstests gegen den Web-Build
(cd crates/app && dx build --platform web --release)
(cd e2e && npm ci && npx playwright install --with-deps && npx playwright test)
```

Die Ende-zu-Ende-Tests liegen in `e2e/tests/`. `helpers.ts` enthält die Bausteine (Grundhinweis bestätigen, Testprofil anlegen, Fixture-Produkt laden). Anfragen an Open Food Facts werden dort mit den Fixtures beantwortet, die Tests laufen also ohne Netz.

Zum Ausprobieren ohne Netz: Barcode `4000000000017` liefert in den Tests das erfundene Produkt „Testjoghurt natur“. In der echten App fragt die Nummer Open Food Facts ab, dort gibt es sie nicht.

| Ordner | Inhalt |
| --- | --- |
| `crates/core` | Datenmodell, Regel-Engine, Bewertung; ohne Abhängigkeiten |
| `crates/ingest` | Import-Adapter je Quelle, Zuordnung |
| `crates/storage` | lokale Datenbank, verschlüsselte Profile |
| `crates/barcode` | Barcode-Erkennung |
| `crates/app` | Dioxus-Oberfläche für Web, Desktop, Android, iOS |
| `locales/` | Oberflächentexte (Fluent), Deutsch zuerst |
| `rules/`, `mappings/`, `estimates/`, `references/`, `sources/` | Regelwerk und Begleitdaten, nur IDs und Quellenangaben |
| `fixtures/` | Testdateien mit erfundenen Werten |
| `e2e/` | Playwright- und axe-Tests |
| `docs/adr/` | Architekturentscheidungen |
| `docs/mockups/` | Oberflächen-Mockups, Design-Tokens und Schrift |

## Lizenz

- Code: GPL-3.0-or-later, siehe [LICENSE](LICENSE)
- Regelwerk, Zuordnungstabellen und Texte: CC BY-SA 4.0, siehe [LICENSES/CC-BY-SA-4.0.txt](LICENSES/CC-BY-SA-4.0.txt)
- Fremddaten: nicht im Repo, es gelten die Bedingungen der jeweiligen Quelle
