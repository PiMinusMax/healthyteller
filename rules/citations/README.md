# Quellen der Regeln

Eine YAML-Datei pro zitiertem Werk; der Dateiname ist die `id`, auf die eine Regel unter `sources` verweist. Erfasst werden nur die bibliografischen Angaben und die Fundstelle, keine übernommenen Tabellen.

~~~yaml
id: beispiel-2026
title: Titel des Werks
publisher: Herausgeber oder Zeitschrift
year: 2026
edition: Auflage, Version oder Heft
url: https://…
retrieved: 2026-10-17   # Abrufdatum
locator: Seite, Tabelle oder Abschnitt des Grenzwerts
~~~

Ein Test prüft, dass jede Regel nur bekannte Quellen nennt und jede Quelle URL und Abrufdatum hat.
