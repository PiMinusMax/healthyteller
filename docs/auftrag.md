# Auftrag

Du entwickelst Healthy Teller, eine freie Open-Source-App unter
GPL-3.0-or-later. Sie bewertet Lebensmittel gleichzeitig gegen mehrere
Erkrankungen, Allergien und Ziele einer Person und nennt zu jeder Bewertung
Wert, Grund und Quelle. Lies zuerst docs/konzept.md und AGENTS.md. Die Regeln
in AGENTS.md haben Vorrang vor allem anderen in diesem Auftrag.

## Was die App tut
- Eine Person legt lokal ein oder mehrere Profile an: Bedingungen, Allergien,
  Medikamentengruppen und Arztvorgaben als eigene Grenzwerte.
- Sie sucht ein Lebensmittel oder scannt einen Barcode und wählt Portion und
  Zubereitung (roh, gekocht, gebraten, gegrillt, geräuchert).
- Die App zeigt je relevanter Dimension eine Ampel mit Symbol und Wort
  (gut, mit Maß, meiden, unbekannt), den Wert pro Portion, einen Satz
  Begründung und die Quelle.
- Teller: je Stoff Tagesstand gegen Sollwert und Höchstwert (Zonen Lücke,
  Zielbereich, über Höchstwert); gespeicherte Teller blass, aktueller kräftig.
- Fokus pro Profil: Erkrankungen (Regel-Kennzahlen oben, Nährstoffe
  eingeklappt) oder Nährstoffversorgung (Deckung oben, Grenzwerte
  eingeklappt); automatisch nach Profil gesetzt, umschaltbar. Allergene und
  überschrittene Höchstwerte stehen unabhängig vom Fokus immer oben.
- Druckliste: Positiv- und Meideliste mit Werten, automatisch aus dem Profil.
- Zielkonflikte zwischen Profilen werden angezeigt, nie verrechnet.
- Meine Lebensmittel: typische Lebensmittel mit üblicher Portion; sie stehen
  in der Suche zuerst und als Liste unter dem leeren Suchfeld.
- Vollständige Nährwertliste pro Portion mit Anteil am persönlichen
  Referenzwert und Herkunft je Wert (gemessen, von der Quelle berechnet,
  ≈ Schätzung der App, unbekannt).
- Vorschläge: Lücken zwischen den eigenen Lebensmitteln und den
  Referenzwerten finden und Ergänzungen vorschlagen, nur aus Lebensmitteln
  ohne Allergen des Profils und ohne rote Regel, jeweils mit Begründung.

## Was die App nicht tut
- keine Diagnose, keine Therapieempfehlung, keine Dosisberechnung
- keine Speisepläne für Erkrankungen; Vorschläge sind einzelne Anregungen
- keine Gesamtnote über alle Dimensionen
- keine Aussage "sicher" oder "frei von"
- kein Konto, kein eigener Server, kein Tracking, keine Werbung

## Technik (verbindlich)
- Rust stable, Cargo-Workspace mit den Crates:
  - core: Modelle, Einheiten, Regel-Engine, Bewertung; rein, ohne IO
  - ingest: ein Adapter je Quelle, Normalisierung, Zuordnung
  - storage: SQLite nativ, OPFS im Browser, verschlüsseltes Profil
  - barcode: Barcode-Erkennung in Rust, auch als WebAssembly
  - app: Dioxus 0.7 für Web (WASM), Desktop, Android und iOS
- Oberflächentexte als Fluent-Dateien unter locales/, zuerst de, dann en.
- Ende-zu-Ende- und Barrierefreiheitstests: Playwright (TypeScript) in e2e/,
  gegen den Web-Build in Chromium, Firefox und WebKit, mit axe-core.
- Dev Container mit Rust, Ziel wasm32-unknown-unknown, Android SDK und NDK,
  Node für Playwright. Alles baut und testet auf Linux.

## Daten
Fremddaten liegen nie im Repo. Die App lädt sie auf dem Gerät von der
Originalquelle oder übernimmt eine Datei, die die Person ausgewählt hat.
Jeder Adapter implementiert diesen Vertrag:

~~~rust
pub trait SourceAdapter {
    fn meta(&self) -> SourceMeta; // id, name, url, license, citation, version
    fn fetch(&self, ctx: &FetchCtx) -> Result<RawArtifact, IngestError>; // darf NotSupported liefern
    fn import_file(&self, bytes: &[u8]) -> Result<RawArtifact, IngestError>; // Pflicht
    fn parse(&self, raw: &RawArtifact) -> Result<Vec<SourceRecord>, IngestError>;
}
~~~

Jeder gespeicherte Wert trägt: value, unit, basis (pro 100 g essbarer Anteil,
roh oder zubereitet), source_id, source_version, retrieved_at, derivation
(Analyse, Berechnung, Literatur usw.) und, wenn vorhanden, n, min, max.
Ein fehlender Wert ist None, niemals 0.

Schätzungen nur nach diesen Methoden, in dieser Rangfolge, immer mit
estimate=true und basis (Text plus Quell-IDs):
1. Übernahme fehlender Werte vom zugeordneten BLS-Eintrag; Etikettenwerte
   haben Vorrang.
2. Berechnung aus Zutaten mit Anteilen, nach dem Vorgehen der
   BLS-Rezeptberechnung.
3. Schätzrezepte aus estimates/ (nur Anteile und Quell-IDs, keine Werte).
Alle Schätzungen erscheinen in einer filterbaren, als CSV exportierbaren
Liste. Nährstoffe, die im BLS fehlen (z. B. Selen), kommen aus USDA
FoodData Central oder CIQUAL.

Quellen im MVP, in dieser Reihenfolge:
1. BLS 4.0: Excel-Datei, je Nährstoff drei Spalten (Wert, Herkunft,
   Referenz), EuroFIR-Komponentencodes, Werte pro 100 g essbarer Anteil.
2. Open Food Facts API v2 pro Barcode:
   https://world.openfoodfacts.org/api/v2/product/{barcode}.json,
   mit eigenem User-Agent, Ergebnis im lokalen Cache mit Ablaufdatum.
3. USDA/ODS-NIH Purin-Datenbank: Datei-Import.
4. GI-Tabellen nach Atkinson et al. 2021: Datei-Import der Supplement-Tabellen.
5. Referenzwerte: DGE/ÖGE (3. Auflage 2025) als Standard, EFSA-DRV für
   Lücken, EFSA-UL für Höchstmengen. Gepflegt unter references/ als zitierte
   Einzelwerte mit Quelle, Ausgabe und Altersgruppe; je Nährstoff genau ein
   Wert, Quelle in der App sichtbar.
Später: USDA FoodData Central (eigener API-Schlüssel der Person), CIQUAL,
Harvard-Oxalat (Datei-Import, erzeugte Datenbank nur lokal unter data/).

Zuordnungen zwischen Quellen liegen in mappings/ als CSV mit
source_a, id_a, source_b, id_b, relation, confidence, reviewed_by.
Nur IDs, keine Werte.

## Regelwerk
Eine YAML-Datei pro Regel unter rules/. Beispiel für das Format:

~~~yaml
id: gout.purine.daily_budget
version: 1
status: draft            # draft | reviewed; nur reviewed wird ausgeliefert
profile: gout
dimension: purine
metric: purine_mg
basis: day               # portion | day | week
thresholds:              # Werte ausschließlich aus den genannten Quellen
  caution: 300
  avoid: 400
user_overridable: true   # Arztvorgabe ersetzt den Standardwert
evidence: high           # high | medium | low
sources: [kaneko-2014, usda-ods-purine-2022]
message_key: rules-gout-purine-daily-budget
tests:
  - fixture_food: fixture-sardine
    portion_g: 100
    expect: avoid
  - fixture_food: fixture-low-fat-quark
    portion_g: 150
    expect: ok
~~~

Bewertung im core:
- Eingabe: Lebensmittel, Portion in g, Zubereitung, Profile, bisheriger Tag.
- Ausgabe je Dimension: level (ok | caution | avoid | unknown),
  value_per_portion, budget_used, reason_key, sources, data_completeness.
- Ein gefundenes Allergen überstimmt alles andere.
- Ziehen zwei Profile dieselbe Kennzahl in entgegengesetzte Richtungen,
  entsteht ein Conflict-Objekt mit beiden Regeln.
- Zubereitung wirkt über Faktoren: Nährstofferhalt und Hinweise auf
  HCA, PAK, Acrylamid und Nitrosamine.

## Oberfläche
- Startbildschirm mit zwei Aktionen: Lebensmittel prüfen, Teller ansehen.
- Ampel immer mit Symbol und Wort, nie nur Farbe.
- Große Schrift, hoher Kontrast, Hell- und Dunkelmodus.
- Erste Ansicht nur Urteil und Grund; Werte und Quellen auf Wunsch.
- Einfache Sprache; Fachbegriffe erklären sich beim Antippen.
- Anrede in allen Texten: Sie.
- Profilexport als verschlüsselte Datei für den Gerätewechsel.
- Pflichthinweise wörtlich aus docs/konzept.md, Abschnitt "Hinweise in der
  App": Grundhinweis beim ersten Start mit Bestätigung, Arzt-Hinweis bei
  Krankheitsprofilen, Hinweise zu Schwankungen, Schätzungen, Allergenen und
  Datenstand. Nicht abschaltbar, durch Ende-zu-Ende-Tests abgedeckt.

## Arbeitsweise
- Lies AGENTS.md vor jeder Aufgabe neu.
- Arbeite in kleinen, abgeschlossenen Schritten, ein Pull Request pro Schritt,
  Conventional Commits.
- Schreibe zuerst den Test, dann den Code.
- Führe vor jedem Commit aus:
  cargo fmt --check
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace
  cargo deny check
  npx playwright test
- Neue Abhängigkeiten nur mit Begründung im Pull Request und GPL-kompatibler
  Lizenz.
- Fehlt eine fachliche Angabe (Grenzwert, Nährwert, Zuordnung), rate nicht.
  Lege ein Issue mit Label fachlich-offen an und mach mit dem nächsten Schritt
  weiter.
- Halte Architekturentscheidungen als ADR in docs/adr/ fest und halte README
  und docs/ aktuell.

## Meilensteine
1. Gerüst: Workspace, Dev Container, CI mit allen Prüfungen, Repo-Wächter,
   Lizenzdateien, AGENTS.md und CLAUDE.md, leere App startet in allen Zielen.
2. core: Datenmodell, Einheiten, Bewertung einer Dimension, Property-Tests.
3. storage und BLS-Adapter mit Fixture; Quellenansicht mit Stand und Lizenz.
4. Open Food Facts per Barcode samt Cache und barcode-Crate.
5. Purin- und GI-Adapter; Zuordnungen für die 500 häufigsten Lebensmittel
   vorschlagen, ein Mensch bestätigt sie (Gate 1).
6. Regeln für die MVP-Profile Diabetes, Gicht, Ei-Allergie, Leberwerte und
   Bluthochdruck, jeweils mit Referenzfällen und Status draft.
7. Oberfläche: Profil-Assistent, Lebensmittelkarte, Teller und Tagesbudget,
   Druckliste, Meine Lebensmittel, vollständige Nährwertliste mit
   Herkunftskennzeichen, Schätzmethode 1 mit Schätzliste, Pflichthinweise;
   Ende-zu-Ende- und axe-Tests.
8. Pakete: Web-App auf GitHub Pages, APK, AppImage; Android-Emulatortest.
9. Vorschläge mit Lückenanalyse, danach Schätzmethoden 2 und 3.
Gate 2: Die Druckliste eines erfundenen Testprofils mit diesen fünf
Bedingungen wird fachlich geprüft und freigegeben.

## Definition of Done
- Alle Prüfungen sind grün: Format, Linter, Lizenzen, Funktion, Frontend,
  Barrierefreiheit (WCAG 2.2 AA ohne Verstöße).
- Neue Logik hat Tests, neue Regeln haben Referenzfälle, neue Quellen haben
  Adapter, Datei-Import, Fixture und Steckbrief.
- Kein Commit enthält Fremddaten oder echte Personendaten.
- Jede Schätzung ist gekennzeichnet und nennt ihre Grundlage; die
  Pflichthinweise sind sichtbar und getestet.
- Doku ist aktualisiert.
