# Healthy Teller

Healthy Teller bewertet jedes Lebensmittel gleichzeitig gegen alle Erkrankungen, Unverträglichkeiten und Ziele einer Person und begründet jede Bewertung mit Messwert und Quelle.

Die App stellt keine Diagnose und ersetzt keine ärztliche Beratung oder Ernährungstherapie.

**Stand:** Konzeptphase, noch kein Code.

## Dokumente

- [docs/konzept.md](docs/konzept.md): Konzept mit Bewertungsdimensionen, Datenquellen, Architektur, Funktionen und Recht
- [docs/auftrag.md](docs/auftrag.md): Startauftrag für KI-Agenten
- [AGENTS.md](AGENTS.md): Projektregeln für Menschen und KI-Agenten

## Grundsätze

- Kein Konto, kein eigener Server, kein Tracking. Gesundheitsdaten bleiben auf dem Gerät.
- Keine Fremddaten im Repo. Jede Installation lädt BLS, Open Food Facts, USDA usw. selbst von der Originalquelle.
- Rust-Kern mit Dioxus-Oberfläche: Web (PWA, auch für iOS), Android, Linux, Windows, macOS.

## Lizenz

- Code: GPL-3.0-or-later, siehe [LICENSE](LICENSE)
- Regelwerk, Zuordnungstabellen und Texte: CC BY-SA 4.0
- Fremddaten: nicht im Repo, es gelten die Bedingungen der jeweiligen Quelle
