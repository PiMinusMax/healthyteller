# ADR 0003: Namenssuche bei Open Food Facts

Stand: 17.10.2026 · Status: angenommen

## Kontext

Die App sucht Produkte nach Name oder Marke bei Open Food Facts. Im Oktober 2026 funktioniert keiner der offiziellen Suchwege vollständig:

- Die ältere Volltextsuche `world.openfoodfacts.org/cgi/search.pl` antwortet auf jede Anfrage mit HTTP 503. Die Dokumentation von Open Food Facts rät für neue Anwendungen ohnehin davon ab.
- Der Produktionsindex von Search-a-licious (`search.openfoodfacts.org`), der empfohlenen Suche, antwortet, wird aber seit dem 15.12.2024 nicht mehr aktualisiert ([search-a-licious#469](https://github.com/openfoodfacts/search-a-licious/issues/469)). Außerdem sendet er keinen CORS-Header, die Web-Version kann ihn also nicht nutzen.
- Der Testindex `search.openfoodfacts.net` ist deutlich neuer (neuestes Produkt vom 01.08.2026) und erlaubt Abrufe aus der Web-Version, ist aber ein Testsystem ohne Zusage für Verfügbarkeit.

Die Abfrage eines Produkts per Barcode (`/api/v2/product/{barcode}.json`) funktioniert.

Als Alternative wurde ein eigener Suchdienst erwogen, also ein Fork von Search-a-licious mit eigenem Index. Er würde Regel 4 brechen: Die Suchbegriffe aller Nutzer liefen über einen Server des Projekts statt direkt zur Quelle. Dazu kämen Betrieb und Kosten für Elasticsearch und den täglichen Import des Gesamtexports (rund 13 GB).

## Entscheidungen

- **Reihenfolge der Suchdienste:** Testindex, dann Produktionsindex, dann `cgi/search.pl`. Die App nimmt die erste erfolgreiche Antwort; scheitern alle, zeigt sie den ersten Fehler.
- **Treffer nur als Verweis.** Die Suche liefert nur Barcode, Name und Marke. Wählt man einen Treffer, lädt die App das Produkt vollständig per Barcode. So sind Nährwerte und Allergene so aktuell wie die Produktdatenbank, auch wenn der Suchindex hinterherhinkt.
- **Fehlerstatus statt Formatfehler.** Antwortet ein Dienst mit einem Fehlerstatus wie 503, meldet die App „Open Food Facts antwortet gerade nicht“ und nicht, die Antwort sei unlesbar.
- **Kein eigener Suchdienst.** Korrekturen gehen ans Original. Braucht das einen Fork, ist er ein eigenes Repo außerhalb dieses Projekts, das nur diese Korrekturen enthält und keinen Dienst betreibt (Regel 20).

## Folgen

- Fällt der Testindex aus oder verschwindet er, sucht die App im Produktionsindex weiter; Produkte nach Dezember 2024 fehlen dann in der Suche, sind per Barcode aber erreichbar. Die Web-Version findet in diesem Fall nichts, solange auch `cgi/search.pl` ausfällt.
- Ist der Produktionsindex wieder aktuell, wird der Testindex aus `SEARCH_SERVICES` in `crates/ingest/src/off.rs` entfernt und dieser ADR ein Nachtrag angefügt.
- Pro Suche können bis zu drei Anfragen an Open Food Facts gehen, alle nur mit dem Suchbegriff.
