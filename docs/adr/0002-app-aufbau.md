# ADR 0002: Aufbau der App

Stand: 17.10.2026 · Status: angenommen

## Kontext

Mit den Mockups in `docs/mockups/` steht das Ziel der Oberfläche fest. Die App soll so weit wie möglich nutzbar sein, ohne eine einzige Zahl zu erfinden (AGENTS.md Regel 2). Fachlich geprüfte Regeln und Referenzwerte gibt es noch nicht.

## Entscheidungen

- **Bewertung ohne erfundene Grenzwerte.** `rules/` ist leer. Eine Erkrankung ohne geprüfte Regel ergibt „unbekannt“ mit dem Satz, dass es noch keine geprüfte Regel gibt. Bewertet wird schon jetzt:
  - Allergene aus den Daten der Quelle (Fund ergibt „meiden“, Spurenhinweis „mit Maß“), immer mit Datenvollständigkeit,
  - Arztvorgaben, die die Person selbst einträgt (Höchstwert pro Tag).
- **Regelentwürfe** lassen sich in den Einstellungen zum Testen einblenden und sind dann als „Entwurf, nicht fachlich geprüft“ gekennzeichnet.
- **serde im Kern.** `crates/core` darf `serde` nutzen, damit Profile und Lebensmittel ohne doppelte Datentypen gespeichert werden können. `scripts/check-core-purity.sh` ist dafür eine Erlaubnisliste.
- **Netzabruf in der App, Parsen im Import.** Der Adapter-Vertrag aus dem Auftrag ist leicht angepasst: `request()` liefert die URL, die App lädt (reqwest, im Browser über fetch), `parse()` liest die Antwort. So bleibt `crates/ingest` ohne Netz testbar.
- **Open Food Facts zuerst.** Einzige eingebundene Quelle ist Open Food Facts, per Barcode-Nummer, per Namenssuche oder als selbst heruntergeladene JSON-Datei. BLS, Purin-Datenbank und GI-Tabellen folgen, sobald ihr Dateiformat anhand echter Dateien geprüft ist.
- **Speicher.** JSON pro Schlüssel in `localStorage` (Web) oder als Dateien im App-Ordner (Desktop, Android). SQLite für Quelldaten folgt später.
- **Gerätewechsel.** Export als verschlüsselte Datei (`.htx`): Argon2id leitet aus dem Passwort den Schlüssel ab, XChaCha20-Poly1305 verschlüsselt. Enthalten sind Profile, Meine Lebensmittel, gespeicherte Teller, Einstellungen und die Quelldaten dieser Lebensmittel. Im Browser speichert die App die Datei über einen Download-Link, auf dem Desktop im Download-Ordner.
- **Zusatzangaben ohne Urteil.** NOVA-Gruppe, Zusatzstoffe und Zutaten zeigt die Karte unverändert aus der Quelle, ohne Ampel, bis es dafür geprüfte Regeln gibt.
- **Tests ohne Netz.** Die Ende-zu-Ende-Tests beantworten Anfragen an Open Food Facts mit den Fixtures aus `fixtures/open-food-facts/`.
- **Farben im Dunkelmodus.** Die Mockup-Tokens definieren für Dunkel keine hellen Hintergrundflächen (soft). Diese Werte sind ergänzt und in `crates/app/assets/main.css` als „added“ markiert; axe prüft den Kontrast in Hell und Dunkel.
- **Kamera-Scan in Rust.** Die Web-App liest Kamerabilder über `getUserMedia`, verkleinert sie auf 640 px Breite und erkennt EAN-13, EAN-8, UPC-A und UPC-E mit `rxing` im Crate `barcode`. Safari auf dem iPhone hat keine Barcode-Schnittstelle, deshalb läuft die Erkennung in WebAssembly. Nur die erkannte Nummer wird weiterverwendet.
- **Vergleich und Arztfragen ohne Urteil.** Der Vergleich zweier Lebensmittel zeigt Werte der Quelle und die Ampeln je Dimension nebeneinander, wählt aber nicht aus. Die Fragen an den Arzt entstehen aus dem Profil (Widersprüche zwischen Regeln, fehlende Vorgaben, Spurenhinweise); die App beantwortet sie nicht.
- **Quellen-Check.** Ein wöchentlicher Workflow lädt ein Produkt live von Open Food Facts, prüft es mit dem Adapter und fragt die Suchdienste ab. Bei einem Fehler legt er ein Issue an. Die geladenen Daten bleiben im Lauf.

## Noch nicht umgesetzt

- Kamera-Scan in Desktop- und Android-App (in der Web-App vorhanden)
- Referenzwerte (Soll) und damit Lücken und Vorschläge
- automatische Vorschläge für Alternativen (brauchen geprüfte Regeln), Schätzmethoden
- Verschlüsselung des lokalen Speichers selbst; bisher ist nur die Exportdatei verschlüsselt
- Export auf Android (dort fehlt noch ein Ordner, auf den die Person zugreifen kann)
