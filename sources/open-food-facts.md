# Open Food Facts

| Feld | Wert |
| --- | --- |
| ID in der App | `off` |
| URL | https://world.openfoodfacts.org |
| Schnittstelle | API v2, ein Produkt je Barcode: `https://world.openfoodfacts.org/api/v2/product/{barcode}.json` |
| Format | JSON, Nährwerte unter `nutriments` als `{name}_100g`, alle außer Energie in Gramm |
| Lizenz | Datenbank: ODbL 1.0, Inhalte: DbCL 1.0, Bilder: CC BY-SA 3.0 |
| Zitierhinweis | Open Food Facts, https://world.openfoodfacts.org, ODbL |
| Adapter | `crates/ingest/src/off.rs` |
| Datei-Import | JSON-Antwort der API für ein Produkt |
| Fixture | `fixtures/open-food-facts/` (erfundene Werte) |

Hinweise:

- Die App fragt nur den Barcode ab und speichert die Antwort im lokalen Cache. Der Gesamtexport wird nicht genutzt.
- Open Food Facts garantiert weder Richtigkeit noch Vollständigkeit. Die App zeigt deshalb Vollständigkeit und Herkunft der Werte an.
- Nährwerte sind Herstellerangaben und tragen in der App das Kennzeichen ∑.
