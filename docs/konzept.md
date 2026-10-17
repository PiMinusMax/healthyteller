# App-Konzept Healthy Teller

Stand: 16.10.2026

## Ausgangslage und Ziel

Die App bewertet jedes Lebensmittel gleichzeitig gegen alle Erkrankungen, Unverträglichkeiten und Ziele einer Person und begründet jede Bewertung mit Messwert und Quelle.

Nährwert-Apps decken Kalorien, Makros und Vitamine ab. Bei mehreren Diagnosen kommen Dimensionen hinzu, die in keiner gängigen App zusammenlaufen. Die Lebensmittelliste für die Mutter zeigt das gut. Diabetes (Zucker, GI), Gicht (Purin), Ei-Allergie, Leberwerte und Bluthochdruck (Salz) mussten von Hand gegeneinander abgewogen werden. Naturjoghurt ist bei Ei-Allergie unproblematisch, schneidet beim Zucker-Protein-Verhältnis aber deutlich schlechter ab als Magerquark. Solche Zielkonflikte fallen erst auf, wenn alle Dimensionen nebeneinanderstehen.

Zielgruppen:

- Menschen mit mehreren gleichzeitigen Ernährungsauflagen (z. B. Diabetes plus Gicht, Niere plus Bluthochdruck) und ihre Angehörigen
- Gesundheitsbewusste ohne Diagnose, die Risiken wie verarbeitetes Fleisch, Acrylamid oder Zusatzstoffe sehen wollen
- Später Ernährungsberatung und Praxen als Prüfwerkzeug

Abgrenzung: Die App stellt keine Diagnose und ersetzt keine Ernährungstherapie. Sie macht sichtbar, was eine Fachkraft prüfen sollte, und liefert dafür die Belege.

## Funktionsprinzip

Lebensmittel, Portion samt Zubereitung und das Profil gehen gemeinsam in die Bewertung. Das Ergebnis bleibt pro Dimension getrennt, damit kein guter Ballaststoffwert eine Allergenwarnung überdeckt.

> **Diagramm fehlt:** Funktionsprinzip · 3 Eingaben, 1 Bewertung, 4 Ergebnisse (im Markdown-Export nicht enthalten)

Widerspricht eine Regel einer anderen (z. B. Kalium bei Bluthochdruck gegen Kalium bei Niereninsuffizienz), entscheidet die App nicht selbst, sondern gibt den Konflikt als Frage an den Arzt aus.

## Bewertungsdimensionen

Die App prüft zwölf Dimensionen, von denen klassische Nährwert-Apps nur die erste abdecken. Jede Dimension hat eine eigene Kennzahl, eine eigene Datenquelle und eine eigene Evidenzstufe, damit eine gut belegte Warnung (Allergen) nicht gleich laut klingt wie eine schwach belegte (Histamin).

| Dimension | Relevant bei | Kennzahl | Evidenz |
| --- | --- | --- | --- |
| Basisnährwerte | allen | Energie, Makros, Ballaststoffe, Vitamine, Mineralstoffe pro 100 g und in % des Referenzwerts | hoch |
| Blutzuckerwirkung | Diabetes, Prädiabetes | Zucker, verfügbare Kohlenhydrate, KE/BE, GI, glykämische Last pro Portion | hoch (GI streut je Sorte und Zubereitung) |
| Purin / Harnsäure | Gicht, Hyperurikämie, Harnsäuresteine | mg Purin bzw. Harnsäureäquivalent pro 100 g und pro Portion | hoch |
| Niere | Niereninsuffizienz, Dialyse | Kalium, Phosphat, Natrium, Eiweiß, Phosphat-Eiweiß-Quotient (mg Phosphat pro g Eiweiß), Phosphat-Zusatzstoffe | hoch |
| Blutdruck | Hypertonie | Salz/Natrium, Kalium | hoch |
| Allergene | Allergien (z. B. Ei) | 14 deklarationspflichtige Hauptallergene, Spurenhinweise, versteckte Quellen (Nudeln, Panade, Mayonnaise) | hoch, aber individuelle Schwelle |
| Histamin | Histaminintoleranz | Verträglichkeitsstufe 0 bis 3, Liberatoren, Reifegrad | mittel bis niedrig |
| Oxalat | Kalziumoxalat-Steine | mg Oxalat pro Portion | mittel (Werte streuen stark zwischen Datenbanken) |
| Verarbeitung | allen | NOVA-Gruppe 1 bis 4, Anzahl und Art der Zusatzstoffe, Nutri-Score | mittel |
| Krebsrelevante Kategorien | allen, besonders bei Darmkrebs in der Familie | verarbeitetes Fleisch (IARC Gruppe 1, je 50 g täglich rund 18 % höheres Darmkrebsrisiko), rotes Fleisch (Gruppe 2A), Pökelware mit Nitrit | hoch für verarbeitetes Fleisch |
| Zubereitung | allen | Garmethode als Faktor: Braten und Grillen von Muskelfleisch bildet heterozyklische Amine (HCA) und PAK, Räuchern PAK, starkes Erhitzen stärkehaltiger Lebensmittel Acrylamid, Pökelware plus Hitze Nitrosamine | mittel (Mechanismus belegt, Effekt beim Menschen nicht abschließend) |
| Wechselwirkung mit Medikamenten | Medikation | Regel pro Wirkstoffgruppe, z. B. Gerinnungshemmer und Vitamin K, Grapefruit und bestimmte Statine | hoch, aber nur für kuratierte Paare |

Zwei Punkte verhindern Fehlalarme. Erstens wird Nitrat aus Gemüse anders behandelt als zugesetztes Nitrit in Pökelware, denn die Krebsbewertung der IARC bezieht sich auf verarbeitetes Fleisch. Zweitens zählt die Zubereitung: dieselbe Hähnchenbrust bekommt gedünstet eine andere Bewertung als scharf gegrillt. Das [NCI-Factsheet](https://www.cancer.gov/about-cancer/causes-prevention/risk/diet/cooked-meats-fact-sheet) nennt offene Flamme, heiße Metallflächen und lange Garzeiten als Haupttreiber für HCA und PAK; die Einstufung von verarbeitetem und rotem Fleisch stammt aus der [IARC-Bewertung](https://www.tabledebates.org/node/10397).

FODMAP (Reizdarm) fehlt bewusst in der Tabelle. Die maßgebliche Datenbasis ist proprietär, eine offene Alternative gibt es nicht.

## Öffentliche Datenquellen

Seit dem 16.12.2025 ist der Bundeslebensmittelschlüssel kostenlos. Damit lässt sich der Kern der App erstmals vollständig aus öffentlichen Quellen speisen: BLS für generische Lebensmittel, Open Food Facts für Markenprodukte, USDA für Lücken. Die Spezialdimensionen (GI, Oxalat, Histamin) hängen an Fachpublikationen, deren Nachnutzung einzeln geklärt werden muss.

| Quelle | Inhalt | Umfang | Lizenz / Zugang | Rolle in der App |
| --- | --- | --- | --- | --- |
| [BLS 4.0 (Max Rubner-Institut)](https://blsdb.de/) | Deutsche Referenz-Nährwerte inkl. Gerichte | 7.140 Lebensmittel, 138 Nährstoffe | kostenfrei seit Dez. 2025, laut FAQ frei nutzbar auch für Apps; die App lädt die Datei selbst, keine Weitergabe nötig | Primärquelle für generische Lebensmittel und die vollständige Nährwertliste |
| [Open Food Facts](https://openfoodfacts.github.io/documentation/docs/Product-Opener/api/tutorials/license-be-on-the-legal-side/) | Markenprodukte per Barcode: Zutaten, Allergene, Spuren, Zusatzstoffe, NOVA, Nutri-Score | rund 3 Mio. Produkte, tägliche Exporte (JSONL, CSV) | ODbL: Namensnennung, abgeleitete Datenbank unter gleicher Lizenz | Barcode-Scan, Allergene, Verarbeitung, Zusatzstoffe |
| [USDA FoodData Central](https://fdc.nal.usda.gov/api-guide) | US-Referenzdaten, Laboranalysen, Markenprodukte | REST-API, 1.000 Anfragen pro Stunde und IP | Public Domain | Lückenfüller (Aminosäuren, seltene Mikronährstoffe) |
| [USDA/ODS-NIH Purin-Datenbank](https://library.mcmaster.ca/data/usda-and-ods-nih-database-purine-content-foods) | vier Purinbasen und Harnsäure | 462 Lebensmittel, 15 alkoholische Getränke | US Open Data | Gicht-Dimension |
| [CIQUAL (ANSES)](https://www.anses.fr/en/content/ciqual-nutritional-composition-table) | Französische Referenz, Einzelzuckerprofile | 3.484 Lebensmittel, 74 Bestandteile | Open Data, Quellenangabe Pflicht | Plausibilitätsabgleich, Zuckerprofile |
| [EFSA EU FCDB](https://www.efsa.europa.eu/bg/data-report/food-composition) | Harmonisierte EU-Daten mit FoodEx2-Codes | rund 28.000 Lebensmittel aus mindestens 16 nationalen Datenbanken | Open Access | Mapping-Rückgrat über FoodEx2, Länderabgleich |
| [EFSA DRV-Finder](https://www.ifst.org/node/8295) | Referenzwerte nach Alter, Geschlecht, Lebensphase | Makros, 14 Vitamine, 13 Mineralstoffe | öffentlich | Ergänzung zu DGE/ÖGE und Höchstmengen (UL) |
| [GI-Tabellen 2021, Atkinson et al.](https://nutrition.org/ajcn-publishes-international-tables-of-glycemic-index-and-glycemic-load-values-2021-a-systematic-review/) | GI und glykämische Last | über 4.000 Einträge, davon rund 2.100 nach ISO-Methode | Fachartikel, Nachnutzung klären | Blutzucker-Dimension |
| Harvard-Oxalattabelle | Oxalat pro Portion | 536 Lebensmittel (Flüssigchromatographie) | nur als Datei, keine Schnittstelle; Anfrage zur Bereitstellung als Datenbank | Oxalat-Dimension |
| [SIGHI-Liste](https://histaminintoleranz.ch/en/downloads.html) | Histamin-Verträglichkeit 0 bis 3 | Positivliste | teils passwortgeschützt, Erlaubnis einholen | Histamin-Dimension |
| [FooDrugs (IMDEA)](https://zenodo.org/records/7874825) | Lebensmittel-Arzneimittel-Wechselwirkungen aus Literatur | 6.422 Interaktionen, 1.464 Stoffe, 3.009 Arzneimittel | Zenodo, Lizenz prüfen | Kandidatenliste für kuratierte Regeln |
| [FooDB](https://foodb.ca/) | Sekundäre Pflanzenstoffe, Inhaltsstoffe | rund 28.000 Stoffe in rund 1.000 Lebensmitteln | CC BY-NC 4.0, nur nicht-kommerziell | entfällt, weil NC nicht zu einem offenen Datensatz passt |

Quellen widersprechen sich, und die App muss das zeigen statt es zu glätten. Beim Oxalat [fand eine Vergleichsstudie](https://urologytimes.com/view/discordance-seen-reporting-food-oxalate-values), dass bei einer Grenze von 90 mg pro Tag fast 40 % der Patienten je nach Datenbank falsch eingeordnet würden. Jeder Wert trägt deshalb Quelle, Methode und Streuung mit sich.

## Datenmodell und Architektur

Die App holt sich die Daten selbst von den Originalquellen und speichert sie lokal. Auf GitHub liegen nur Code, Regelwerk und Zuordnungstabellen; fremde Datenbestände werden weder gespeichert noch weiterverteilt. Diagnosen und Arztvorgaben verlassen das Gerät nie.

> **Diagramm fehlt:** Architektur · Daten direkt von den Originalquellen, GitHub nur für Code und Regeln (im Markdown-Export nicht enthalten)

Die schwierigste Stelle ist das Matching. "Hähnchenbrust, gegrillt" aus dem BLS, ein Markenprodukt aus Open Food Facts und der Purinwert aus der USDA-Tabelle müssen als dasselbe Lebensmittel erkannt werden. Gemeinsamer Schlüssel ist der FoodEx2-Code der EFSA, ergänzt um unscharfe Namenssuche; jede Verknüpfung speichert eine Trefferquote, und unsichere Treffer werden von Hand bestätigt. Die bestätigten Zuordnungen liegen als Tabelle im Repo und enthalten nur die IDs der Quellen, keine fremden Werte; jedes Gerät stellt die Verknüpfung damit selbst her.

Kerntabellen der lokalen Datenbank (SQLite auf dem Gerät):

| Tabelle | Inhalt |
| --- | --- |
| lebensmittel | ID, Name, FoodEx2-Code, Kategorie, Typ (generisch, Marke, Gericht) |
| naehrwert | Lebensmittel, Stoff, Wert, Einheit, Bezug (100 g roh oder zubereitet), Quelle, Methode, Anzahl Proben, Min, Max |
| verknuepfung | Lebensmittel A, Lebensmittel B, Art (gleich, ähnlich, Zutat von), Trefferquote, bestätigt von |
| zubereitung | Garmethode, Verlust- und Bildungsfaktoren (z. B. Vitaminverlust, HCA-Bildung) |
| quelle | Name, Version, Abrufdatum, Lizenz, Pflichthinweis |

Import, Matching, Regel-Engine und App sind durchgehend in Rust geschrieben (Details im nächsten Abschnitt). Die lokale Datenbank ist eine SQLite-Datei auf dem Gerät; jede Quelle lässt sich einzeln aktualisieren, und die App zeigt pro Quelle den Stand an.

### Datenbezug auf dem Gerät

Jeder Import-Adapter kann zwei Wege: automatisch von der Originalquelle laden oder eine Datei übernehmen, die die Person selbst heruntergeladen hat. Der zweite Weg ist Pflicht, weil manche Quellen keinen stabilen Direktlink haben und die Web-App wegen der Sicherheitsregeln im Browser (CORS) nicht jede fremde Seite abrufen darf.

| Quelle | Bezug durch die App | Aktualisierung | Hürde |
| --- | --- | --- | --- |
| BLS 4.0 | Excel-Datei von blsdb.de, automatisch oder per Datei-Import | bei neuer BLS-Version | Datenseite arbeitet mit Sitzungstoken; ohne stabilen Link wählt die Person die geladene Datei aus |
| Open Food Facts | API-Abfrage pro Barcode, Ergebnis im lokalen Cache | bei jedem Scan, Cache mit Ablaufdatum | Gesamtexport (rund 9 GB unkomprimiert) passt nicht auf ein Telefon und wird nicht genutzt |
| USDA FoodData Central | REST-API mit eigenem kostenlosen Schlüssel oder CSV-Download | Foundation Foods zweimal jährlich | Schlüssel beantragt die Person selbst; der Demo-Schlüssel reicht nur zum Testen |
| USDA/ODS-NIH Purin-DB | Datendatei herunterladen | selten | keine |
| CIQUAL | Open-Data-Download | jährlich | keine |
| GI-Tabellen 2021 | Supplement-Tabellen als Datei importieren | statisch | Tabellen liegen als PDF vor, ein Parser ist nötig; Nachnutzung klären |
| Harvard-Oxalat | Datei-Import | selten | keine Datenbank, keine Schnittstelle (siehe unten) |
| SIGHI | erst nach Freigabe |  | passwortgeschützt |

Für Oxalat gibt es keine maschinenlesbare Datenbank. Der Adapter liest deshalb die von Harvard veröffentlichte Datei und erzeugt daraus lokal eine Datenbank; die erzeugte Datei steht in der .gitignore und landet nie im Repo. Parallel geht eine Anfrage an das Harvard Department of Nutrition, die Tabelle als offene, maschinenlesbare Datenbank auf GitHub bereitzustellen oder dem Projekt das unter offener Lizenz zu erlauben. Entwurf:

```text
Subject: Request for the oxalate food table as an open, machine-readable dataset

Dear Department of Nutrition team,

I am starting an open source app that helps people who have to follow several
dietary restrictions at once, for example kidney stones combined with diabetes
or gout. The app evaluates foods against public reference data and runs entirely
on the user's device without collecting any personal data.

Your oxalate table is the most reliable source for oxalate content that we know
of. At the moment it is only available as a downloadable file, which makes
regular automated updates difficult and invites copying errors.

Would you consider publishing the table as a machine-readable dataset, for
example as a CSV file in a public GitHub repository with a stated open license
such as CC BY 4.0? If you prefer not to host it yourselves, we would be glad to
maintain such a repository with clear attribution, but only with your written
permission.

Our project does not redistribute your data. Each user loads the file directly
from your website.

Thank you for considering this request. I am happy to share more details.

Kind regards,
[Name]
[Link to the project repository]
```

## Open Source, Sprache und Plattformen

Das Projekt entsteht vollständig offen auf GitHub: Code, Regelwerk und Zuordnungstabellen. Fremde Datenbestände gehören nicht dazu, die lädt jede Installation selbst von den Originalquellen. Kern und Oberfläche werden in Rust geschrieben; ein Code-Stand ergibt Android-, Linux-, Windows-, macOS- und Web-Version, und iOS wird ohne Apple-Lizenz über die Web-App erreicht.

### Warum Rust

| Sprache | Zielplattformen | Speichersicherheit | Eignung für KI-gestützte Entwicklung | Fazit |
| --- | --- | --- | --- | --- |
| Rust | Android, iOS, Linux, Windows, macOS und Web (WebAssembly) aus einer Codebasis | ja, ohne Garbage Collector | sehr gut: der strenge Compiler meldet Fehler sofort und genau, ein KI-Agent kann selbst nachbessern | Empfehlung |
| C++ | alle, aber eigenes Build-Setup je Plattform (CMake, NDK, Xcode) | nein | mittel: fehlerhafter Code kompiliert oft und fällt erst zur Laufzeit auf | zu viel Plattformaufwand |
| Zig | sehr gute Cross-Kompilierung | teilweise | schwach: Sprache ändert sich noch zwischen Versionen, kleines Mobil-Ökosystem | zu früh |
| Go | Kern über gomobile auf Android und iOS | ja, mit Garbage Collector | gut | Oberflächen auf Mobilgeräten schwach |

Für die Oberfläche passt [Dioxus 0.7](https://docs.rs/dioxus/=0.7.7/): eine Rust-Codebasis für Web, Desktop, Android und iOS, gestaltet mit HTML und CSS, lizenziert unter MIT oder Apache-2.0. Weil Dioxus jünger ist als etwa Flutter, bleibt der Kern frei von jeder UI-Abhängigkeit; über UniFFI ließe er sich später auch an native Kotlin- oder Swift-Oberflächen anbinden. Die Alternativen haben je einen Haken. [Tauri 2](https://v2.tauri.app/blog/tauri-20/) braucht ein Web-Frontend in JavaScript, also eine zweite Sprache. [Slint](https://lib.rs/crates/slint-build) ist unter GPLv3 für Open Source frei, nennt iOS aber noch als offen.

Entwickelt wird wie bei PhotonEx im Dev Container mit VS Code und Claude Code. Der Container bringt die Rust-Toolchain, das Android-NDK und das WebAssembly-Ziel mit. Eine AGENTS.md im Repo (die CLAUDE.md verweist darauf) hält die Projektregeln fest: Kern ohne UI- und Netzabhängigkeiten, jede Regel mit Testfällen, jede Zahl mit Quelle. Strenger Compiler plus Testfälle erlauben es einem KI-Agenten, Änderungen selbst zu prüfen, bevor ein Mensch sie freigibt.

### Plattformen und Verteilung

| Plattform | Weg | Kosten und Voraussetzung |
| --- | --- | --- |
| Web | WebAssembly-Build auf GitHub Pages, installierbar als PWA | kostenlos |
| iOS | dieselbe Web-App über Safari, "Zum Home-Bildschirm" | kostenlos; eine native iOS-App bräuchte das kostenpflichtige Apple-Entwicklerprogramm und einen Mac, das kann später ein Mitwirkender mit eigenem Konto übernehmen |
| Android | APK in GitHub Releases, danach [F-Droid](https://www.f-droid.org/en/docs/Inclusion_Policy/) (verlangt freie Lizenz und Build aus dem Quellcode) | kostenlos; Google Play optional gegen einmalige Gebühr |
| Linux | AppImage in GitHub Releases, Flatpak über Flathub | kostenlos |
| Windows | Installer in GitHub Releases | kostenlos; ohne Codesignatur zeigt Windows eine Warnung |
| macOS | App in GitHub Releases, unsigniert | kostenlos; ohne Apple-Entwicklerprogramm warnt macOS beim ersten Start |

Die Barcode-Erkennung sitzt im Rust-Kern statt in einer Browser-Schnittstelle, damit der Scan auch in der Web-App auf dem iPhone funktioniert. Nur der erkannte Code geht an die Open-Food-Facts-API.

### Lizenzen des Projekts

| Bestandteil | Lizenz | Grund |
| --- | --- | --- |
| Code | GPL-3.0-or-later (bestätigt) | Abwandlungen bleiben offen; passt zu F-Droid; Dioxus (MIT/Apache) ist damit vereinbar |
| Regelwerk, Zuordnungstabellen, Texte | CC BY-SA 4.0 | Fachleute dürfen übernehmen und verbessern, Änderungen bleiben offen |
| Fremddaten (BLS, Open Food Facts, USDA usw.) | nicht im Repo | jede Installation lädt sie von der Quelle; es gelten die Bedingungen der jeweiligen Quelle, weiterverteilt wird nichts |
| FooDB | nicht genutzt | CC BY-NC verträgt sich nicht mit einem frei nutzbaren Projekt |

Weil nichts weiterverteilt wird, entfällt die Frage, ob BLS-Rohwerte in einem öffentlichen Paket stehen dürfen; die [BLS-FAQ](https://blsdb.de/faq) erlaubt die freie Nutzung ausdrücklich auch für App-Entwicklung. Die App zeigt zu jeder Quelle Lizenz, Version, Abrufdatum und Zitierhinweis an, beim BLS die DOI.

### Repository und Automatisierung

```text
<repo>/
├── crates/
│   ├── core/        Datenmodell, Regel-Engine, Bewertung (ohne UI, Netz, Dateisystem)
│   ├── ingest/      Import-Adapter je Quelle, Normalisierung, Zuordnung
│   ├── storage/     lokale Datenbank (SQLite nativ, OPFS im Browser), Verschlüsselung
│   ├── barcode/     Barcode-Erkennung, läuft auch als WebAssembly
│   └── app/         Dioxus-Oberfläche für Web, Desktop, Android, iOS
├── rules/           Regelwerk als YAML, eine Datei pro Regel
├── mappings/        Zuordnungstabellen, nur IDs der Quellen
├── estimates/       Schätzrezepte, nur Anteile und Quell-IDs
├── references/      Referenzwerte (DGE/ÖGE, EFSA), zitiert je Wert
├── sources/         Steckbrief je Quelle: URL, Format, Lizenz, Zitierhinweis
├── fixtures/        kleine Testdateien im Originalformat, Werte erfunden
├── e2e/             Playwright-Tests für Oberfläche und Barrierefreiheit
├── docs/            Konzept und Architekturentscheidungen (ADR)
├── data/            lokal erzeugte Daten, per .gitignore ausgeschlossen
├── AGENTS.md        Projektregeln für KI-Agenten
├── CLAUDE.md        verweist auf AGENTS.md
└── .github/workflows/
```

| Workflow | Auslöser | Ergebnis |
| --- | --- | --- |
| Prüfung | jeder Push und Pull Request | Formatierung, Linter, Lizenzprüfung der Abhängigkeiten, Funktions-, Frontend- und Barrierefreiheitstests; Merge nur bei Grün |
| Repo-Wächter | jeder Push | bricht ab, wenn Dateien aus data/ oder große Datendateien im Commit landen |
| Android-Test | Pull Request auf main, vor jedem Release | App im Android-Emulator auf Linux |
| Quellen-Check | wöchentlich | lädt jede Quelle testweise und meldet geänderte Formate als Issue; veröffentlicht nichts |
| Release | Versions-Tag | APK, AppImage, Windows-Installer, macOS-App, Web-App auf GitHub Pages |

Weil keine Fremddaten im Repo liegen, laufen alle Pflichttests gegen kleine Fixture-Dateien im Originalformat der Quellen, mit erfundenen Werten. Nur der wöchentliche Quellen-Check lädt echte Daten, nutzt sie ausschließlich im Testlauf und veröffentlicht nichts davon.

Testprofile im Repo sind erfunden und ohne Personenbezug. Echte Profile, auch das der Mutter, liegen nur auf dem jeweiligen Gerät.

### Tests und Qualität

Alles lässt sich auf Linux testen. Android läuft im Emulator, iOS wird über die WebKit-Engine abgedeckt, auf der Safari aufbaut; nur eine native iOS-App bräuchte einen Mac.

| Testart | Werkzeug | Was geprüft wird | Pflicht für Merge |
| --- | --- | --- | --- |
| Funktionstests Kern | cargo test, proptest | Bewertung, Tagesbudget, Einheiten, Grenzfälle | ja |
| Regel-Referenzfälle | Testprofile mit erwarteter Ampel je Lebensmittel | jede Regel mit mindestens einem Fall innerhalb und einem außerhalb der Schwelle | ja |
| Import-Adapter | Fixture-Dateien im Originalformat | Einlesen, Einheiten, fehlende Werte werden nicht zu Null | ja |
| Frontend, Ende-zu-Ende | Playwright gegen den Web-Build in Chromium, Firefox und WebKit | Profil anlegen, Lebensmittel prüfen, Teller bauen, Druckliste erzeugen | ja |
| Barrierefreiheit | axe-core in Playwright, dazu Tastatur- und Zoomtests | WCAG 2.2 AA ohne Verstöße, Bedienung ohne Maus, 200 % Zoom ohne abgeschnittene Inhalte | ja |
| Visuelle Regression | Playwright-Screenshots | ungewollte Layoutänderungen in Hell- und Dunkelmodus | ja |
| Android | Emulator in GitHub Actions auf Linux | Start, Scan-Ablauf mit Testbild, Offline-Betrieb | vor Release |
| Live-Quellen | wöchentlicher Quellen-Check | Formate der Originalquellen unverändert | nein, erzeugt ein Issue |

Playwright bringt einen [WebKit-Build für Linux mit, der Safari-Verhalten nachbildet](https://playwright.bootcss.com/docs/why-playwright); Unterschiede gibt es vor allem bei Medien-Codecs, für Layout und Logik reicht Linux. Der [Android Emulator Runner](https://github.com/ReactiveCircus/android-emulator-runner) startet hardwarebeschleunigte Emulatoren auf Linux-Runnern von GitHub; lokal geht dasselbe mit dem Android-SDK-Emulator und KVM.

## Funktionen und Screens

Im Zentrum steht die Lebensmittelkarte: Sie zeigt nur die Dimensionen, die für das aktive Profil zählen, jeweils mit Ampel, Wert pro Portion und einem Satz Begründung samt Quelle.

| Funktion | Was sie leistet | Phase |
| --- | --- | --- |
| Profil-Assistent | Bedingungen, Allergien, Medikamentengruppen, Alter und Geschlecht für die Referenzwerte; Arztvorgaben als eigene Grenzwerte; mehrere Profile (z. B. eigenes und das der Mutter) | MVP |
| Meine Lebensmittel | eigene Liste typischer Lebensmittel mit üblicher Portion; sie stehen in der Suche zuerst und als Liste unter dem Suchfeld | MVP |
| Lebensmittel-Check | Suche oder Barcode, Portion per Regler, Zubereitung wählbar (roh, gekocht, gebraten, gegrillt, geräuchert); Karte mit Ampel je relevanter Dimension | MVP |
| Vollständige Nährwertliste | alle verfügbaren Nährstoffe pro Portion, gruppiert, mit Anteil am persönlichen Referenzwert und Herkunft je Wert | MVP |
| Teller und Tagesbudget | Mahlzeiten zusammenstellen; je Stoff Sollwert und Höchstwert über den Tag, Reihenfolge je nach Fokus (Erkrankungen oder Nährstoffversorgung) | MVP |
| Druckliste | erzeugt aus dem Profil automatisch eine Positiv- und Meideliste mit Werten, so wie die bisher von Hand gebaute Liste für die Mutter | MVP |
| Hinweise | Grundhinweis beim ersten Start, Kennzeichnung von Schätzungen, Arzt-Hinweis bei Krankheitsprofilen (siehe Hinweise in der App) | MVP |
| Schätzungen | fehlende Werte nach dokumentierter Methode schätzen, klar gekennzeichnet, als Liste exportierbar | MVP (Übernahme vom BLS-Gegenstück), Ausbau (Berechnung aus Zutaten) |
| Vorschläge | Lückenanalyse aus Meine Lebensmittel und den Tellern; passende Ergänzungen, auch beim Hinzufügen neuer Lebensmittel | Ausbau |
| Rezept-Analyse | Rezept einfügen, Zutaten werden erkannt und mit Zubereitung bewertet, inklusive versteckter Allergene | Ausbau |
| Konfliktbericht für den Arzt | PDF mit allen Zielkonflikten zwischen den Profilen und offenen Fragen für den nächsten Termin | Ausbau |
| Quellenansicht | zu jedem Wert Quelle, Herkunft, Streuung und abweichende Werte anderer Datenbanken | MVP (einfach), Ausbau (Vergleich) |

Formulierungen folgen einer festen Regel: Die App sagt nie "eifrei" oder "sicher", sondern "keine Ei-Zutat in den Daten gefunden, Spurenhinweis: nein, Datenvollständigkeit 80 %". So bleibt sichtbar, worauf die Aussage beruht.

Die Optik soll so klar sein, dass auch die Mutter die App ohne Erklärung bedienen kann:

- Startbildschirm mit nur zwei Aktionen, Lebensmittel prüfen und Teller ansehen
- Ampel immer mit Symbol und Wort (gut, mit Maß, meiden), nie nur Farbe
- große Schrift, hoher Kontrast, Hell- und Dunkelmodus, großzügige Touch-Flächen
- einfache Sprache; Fachbegriffe wie Purin oder glykämische Last erklären sich beim Antippen
- die erste Ansicht zeigt nur Urteil und Grund, Detailwerte und Quellen erst auf Wunsch
- kein Konto, keine Werbung, kein Tracking; Profil und Daten bleiben lokal, ein Gerätewechsel läuft über eine verschlüsselte Exportdatei

### Teller mit Soll- und Höchstwerten

Der Teller zeigt für jeden Stoff, wie viel bisher am Tag zusammenkommt, und vergleicht das mit zwei Marken: dem Sollwert (was der Mensch zu sich nehmen sollte) und dem Höchstwert (was er höchstens zu sich nehmen sollte).

| Stoffart | Sollwert | Höchstwert | Beispiel |
| --- | --- | --- | --- |
| Nährstoff mit Bedarf und Obergrenze | Referenzwert DGE/ÖGE | EFSA-Höchstmenge (UL) | Jod, Vitamin D |
| Nährstoff nur mit Bedarf | Referenzwert DGE/ÖGE | keiner | Ballaststoffe |
| Belastungsstoff aus einer Regel | keiner | Tagesbudget aus Regel oder Arztvorgabe | Purin, Salz, Zucker |

Jeder Balken hat drei Zonen: unter dem Soll (Lücke), im Zielbereich und über dem Höchstwert. Bereits gespeicherte Teller des Tages erscheinen blass, der aktuelle Teller kräftig, so sieht man vor dem Speichern, was er verändert. Ein Tippen auf eine Marke zeigt, woher der Wert stammt (DGE/ÖGE, EFSA, Regel oder Arztvorgabe).

### Fokus

Die Übersicht auf dem Teller ordnet sich nach dem Fokus. Er wird beim Anlegen des Profils automatisch gesetzt und lässt sich auf dem Teller jederzeit umschalten.

|  | Fokus Erkrankungen | Fokus Nährstoffversorgung |
| --- | --- | --- |
| Automatisch gewählt, wenn | das Profil mindestens eine Erkrankung enthält | das Profil keine Erkrankung enthält |
| Oben | die Kennzahlen der aktiven Regeln, etwa Purin-Budget, Zucker und glykämische Last, Salz, Kalium | Deckung der Sollwerte je Gruppe (Vitamine, Mineralstoffe, Ballaststoffe, Eiweiß, Fettsäuren) mit hervorgehobenen Lücken |
| Darunter, eingeklappt | Nährstoffversorgung | Grenzwerte für Purin, Salz, Zucker und Co. |
| Vorschläge zielen auf | Alternativen, die Budgets entlasten | Lebensmittel, die Lücken schließen |

Unabhängig vom Fokus stehen zwei Dinge immer ganz oben: ein gefundenes Allergen des Profils und jeder überschrittene Höchstwert. Der Fokus ändert nur die Reihenfolge, nie, ob eine Warnung sichtbar ist.

### Meine Lebensmittel

Wer sich gesund ernähren will, isst meist immer wieder dieselben Dinge. Diese typischen Lebensmittel legt man einmal mit der üblichen Portion an, etwa „Vollkorn-Knäckebrot, 2 Scheiben“ oder „Magerquark, 150 g“. Sie erscheinen bei jeder Suche als erste Treffer und, solange das Suchfeld leer ist, als Liste direkt darunter; ein Tippen legt sie mit der gespeicherten Portion auf den Teller. Aus mehreren Einträgen lassen sich feste Teller speichern, zum Beispiel ein typisches Frühstück. Die Liste liegt wie das Profil nur auf dem Gerät.

### Vorschläge

Die Vorschläge vergleichen, was die eigenen Lebensmittel liefern, mit dem persönlichen Bedarf und suchen gezielt nach Ergänzungen.

1. Bedarf bestimmen: Referenzwerte nach Alter und Geschlecht, DGE/ÖGE als Standard und EFSA als Ergänzung (siehe unten); Arztvorgaben ersetzen sie.
2. Lücken finden: Aus Meine Lebensmittel mit üblichen Portionen und den gespeicherten Tellern der letzten Tage berechnet die App die Deckung je Nährstoff. Was deutlich unter dem Referenzwert liegt, gilt als Lücke; die Schwelle ist einstellbar.
3. Kandidaten filtern: nur Lebensmittel, die kein Allergen des Profils enthalten und keine Regel rot verletzen.
4. Sortieren: nach Beitrag zur Lücke pro üblicher Portion, abzüglich neuer Belastungen für Purin, Salz, Zucker und andere Budgets. Lebensmittel aus bereits genutzten Kategorien kommen nach vorn, damit die Vorschläge alltagstauglich bleiben.
5. Begründen: Jeder Vorschlag nennt die Lücke, den Beitrag einer Portion und die Quelle.

Kommt ein neues Lebensmittel auf die Liste, zeigt die App sofort, welche Lücken es schließt, welche Budgets es belastet und welche vorhandenen Lebensmittel es gut ergänzen. Vorschläge sind einzelne Anregungen und kein Speiseplan; bei aktivem Krankheitsprofil steht daneben der Hinweis auf die Absprache mit dem Arzt. Fehlen für einen Nährstoff zu viele Werte, sagt die App das, statt eine Lücke zu behaupten.

**Referenzwerte.** Referenzwerte sind keine Lebensmittellisten, sondern sagen, wie viel ein gesunder Mensch von einem Nährstoff braucht. Was in den Lebensmitteln steckt, kommt weiter aus BLS, USDA und CIQUAL. Ergänzen lassen sich die beiden Quellen deshalb pro Nährstoff, nicht pro Lebensmittel.

| Aufgabe | Quelle | Grund |
| --- | --- | --- |
| Bedarf (Standard) | [DGE/ÖGE-Referenzwerte, 3. Auflage 2025](https://www.dge.de/fileadmin/dok/wissenschaft/referenzwerte/DGE_Referenzwerte_Erlaeuterungen_zur_3._Auflage_2025.pdf) | deutscher Standard, mit dem Ärzte und Ernährungsfachkräfte hierzulande arbeiten; 2025 komplett überarbeitet |
| Bedarf (Lücken) | [EFSA-Referenzwerte (DRV)](https://www.ifst.org/node/8295) | springt ein, wo DGE/ÖGE für einen Nährstoff oder eine Altersgruppe keinen Wert hat |
| Höchstmengen | [EFSA-Übersicht der tolerierbaren Höchstmengen (UL)](https://www.efsa.europa.eu/sites/default/files/2024-05/ul-summary-report.pdf), Stand August 2025 | warnt vor Überversorgung, etwa durch angereicherte Lebensmittel |

Die beiden Systeme nähern sich an. Beim Jod gilt nach der Überarbeitung 2025 für Erwachsene einheitlich [150 µg pro Tag statt bisher 200 bzw. 180 µg](https://www.aerzteblatt.de/themen/ernaehrung/niedrigere-referenzwerte-fur-die-jodzufuhr-f6df0d1e-d831-41d8-a0d1-5123261f8472), und das BfR beschreibt die neuen DGE/ÖGE-Werte als an der EFSA orientiert. Pro Nährstoff gilt immer genau ein Wert; die App mischt nie stillschweigend zwei Quellen und zeigt an, woher der Wert stammt. In den Einstellungen lässt sich EFSA als Standard wählen. Weil Referenzwerte [nur für Gesunde gelten](https://www.dge.de/fileadmin/dok/gesunde-ernaehrung/faq/DGE-FAQ-Referenzwerte_allgemein-2018.pdf), ersetzen bei einem Krankheitsprofil die Arztvorgaben sie, und der Arzt-Hinweis erscheint.

Die Referenzwerte werden wie die Schwellen im Regelwerk gepflegt, als zitierte Einzelwerte mit Quelle, Ausgabe und Altersgruppe unter references/. Die DGE vertreibt die vollständigen Tabellen als kostenpflichtige Loseblattsammlung; vor der Übernahme sind ihre Nutzungsbedingungen zu prüfen.

### Vollständige Nährwertliste und Schätzungen

Für die vollständige Nährwertliste braucht es keine eigene Erhebung. Der [BLS 4.0](https://blsdb.de/faq) liefert pro Lebensmittel 138 Nährstoffe in zwölf Gruppen: Energie, Makronährstoffe, Kohlenhydrate und Zucker, Ballaststoff-Fraktionen, Einzelfettsäuren mit Omega-3 und Omega-6, Aminosäuren, die Vitamine A, D, E, K, C und alle B-Vitamine, Mineralstoffe und Spurenelemente wie Eisen, Zink und Jod sowie Cholesterin. Zu jedem Wert nennt er die Herkunft (Analyse, Rezeptberechnung, Literatur, übernommener Wert und weitere) und unterscheidet fehlende Werte von echten Nullen. Was dort fehlt, etwa Selen, ergänzen USDA FoodData Central und CIQUAL.

Die Liste zeigt jeden Nährstoff pro Portion und als Anteil am persönlichen Referenzwert, gruppiert und einklappbar. Neben jedem Wert steht seine Herkunft:

| Kennzeichen | Bedeutung | Beispiel |
| --- | --- | --- |
| gemessen | Laboranalyse aus der Quelle | BLS-Herkunft „Analyse“ |
| von der Quelle berechnet | die Quelle hat den Wert selbst abgeleitet | BLS-Rezeptberechnung oder übernommener Wert |
| ≈ Schätzung der App | von der App nach dokumentierter Methode geschätzt | Vitamine eines Markenprodukts vom BLS-Gegenstück übernommen |
| unbekannt | kein Wert in irgendeiner Quelle | zählt nie als 0 |

Die App schätzt nur dort, wo keine Quelle einen Wert hat, und nur nach diesen Methoden in dieser Rangfolge:

1. Übernahme vom zugeordneten generischen Lebensmittel. Ein Markenprodukt aus Open Food Facts nennt meist nur die Pflichtangaben; fehlende Vitamine und Mineralstoffe kommen vom passenden BLS-Eintrag, Etikettenwerte haben immer Vorrang.
2. Berechnung aus Zutaten. Nennt das Produkt Zutaten mit Anteilen, rechnet die App wie der BLS bei Rezepten aus den Einzelzutaten hoch.
3. Schätzrezepte des Projekts. Für Lebensmittel, die in keiner Quelle stehen, hinterlegt das Projekt geprüfte Schätzrezepte im Repo. Sie enthalten nur Anteile und Quell-IDs, keine Werte; gerechnet wird auf dem Gerät.

Jede Schätzung trägt das Zeichen ≈ und nennt ihre Grundlage, zum Beispiel „geschätzt anhand des BLS-Eintrags Gouda, 45 % Fett i. Tr.“. Alle geschätzten Werte stehen zusätzlich in einer eigenen Liste, die sich filtern und als CSV exportieren lässt. Liegt ein geschätzter Wert nahe an einer Schwelle, sagt die Ampel das ausdrücklich.

## Umsetzung in Phasen

Der erste echte Test ist die bestehende Liste für die Mutter: Erzeugt die App sie aus dem Profil automatisch und hält sie einer fachlichen Prüfung stand, trägt das Konzept. Erst danach kommen weitere Profile und eine öffentliche Version.

> **Diagramm fehlt:** Umsetzung · 4 Phasen, 3 Gates, ohne Termine (im Markdown-Export nicht enthalten)

Die Phasen sind nach Inhalt geordnet, nicht nach Datum; Termine stehen noch nicht fest. Jede Phase endet an einem Gate, dessen Kriterium erfüllt sein muss, bevor die nächste beginnt.

## Recht, Risiken und Grenzen

Die größte Weiche ist die Zweckbestimmung. Software mit medizinischem Zweck (Diagnose, Überwachung, Behandlung einer Krankheit) gilt in der EU als Medizinprodukt; reine Wellness-Apps nicht. [Laut einer Einordnung zur MDR](https://www.chino.io/post/what-mdr-class-is-my-software) kann dieselbe Ernährungs-App als Wellness-Produkt durchgehen oder, sobald sie der Behandlung von Diabetes dient, Klasse I oder IIa erreichen. Für Software gilt dabei Regel 11 der MDR, die viele Gesundheits-Apps höher einstuft als früher. Das ist keine Rechtsberatung; vor einer Veröffentlichung gehört eine regulatorische Prüfung dazu.

| Thema | Risiko | Gegenmaßnahme |
| --- | --- | --- |
| Medizinprodukt (MDR) | App wird durch Formulierungen oder Funktionen zum Medizinprodukt, z. B. Insulin- oder Dosisberechnung | Zweckbestimmung als Informationswerkzeug, keine Therapieentscheidungen, keine Dosisberechnung; Arztvorgaben nur übernehmen, nicht festlegen |
| Gesundheitsdaten (DSGVO Art. 9) | Diagnosen sind besonders geschützte Daten, Profile von Angehörigen betreffen Dritte | Profile nur lokal und verschlüsselt auf dem Gerät, kein eigener Server, ausdrückliche Einwilligung, Einwilligung der Mutter für ihr Profil |
| Lizenzen | ODbL verlangt Namensnennung und Weitergabe abgeleiteter Datenbanken unter gleicher Lizenz; FooDB nur nicht-kommerziell; SIGHI, GI-Tabellen und Harvard-Oxalat ungeklärt | Lizenz pro Quelle im Datenmodell speichern, Quellen mit unklarer Lizenz erst nach schriftlicher Freigabe einbinden |
| Datenqualität | [Open Food Facts garantiert weder Richtigkeit noch Vollständigkeit](https://tostracker.app/document/openfoodfacts/version/965112), Spezialtabellen widersprechen sich | Vollständigkeit und Quelle sichtbar machen, Allergen-Aussagen nie als Garantie formulieren |
| Fachliche Fehler | eine falsche Regel betrifft alle Nutzer mit diesem Profil | Vier-Augen-Freigabe durch Fachkraft, Regeln versioniert, Änderungen nachvollziehbar |

Grenzen, die die App offen benennt: individuelle Verträglichkeit (Histamin, Allergieschwellen) lässt sich aus Tabellen nicht ableiten, Laborwerte ändern sich, und ein Grenzwert ohne aktuelle Arztvorgabe bleibt ein Leitlinien-Durchschnitt.

## Hinweise in der App

Die Hinweise sind Teil der Funktion und lassen sich nicht abschalten. Sie sind kurz, in einfacher Sprache und dort sichtbar, wo sie gebraucht werden. Damit die App ein Informationswerkzeug bleibt (siehe Recht, Risiken und Grenzen), bleiben auch die Vorschläge bewusst allgemeine Anregungen.

| Hinweis | Wo er erscheint |
| --- | --- |
| Grundhinweis | beim ersten Start mit Bestätigung, dauerhaft unter „Über die App“, auf jeder Druckliste |
| Absprache mit dem Arzt | bei Vorschlägen, Druckliste und Konfliktbericht, sobald ein Krankheitsprofil aktiv ist |
| Schwankende Zusammensetzung | in der Nährwertliste und der Quellenansicht |
| Schätzungen | an jedem geschätzten Wert und über der Schätzliste |
| Allergene | auf jeder Karte, wenn das Profil eine Allergie enthält |
| Datenstand | in der Quellenansicht und auf der Druckliste |

Textentwürfe (Anrede in der App: Sie):

**Grundhinweis.** Healthy Teller hilft bei der Suche nach passenden Lebensmitteln und beim Erkennen von Unverträglichkeiten. Die App stellt keine Diagnose und ersetzt keine ärztliche Beratung. Alle Angaben stammen aus öffentlichen Datenquellen oder sind als Schätzung gekennzeichnet. Eine Garantie für ihre Richtigkeit und Vollständigkeit gibt es nicht.

**Absprache mit dem Arzt.** Bei einer Erkrankung sollten Sie Ernährungspläne und Änderungen Ihrer Ernährung immer mit Ihrer behandelnden Ärztin oder Ihrem behandelnden Arzt absprechen. Vorschläge der App sind Anregungen und kein Behandlungsplan.

**Schwankende Zusammensetzung.** Lebensmittel schwanken in ihrer Zusammensetzung. Sorte, Reife, Anbau, Lagerung, Rezeptur und Zubereitung verändern die Werte. Die angezeigten Zahlen sind Durchschnittswerte und können vom Lebensmittel auf Ihrem Teller abweichen.

**Schätzungen.** Werte mit dem Zeichen ≈ sind geschätzt. Bei jedem dieser Werte steht, worauf die Schätzung beruht, zum Beispiel auf einem vergleichbaren Lebensmittel aus dem Bundeslebensmittelschlüssel oder auf der Zutatenliste. Schätzungen sind ungenauer als gemessene Werte.

**Allergene.** Findet die App kein Allergen, heißt das nicht, dass das Lebensmittel frei davon ist. Maßgeblich sind die Zutatenliste und die Allergenkennzeichnung auf der Verpackung. Bei Anzeichen einer schweren allergischen Reaktion wählen Sie sofort den Notruf 112.

**Datenstand.** Die Werte stammen von {Quelle}, Stand {Datum}. Für die Richtigkeit der Datenquellen übernimmt Healthy Teller keine Gewähr. Eine Haftung für Fehler in den Quelldaten ist ausgeschlossen, soweit das gesetzlich zulässig ist.

## Name

Entschieden ist **Healthy Teller**, als Repo `healthyteller`. Die Mischung aus Englisch und Deutsch ist in beiden Sprachen verständlich und sagt direkt, worum es geht. Marken-, Domain- und Store-Prüfung stehen noch aus; bis dahin bleiben die übrigen Vorschläge als Ausweichnamen.

| Name | Repo | Idee |
| --- | --- | --- |
| Klarteller | klarteller | klare Auskunft über den Teller |
| Tellerblick | tellerblick | ein Blick, alle Dimensionen |
| Speiselotse | speiselotse | lotst durch mehrere Einschränkungen gleichzeitig |
| Nährwerk | naehrwerk | Nährwerte plus Regelwerk |
| Zutatenwache | zutatenwache | Schwerpunkt auf versteckten Allergenen und Zusatzstoffen |
| ClearPlate | clearplate | englische Variante für internationale Mitwirkende |

## Projektregeln

Diese Regeln stehen wörtlich in der AGENTS.md und gelten für Menschen wie für KI-Agenten.

1. Keine Fremddaten im Repo. Erlaubt sind Code, Regelwerk, Zuordnungstabellen mit IDs, Schätzrezepte mit Anteilen und IDs sowie Fixtures mit erfundenen Werten; data/ steht in der .gitignore, der Repo-Wächter prüft jeden Push.
2. Keine erfundenen Zahlen. Jeder Nährwert und jeder Grenzwert hat Quelle, Version und Abrufdatum. KI-Agenten tragen nie Werte aus dem Gedächtnis ein; fehlt eine Quelle, entsteht ein Issue.
3. Schätzungen nur nach dokumentierter Methode aus Quellwerten. Jede Schätzung trägt das Zeichen ≈, nennt ihre Grundlage und steht in der Schätzliste.
4. Gesundheitsdaten bleiben auf dem Gerät. Kein Konto, kein eigener Server, keine Analyse- oder Absturzdienste Dritter; Netzverkehr nur zu den Originalquellen.
5. Fehlende Werte sind keine Null. Unbekannt bleibt unbekannt und wird so angezeigt.
6. Kein Urteil ohne Begründung. Jede Ampel und jeder Vorschlag zeigt Wert pro Portion, einen Satz Grund und die Quelle.
7. Keine Gesamtnote, keine Diagnose, keine Dosisberechnung, keine Speisepläne für Erkrankungen. Konflikte zwischen Profilen werden angezeigt, nicht entschieden.
8. Vorsichtige Sprache. Nie "sicher" oder "frei von", sondern "in den Daten gefunden" oder "nicht gefunden", jeweils mit Datenvollständigkeit.
9. Der Kern bleibt rein. crates/core kennt weder Oberfläche noch Netz noch Dateisystem.
10. Merge nur bei Grün: Formatierung, Linter ohne Warnungen, Lizenzprüfung, Funktions-, Frontend- und Barrierefreiheitstests (WCAG 2.2 AA ohne Verstöße).
11. Jede Regel hat Referenzfälle innerhalb und außerhalb der Schwelle und wird erst nach fachlicher Freigabe ausgeliefert.
12. Jede Quelle hat einen Adapter, den Datei-Import als Rückfallweg, eine Fixture und einen Steckbrief mit Lizenz und Zitierhinweis.
13. Barrierefreiheit von Anfang an: nie nur Farbe, volle Tastaturbedienung, Screenreader-Beschriftungen, 200 % Zoom, einfache Sprache.
14. Alles ist auf Linux bau- und testbar; kein Schritt setzt einen Mac voraus.
15. Abhängigkeiten nur mit GPL-kompatibler Lizenz und ohne proprietäre SDKs, damit die App F-Droid-tauglich bleibt.
16. Code, Kommentare und Commits auf Englisch; Oberflächentexte nur über Übersetzungsdateien, Deutsch zuerst.
17. Kleine Pull Requests mit Conventional Commits; Architekturentscheidungen als ADR in docs/adr/.
18. Testprofile sind erfunden. Echte Personen- oder Gesundheitsdaten erscheinen nie in Code, Tests, Issues oder Logs.
19. Die Pflichthinweise (Grundhinweis, Absprache mit dem Arzt, schwankende Zusammensetzung, Schätzungen, Allergene, Datenstand) sind Teil der Oberfläche, lassen sich nicht abschalten und werden in den Ende-zu-Ende-Tests geprüft.

## Offene Entscheidungen

- [x] Öffentliches Open-Source-Projekt auf GitHub, alles frei verfügbar.
- [x] Nicht kommerziell; FooDB entfällt deshalb.
- [x] MVP mit den fünf Profilen aus der Liste der Mutter (Diabetes, Gicht, Ei-Allergie, Leberwerte, Bluthochdruck).
- [x] Plattform: Rust-Kern mit Dioxus-Oberfläche; iOS über die Web-App ohne Apple-Lizenz.
- [x] Code-Lizenz GPL-3.0-or-later.
- [x] Fremddaten nicht im Repo; die App lädt sie selbst von den Originalquellen.
- [x] Name Healthy Teller, Repo healthyteller.
- [x] Anrede in der App: Sie.
- [x] Referenzwerte: DGE/ÖGE als Standard, EFSA für Lücken und Höchstmengen.
- [ ] Marken-, Domain- und Store-Prüfung für Healthy Teller, danach Repo anlegen.
- [ ] Nutzungsbedingungen der DGE für die Übernahme einzelner Referenzwerte prüfen.
- [ ] Anfrage an Harvard zur Oxalat-Tabelle senden (Entwurf im Abschnitt Datenbezug auf dem Gerät).
- [ ] Nachnutzung bei SIGHI und den Autoren der GI-Tabellen anfragen.
- [ ] Prüfen, ob blsdb.de einen stabilen Direktlink zur Datendatei anbietet.
- [ ] Wer gibt Regeln fachlich frei (Ernährungsfachkraft, Hausärztin der Mutter)?

## Prompt für KI-Agenten

Der folgende Prompt ist der Startauftrag für Claude Code oder andere Agenten. Er gehört als docs/auftrag.md ins Repo; die AGENTS.md enthält die Projektregeln von oben wörtlich, die CLAUDE.md besteht nur aus dem Verweis @AGENTS.md. Im Dev Container startet der Agent wie gewohnt mit `IS_SANDBOX=1 claude --dangerously-skip-permissions`, das Repo entsteht vorher mit `gh repo create healthyteller --public --license gpl-3.0`.

```markdown
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
```
