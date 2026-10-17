# Open Food Facts

| Feld | Wert |
| --- | --- |
| ID in der App | `off` |
| URL | https://world.openfoodfacts.org |
| Schnittstelle | API v2, ein Produkt je Barcode: `https://world.openfoodfacts.org/api/v2/product/{barcode}.json` |
| Namenssuche | [Search-a-licious](https://search.openfoodfacts.org/docs), der Reihe nach: Testindex `https://search.openfoodfacts.net/search`, Produktionsindex `https://search.openfoodfacts.org/search`, ältere Suche `https://world.openfoodfacts.org/cgi/search.pl` |
| Format | JSON, Nährwerte unter `nutriments` als `{name}_100g`, alle außer Energie in Gramm |
| Lizenz | Datenbank: ODbL 1.0, Inhalte: DbCL 1.0, Bilder: CC BY-SA 3.0 |
| Zitierhinweis | Open Food Facts, https://world.openfoodfacts.org, ODbL |
| Adapter | `crates/ingest/src/off.rs` |
| Datei-Import | JSON-Antwort der API für ein Produkt |
| Fixture | `fixtures/open-food-facts/` (erfundene Werte) |

Hinweise:

- Die App fragt nur den Barcode oder den Suchbegriff ab und speichert die Antwort im lokalen Cache. Der Gesamtexport wird nicht genutzt.
- Die Namenssuche liefert nur Barcode, Name und Marke. Wählt man einen Treffer, lädt die App das Produkt vollständig per Barcode, weil der Suchindex älter sein kann als die Produktdaten.
- Open Food Facts garantiert weder Richtigkeit noch Vollständigkeit. Die App zeigt deshalb Vollständigkeit und Herkunft der Werte an.
- Nährwerte sind Herstellerangaben und tragen in der App das Kennzeichen ∑.

## Ausweichweg Namenssuche

Stand: 17.10.2026. Begründung in [ADR 0003](../docs/adr/0003-namenssuche.md).

| Dienst | Stand | Abruf aus dem Browser | Rolle |
| --- | --- | --- | --- |
| Testindex `search.openfoodfacts.net` | deutlich neuer, neuestes Produkt vom 01.08.2026 | erlaubt für `piminusmax.github.io` | zuerst, vorübergehend |
| Produktionsindex `search.openfoodfacts.org` | seit 15.12.2024 nicht aktualisiert ([search-a-licious#469](https://github.com/openfoodfacts/search-a-licious/issues/469)) | nicht erlaubt (kein CORS-Header) | Rückfall |
| `cgi/search.pl` | antwortete am 17.10.2026 auf jede Anfrage mit HTTP 503 | erlaubt | letzter Rückfall |

- Der Testindex ist ein Dienst von Open Food Facts ohne Zusage für Verfügbarkeit. Er gibt Marken mit Sprachpräfix zurück (`xx:Marke`); der Adapter entfernt das.
- Endbedingung: Ist der Produktionsindex wieder aktuell, wird der Testindex aus `SEARCH_SERVICES` in `crates/ingest/src/off.rs` entfernt.
- Korrekturen am Original laufen über einen eigenen Fork außerhalb dieses Projekts (AGENTS.md Regel 20), nicht über einen eigenen Suchdienst.
