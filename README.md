# Healthy Teller

Healthy Teller bewertet jedes Lebensmittel gleichzeitig gegen alle Erkrankungen, Unverträglichkeiten und Ziele einer Person und begründet jede Bewertung mit Messwert und Quelle.

Die App stellt keine Diagnose und ersetzt keine ärztliche Beratung oder Ernährungstherapie.

**Stand:** Grundgerüst (Meilenstein 1). Die App startet, hat aber noch keine Funktionen.

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
dx serve --platform web
dx serve --platform desktop

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

## Lizenz

- Code: GPL-3.0-or-later, siehe [LICENSE](LICENSE)
- Regelwerk, Zuordnungstabellen und Texte: CC BY-SA 4.0, siehe [LICENSES/CC-BY-SA-4.0.txt](LICENSES/CC-BY-SA-4.0.txt)
- Fremddaten: nicht im Repo, es gelten die Bedingungen der jeweiligen Quelle
