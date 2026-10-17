# Testlebensmittel für Regeln

Erfundene Lebensmittel für die Referenzfälle der Regeln (`tests` in `rules/*.yaml`). Ein Test bewertet jedes Testlebensmittel mit der Regel und vergleicht das Ergebnis mit `expect`.

~~~yaml
id: fixture-salty-snack
name: Testsalzgebäck
per_100g:
  salt: { amount: 3, unit: g }
~~~

Die Werte sind erfunden und so gewählt, dass sie innerhalb und außerhalb der Schwelle der Regel liegen.
