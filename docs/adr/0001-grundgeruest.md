# ADR 0001: Grundgerüst

Stand: 17.10.2026 · Status: angenommen

## Kontext

Das Konzept legt Rust, Dioxus 0.7 und einen Cargo-Workspace mit den Crates core, ingest, storage, barcode und app fest. Für das Gerüst waren einige Details offen.

## Entscheidungen

- **Crate-Namen mit Präfix.** Die Pakete heißen `healthyteller-core` usw. Ein Paket namens `core` würde mit der Rust-Standardbibliothek `core` kollidieren.
- **Reinheit des Kerns.** `crates/core` hat keine Abhängigkeiten. `scripts/check-core-purity.sh` prüft das in der CI. Braucht der Kern später eine reine Bibliothek (etwa für Einheiten), wird die Prüfung um eine Erlaubnisliste erweitert.
- **Kein unsicherer Code.** `unsafe_code = "forbid"` gilt für den ganzen Workspace.
- **Oberflächentexte über Fluent.** Die App liest ihre Texte aus `locales/<sprache>/app.ftl` (Deutsch zuerst). Ein Test prüft, dass Deutsch und Englisch dieselben Schlüssel haben.
- **dx 0.7.10** baut Web, Desktop und Android. Die Version von dx entspricht der Dioxus-Version im Workspace und ist in CI und Dev Container festgelegt.
- **Lizenzprüfung mit cargo-deny.** Erlaubt sind nur GPL-kompatible Lizenzen und nur crates.io. Ungepflegte Crates sind nur bei direkten Abhängigkeiten ein Fehler; zwei bekannte Meldungen in transitiven Abhängigkeiten von Dioxus Desktop sind mit Begründung in `deny.toml` ausgenommen.
- **Repo-Wächter** (`scripts/repo-guard.sh`) bricht ab bei Dateien unter `data/`, bei Datenbank- und Archivdateien und bei Dateien über 1 MiB, und zwar für jeden Commit eines Pushes, nicht nur für den Endstand.
- **Android-Test** läuft für Pull Requests auf main und vor Releases, nicht bei jedem Push, weil der Emulator-Lauf lange dauert.

## Folgen

- Desktop-Builds auf Linux brauchen `libwebkit2gtk-4.1-dev`, `libgtk-3-dev` und `libxdo-dev`.
- Eine native iOS-App ist nicht Teil des Gerüsts; iOS wird über die Web-App erreicht.
