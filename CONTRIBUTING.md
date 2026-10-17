# Mitmachen bei Healthy Teller

Danke, dass Sie helfen wollen. Healthy Teller ist ein Gesundheitswerkzeug: Ein Fehler in einer Regel betrifft alle Menschen mit diesem Profil. Deshalb gelten ein paar feste Regeln, für Menschen wie für KI-Agenten. Die vollständige Liste steht in [AGENTS.md](AGENTS.md).

## Wege zum Beitrag

### Fehler melden oder Ideen vorschlagen

Legen Sie ein [Issue](../../issues/new/choose) an. Bitte schreiben Sie dazu, auf welchem Gerät und in welchem Browser es passiert ist. **Tragen Sie keine echten Gesundheitsdaten ein**, weder Ihre eigenen noch die von Angehörigen. Für Beispiele reicht ein erfundenes Profil.

### Fachwissen beitragen

Die App bewertet Erkrankungen erst, wenn es dafür eine fachlich geprüfte Regel gibt. Jede Regel ist eine YAML-Datei in `rules/` mit Grenzwert, Quelle und Testfällen (Format in [docs/auftrag.md](docs/auftrag.md#regelwerk)).

So läuft es ab:

1. Vorschlag als Issue mit der Vorlage „Fachliche Angabe“: Erkrankung, Kennzahl, Grenzwert und die Quelle mit Ausgabe und Seite.
2. Daraus entsteht eine Regel mit `status: draft` und Referenzfällen innerhalb und außerhalb der Schwelle. Die Quelle kommt als eigene Datei nach `rules/citations/`, die erfundenen Testlebensmittel der Referenzfälle nach `fixtures/rules/`. `cargo test` bewertet jeden Referenzfall mit der Regel und prüft, dass jede genannte Quelle erfasst ist.
3. Eine Fachkraft prüft die Regel. Erst dann wird sie auf `status: reviewed` gesetzt und ausgeliefert.

Zahlen ohne Quelle nehmen wir nicht an, auch keine „allgemein bekannten“.

### Code beitragen

1. Issue suchen oder anlegen, damit klar ist, was sich ändert.
2. Branch anlegen, Änderung in kleinen Schritten, Test zuerst.
3. Pull Request gegen `main`. Gemergt wird nur, wenn alle Prüfungen grün sind.

## Entwicklungsumgebung

Am einfachsten ist der Dev Container in `.devcontainer/`. Er bringt Rust, dx, cargo-deny, Node, Playwright und das Android SDK mit. Ohne Container siehe [README](README.md#selbst-bauen).

## Vor jedem Commit

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
scripts/check-core-purity.sh
scripts/repo-guard.sh

# Ende-zu-Ende-Tests gegen den Web-Build
(cd crates/app && dx build --platform web --release)
(cd e2e && npm ci && npx playwright install --with-deps && npx playwright test)
```

Die CI führt dieselben Prüfungen aus, dazu den Start der Desktop-App und bei Pull Requests auf `main` einen Test im Android-Emulator.

## Regeln für Beiträge

**Daten**

- Keine Fremddaten im Repo. Daten von BLS, Open Food Facts, USDA usw. lädt jede Installation selbst. Erlaubt sind Code, Regeln, Zuordnungen nur mit IDs und Testdateien mit erfundenen Werten. Der Repo-Wächter lehnt Dateien unter `data/`, Datenbanken, Archive und Dateien über 1 MB ab.
- Keine erfundenen Zahlen. Jeder Nährwert und jeder Grenzwert hat Quelle, Version und Abrufdatum. Fehlt eine Quelle, entsteht ein Issue mit dem Label `fachlich-offen`, kein Platzhalterwert.
- Fehlende Werte sind keine Null. Unbekannt bleibt unbekannt.
- Testprofile sind erfunden. Echte Personen- oder Gesundheitsdaten erscheinen nie in Code, Tests, Issues oder Logs.

**Quellen**

- Die App spricht nur mit den Diensten der Originalquelle, nie mit einem eigenen Server.
- Ist bei einer Quelle etwas kaputt, wird es dort behoben: als Issue oder Pull Request beim Original. Braucht das einen Fork, ist er ein eigenes Repo außerhalb dieses Projekts, enthält nur diese Korrekturen und betreibt keinen eigenen Dienst.
- Ein vorübergehender Ausweichweg, etwa ein Testsystem der Quelle, steht mit Grund und Endbedingung im Steckbrief unter `sources/` und in einer ADR.

**Aussagen der App**

- Kein Urteil ohne Begründung: Wert pro Portion, ein Satz Grund, Quelle.
- Keine Gesamtnote, keine Diagnose, keine Dosisberechnung, keine Speisepläne für Erkrankungen.
- Vorsichtige Sprache: nie „sicher“ oder „frei von“, sondern „in den Daten gefunden“ oder „nicht gefunden“ mit Datenvollständigkeit.
- Die sechs Pflichthinweise stehen im festen Wortlaut aus dem [Konzept](docs/konzept.md#hinweise-in-der-app) und lassen sich nicht abschalten.

**Code**

- `crates/core` bleibt rein: keine Oberfläche, kein Netz, kein Dateisystem. Erlaubt ist nur `serde`.
- Code, Kommentare und Commit-Nachrichten auf Englisch. Oberflächentexte nur über `locales/`, Deutsch zuerst, Anrede „Sie“. Jeder neue Text braucht einen deutschen und einen englischen Eintrag; ein Test prüft das.
- Neue Abhängigkeiten nur mit GPL-kompatibler Lizenz, ohne proprietäre SDKs, mit Begründung im Pull Request. `cargo deny` prüft das.
- Alles muss auf Linux bau- und testbar sein.

**Barrierefreiheit**

- Nie nur Farbe: Ampeln haben immer Form, Symbol und Wort.
- Volle Tastaturbedienung, Beschriftungen für Screenreader, 200 % Zoom ohne abgeschnittene Inhalte.
- axe prüft jeden Screen in Hell und Dunkel gegen WCAG 2.2 AA. Neue Screens kommen in `e2e/tests/accessibility.spec.ts`.

**Pull Requests**

- Klein und mit einem Thema.
- Commit-Nachrichten nach [Conventional Commits](https://www.conventionalcommits.org/de/), zum Beispiel `feat(app): add file import`.
- Architekturentscheidungen als kurze ADR in `docs/adr/`.
- README und `docs/` aktuell halten.

## Neue Version veröffentlichen

1. Versionsnummer in `Cargo.toml` (`[workspace.package] version`) erhöhen und per Pull Request mergen.
2. Auf `main` einen Tag setzen, zum Beispiel `git tag v0.2.0 && git push origin v0.2.0`.
3. Der Workflow `Release` baut APK, AppImage, deb, Windows-Installer und DMG und hängt sie an ein neues Release.

Damit Android-Updates sich über ältere Versionen installieren lassen, braucht das Repository einen festen Schlüssel als Secrets `ANDROID_KEYSTORE_BASE64` und `ANDROID_KEYSTORE_PASSWORD` (Keystore mit Alias `healthyteller`). Ohne sie wird jede Version mit einem Wegwerf-Schlüssel signiert und muss vor einem Update deinstalliert werden.

## Lizenz Ihrer Beiträge

Mit einem Beitrag stimmen Sie zu, dass er unter den Lizenzen des Projekts steht: Code unter GPL-3.0-or-later, Regelwerk, Zuordnungen und Texte unter CC BY-SA 4.0.

## Verhalten

Wir gehen freundlich und sachlich miteinander um. Viele, die hier mitlesen, sind selbst krank oder pflegen Angehörige. Persönliche Angriffe und abwertende Bemerkungen haben hier keinen Platz.
