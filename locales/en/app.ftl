## Healthy Teller, English UI texts.

app-title = Healthy Teller

## Navigation and actions

nav-back = Back
nav-profile-switch = , switch profile { $name }
action-add = Add
action-back = Back
action-cancel = Cancel
action-change = Change
action-continue = Continue
action-create-profile = Create profile
action-edit = Edit
action-evaluate = Evaluate
action-home = Go to start page
action-move-down = Move { $name } down
action-move-up = Move { $name } up
action-remove-named = Remove { $name }
action-save = Save
action-switch = Switch
action-take-over = Use
action-understood = Got it
action-undo = Undo
not-found-title = This page does not exist.

## Mandatory notices (fixed wording)

notice-basic-title = Basic notice.
notice-basic = Healthy Teller helps you find suitable foods and recognise intolerances. The app does not make a diagnosis and does not replace medical advice. All information comes from public data sources or is marked as an estimate. There is no guarantee that it is correct or complete.
notice-doctor-title = Talk to your doctor.
notice-doctor = If you have a medical condition, always discuss diet plans and changes to your diet with your doctor. Suggestions from the app are ideas, not a treatment plan.
notice-variation-title = Varying composition.
notice-variation = The composition of food varies. Variety, ripeness, growing, storage, recipe and preparation change the values. The numbers shown are averages and may differ from the food on your plate.
notice-estimates-title = Estimates.
notice-estimates = Values marked ≈ are estimated. Each of these values states what the estimate is based on, for example a comparable food from the German food composition database or the list of ingredients. Estimates are less precise than measured values.
notice-allergens-title = Allergens.
notice-allergens = If the app finds no allergen, this does not mean the food is free of it. The list of ingredients and the allergen labelling on the package are what counts. If you notice signs of a severe allergic reaction, call the emergency number 112 immediately.
notice-data-status-title = Data status.
notice-data-status = The values come from { $source }, as of { $date }. Healthy Teller does not guarantee that the data sources are correct. Liability for errors in the source data is excluded as far as the law allows.

## 01 Basic notice

welcome-title = Welcome to Healthy Teller
welcome-lead = Please read this notice before you start.
welcome-where = You can find this notice at any time under Settings, About the app.
welcome-confirm = I have read and understood the notice.

## 02 Start

home-checked-for = Checking for
home-no-profile = No profile yet. Create a profile so the app can evaluate food for you.
home-check-title = Check food
home-check-sub = Search or enter a barcode
home-plate-title = View plate
home-plate-empty = Nothing entered today
home-plate-count = { $count } foods entered today
home-local = All data stays on this device.
home-no-diagnosis = No diagnosis. The app shows what a professional should check.

## 03/04 Search

search-title = Check food
search-label = Search food
search-placeholder = Name or brand
search-results = Results
search-no-hits = No results in storage. Search Open Food Facts online below or enter the barcode.
search-mine = My foods
search-local-only = The search only covers foods already stored on this device.
barcode-title = Barcode
barcode-label = Barcode number
barcode-check = Check barcode
barcode-loading = Loading …
barcode-privacy = Only the number leaves the device. It goes to Open Food Facts.
barcode-camera-later = Scanning with the camera is available in the web app. Here you enter the number.
fetch-invalid-code = This number is not a valid barcode. A barcode has 8, 12, 13 or 14 digits.
fetch-not-found = Open Food Facts does not know this product.
fetch-network = Open Food Facts cannot be reached right now. Please check the internet connection.
fetch-unavailable = Open Food Facts is not answering right now. Please try again later.
fetch-format = The answer from Open Food Facts could not be read.
plates-saved = Saved plates
plates-saved-empty = No plates saved yet.
plates-food-count = { $count } foods
my-foods-title = My foods
my-foods-hint = Tapping puts the saved portion on the plate.
my-foods-empty = No foods of your own yet. You can add one on the food card.
my-foods-details = View evaluation of { $name }
plate-added = { $name } is on the plate.

## 06 Portion and preparation

portion-topbar = Portion
portion-how-much = How much?
portion-grams = Amount in grams
portion-quick = Quick choice
portion-how-prepared = How is it prepared?
portion-preparation-note = The preparation is saved. The app evaluates its effect on the values once reviewed sources are available.
portion-per-100 = The source gives values per 100 g. The app converts them to your portion.
food-kind-generic = Generic
food-kind-brand = Branded product
food-kind-dish = Dish
food-missing-title = This food is not stored.
food-missing-text = Search for it again or enter the barcode.
preparation-raw = raw
preparation-boiled = boiled
preparation-steamed = steamed
preparation-fried = fried
preparation-grilled = grilled
preparation-smoked = smoked

## 07/08 Food card

card-topbar = Result
card-checked-for = Checked for { $name }
card-no-profile = For an evaluation, first create a profile with conditions and allergies.
card-attention = Pay attention
card-unknown = Not known yet
card-unremarkable = Unremarkable
card-nothing-to-check = Your profile contains no condition and no allergy, so there is nothing to evaluate. You find the nutrients below.
card-conflicts = Question for your doctor
card-conflict = For { $nutrient }, { $upper } and { $lower } ask for opposite things. The app does not decide this. Please clarify it with your doctor.
card-draft = Draft, not reviewed
card-values-and-source = Values and source
card-all-nutrients = All nutrients
card-to-plate = Put on the plate
card-add-my-foods = Add to my foods
card-in-my-foods = In my foods.
level-ok = good
level-caution = in moderation
level-avoid = avoid
level-unknown = unknown
why-allergy = because of your allergy
why-doctor = because of your doctor's limit
why-conditions = because of { $conditions }
reason-allergen-found = Found in the data: { $allergens }. Data completeness { $completeness }.
reason-allergen-traces = Traces noted in the data: { $allergens }. Data completeness { $completeness }.
reason-no-allergen-found = No allergen of your profile found in the data. Data completeness { $completeness }.
reason-allergen-data-missing = The source has no information on ingredients or allergens.
reason-within-limit = With this portion you stay below the limit of { $limit }.
reason-near-limit = With this portion you get close to the limit of { $limit }.
reason-over-limit = With this portion the limit of { $limit } is exceeded.
reason-value-unknown = The source has no value for { $nutrient }. Unknown does not mean zero.
reason-no-reviewed-rule = There is no reviewed rule for this yet, so the app does not evaluate it.
completeness-unknown = unknown
fact-per-portion = Per portion
fact-limit = Limit
fact-today = Today with this portion
fact-source = Source
fact-evidence = Evidence
fact-completeness = Data completeness
origin-doctor = your doctor's limit
origin-rule = rule { $id }
origin-rule-short = rule
evidence-high = high
evidence-medium = medium
evidence-low = low
basis-portion = per portion
basis-day = per day
basis-week = per week
dimension-allergens = Allergens
dimension-blood-sugar = Blood sugar
dimension-purine = Purine
dimension-kidney = Kidney
dimension-blood-pressure = Blood pressure
dimension-liver = Liver
dimension-histamine = Histamine
dimension-oxalate = Oxalate
dimension-processing = Processing
dimension-cancer = Cancer-related categories
dimension-preparation = Preparation
dimension-medication = Medication
dimension-basics = Basic nutrients

## 09 Terms

term-topbar = Term
term-explain = explained simply
term-purine-title = Purine
term-purine-text = Purines are building blocks found in many foods, especially meat, offal, fish and pulses. The body breaks them down into uric acid. With gout, too much uric acid can cause joint pain. The app therefore states purine as uric acid.
term-glycemic-load-title = Glycaemic load
term-glycemic-load-text = The glycaemic load describes how much a portion raises blood sugar. It combines two things: how fast the carbohydrates of a food enter the blood and how many carbohydrates the portion contains.
term-nova-title = NOVA group
term-nova-text = NOVA sorts foods into four groups by how processed they are. Group 1 is unprocessed or minimally processed food, group 4 is highly processed products.
term-traces-title = Trace warning
term-traces-text = A trace warning such as "May contain traces of egg" means that an allergen can get into the product unintentionally, for example through the same production line. For people with a strong allergy, this can already be too much.
term-histamine-title = Histamine
term-histamine-text = Histamine is a messenger substance that also occurs in food, mainly in long-matured, fermented or no longer fresh food. With histamine intolerance the body breaks it down poorly. How much someone tolerates varies a lot.
term-oxalate-title = Oxalate
term-oxalate-text = Oxalate is a substance in plants, for example in spinach, rhubarb and chard. In the body it can bind with calcium. In some people this forms calcium oxalate kidney stones.
term-completeness-title = Data completeness
term-completeness-text = Data completeness shows how much information the data source has on a food. The lower it is, the more likely information is missing, for example on ingredients or allergens. The app never counts missing information as zero.
term-estimate-title = Estimate
term-estimate-text = If a value is missing in all sources, the app can estimate it with a fixed method, for example from a comparable food. Estimated values are marked ≈ and state what they are based on.

## 10 Nutrient list

nutrients-title = All nutrients
nutrients-lead = Per portion. The share of your personal target appears once reference values with a source are available.
nutrients-origin = Origin of the values
nutrients-group-summary = { $count } values, { $unknown } unknown
nutrients-col-name = Nutrient
nutrients-col-portion = Portion
nutrients-col-origin = Origin
value-unknown = unknown
origin-measured = measured
origin-calculated = calculated by the source
origin-declared = manufacturer's declaration
origin-estimate = estimate
origin-unknown = unknown
group-energy = Energy
group-macronutrients = Macronutrients
group-carbohydrates = Carbohydrates and sugars
group-fats = Fatty acids
group-minerals = Minerals
group-trace-elements = Trace elements
group-vitamins = Vitamins
group-other = Other
nutrient-energy = Energy
nutrient-protein = Protein
nutrient-fat = Fat
nutrient-carbohydrate = Carbohydrates
nutrient-fibre = Fibre
nutrient-water = Water
nutrient-alcohol = Alcohol
nutrient-sugars = Sugars
nutrient-starch = Starch
nutrient-saturated-fat = Saturated fatty acids
nutrient-monounsaturated-fat = Monounsaturated fatty acids
nutrient-polyunsaturated-fat = Polyunsaturated fatty acids
nutrient-omega-3 = Omega-3 fatty acids
nutrient-omega-6 = Omega-6 fatty acids
nutrient-cholesterol = Cholesterol
nutrient-salt = Salt
nutrient-sodium = Sodium
nutrient-potassium = Potassium
nutrient-calcium = Calcium
nutrient-magnesium = Magnesium
nutrient-phosphorus = Phosphorus
nutrient-iron = Iron
nutrient-zinc = Zinc
nutrient-iodine = Iodine
nutrient-selenium = Selenium
nutrient-vitamin-a = Vitamin A
nutrient-vitamin-d = Vitamin D
nutrient-vitamin-e = Vitamin E
nutrient-vitamin-k = Vitamin K
nutrient-vitamin-c = Vitamin C
nutrient-vitamin-b1 = Vitamin B1
nutrient-vitamin-b2 = Vitamin B2
nutrient-niacin = Niacin
nutrient-vitamin-b6 = Vitamin B6
nutrient-folate = Folate
nutrient-vitamin-b12 = Vitamin B12
nutrient-purine = Purine (as uric acid)
nutrient-oxalate = Oxalate

## 11 Estimates

estimates-all = View all estimates
estimates-title = All estimates
estimates-filter = Filter by food or nutrient
estimates-list = Estimated values
estimates-empty = There are no estimated values at the moment.
estimates-csv = Export as CSV

## 12/13 Plate

plate-topbar = Plate
plate-today = Today
plate-focus = Focus
focus-conditions = Conditions
focus-conditions-sub = Key figures of your conditions first
focus-nutrients = Nutrients
focus-nutrients-sub = Vitamins, minerals and gaps first
plate-limits = Key figures of your conditions
plate-no-limits = No reviewed limits are available for your conditions yet. With a limit from your doctor, the plate shows here how much adds up today.
plate-no-conditions = Your profile contains no condition.
plate-enter-doctor-limits = Enter doctor's limits
plate-nutrients = Nutrient supply
plate-nutrients-note = Daily total per nutrient. Targets according to DGE/ÖGE appear once they are available with a source.
plate-empty = Nothing entered today yet.
plate-incomplete = (not all values known)
plate-incomplete-long = A value is not known for every food. The total may be higher.
plate-current = Current plate
plate-current-empty = The current plate is empty.
plate-saved-today = Saved today: { $plates } plates with { $foods } foods.
plate-add-food = Add food
plate-save = Save plate
plate-save-name = Name, for example My breakfast
plate-save-name-hint = With a name, the plate appears under Saved plates and can be used again later.
plate-save-action = Save plate
plate-allergen-found = { $allergens } found
plate-allergen-in = In { $name } on today's plate.
plate-over-max = { $nutrient } over max
plate-over-max-detail = Today { $total }, the limit is { $limit }.
legend-saved = saved plates
legend-current = current plate
legend-target = target range
legend-over = over max
bar-max = Max
bar-left = Target range, { $amount } left to max
bar-over = Over max, { $amount } above
bar-origin = Max { $limit }, origin: { $origin }
bar-label = { $nutrient }: { $total } today of at most { $limit }

## 15 Edit my foods

my-foods-edit-lead = The foods you eat often and like. Search for them above and add them. Change the portion in the list; the arrows change the order.
my-foods-count = { $count } foods
my-foods-portion = Portion
my-foods-portion-of = Portion of { $name } in grams

## 17/18/19 Profile

profiles-topbar = Profiles
profiles-title = Profiles
profiles-list = On this device
profiles-empty = No profile created yet.
profiles-new = New profile
profiles-local = Stored only on this device.
profile-new-title = New profile
profile-edit-title = Profile for { $name }
profile-step = Step { $step } of { $total } · { $name }
profile-step-1 = Person
profile-step-2 = Conditions and allergies
profile-step-3 = Doctor's limits
profile-person-title = Who is the profile for?
profile-person-lead = Date of birth and sex later determine the matching reference values. The app works out the age itself.
profile-person = Person
profile-name = Name
profile-name-hint = For example your first name or "Mum".
profile-name-missing = Please enter a name.
profile-sex = Sex
profile-reference-hint = Only for the reference values. It stays on this device.
sex-female = female
sex-male = male
sex-diverse = diverse
profile-conditions-title = What should the app take into account?
profile-conditions-lead = Choose what a doctor has diagnosed. You can change this at any time.
profile-conditions = Conditions
profile-allergies = Allergies and intolerances
profile-allergies-all = Show all 14 major allergens
profile-focus = Focus on the plate
profile-focus-auto-conditions = Because a condition is selected, the plate first shows the values of your conditions. You can switch at any time.
profile-focus-auto-nutrients = Without a condition, the plate first shows the nutrient supply. You can switch at any time.
profile-doctor-title = Enter doctor's limits
profile-doctor-lead = Optional. Only enter values your doctor has given you. Each limit counts as a maximum per day. Empty fields stay empty.
profile-doctor-per-day = Maximums per day
profile-doctor-none = There is nothing to enter here for the selected conditions.
profile-doctor-empty = Empty, no limit applies.
profile-doctor-set = Your limit replaces the default value of a rule.
profile-doctor-origin = Where does the limit come from?
profile-doctor-note = Practice and date
profile-doctor-note-hint = For example family practice, 30.09.2026.
profile-doctor-no-dose = The app only adopts limits. It does not calculate medication or insulin doses.
profile-delete = Delete profile
profile-no-conditions = no condition and no allergy given
condition-diabetes = Diabetes or prediabetes
condition-gout = Gout or high uric acid
condition-hypertension = High blood pressure
condition-kidney-disease = Kidney failure
condition-liver-values = Raised liver values
condition-histamine-intolerance = Histamine intolerance
condition-oxalate-stones = Calcium oxalate kidney stones
allergen-gluten = Cereals containing gluten
allergen-crustaceans = Crustaceans
allergen-eggs = Egg
allergen-fish = Fish
allergen-peanuts = Peanuts
allergen-soybeans = Soy
allergen-milk = Milk
allergen-nuts = Tree nuts
allergen-celery = Celery
allergen-mustard = Mustard
allergen-sesame = Sesame
allergen-sulphites = Sulphur dioxide and sulphites
allergen-lupin = Lupin
allergen-molluscs = Molluscs
allergy-gluten = Gluten intolerance
allergy-crustaceans = Crustacean allergy
allergy-eggs = Egg allergy
allergy-fish = Fish allergy
allergy-peanuts = Peanut allergy
allergy-soybeans = Soy allergy
allergy-milk = Milk allergy
allergy-nuts = Nut allergy
allergy-celery = Celery allergy
allergy-mustard = Mustard allergy
allergy-sesame = Sesame allergy
allergy-sulphites = Sulphite intolerance
allergy-lupin = Lupin allergy
allergy-molluscs = Mollusc allergy

## 20 Settings and data sources

settings-title = Settings
settings-reference = Reference values
settings-reference-lead = Determine target and max on the plate, matching age and sex.
reference-dge = DGE/ÖGE
reference-dge-sub = Standard for Germany and Austria
reference-efsa = EFSA
reference-efsa-sub = European reference values
settings-reference-pending = The reference values are being added as cited single values. Until then, the plate shows no targets.
settings-theme = Appearance
theme-system = like the device
theme-light = light
theme-dark = dark
settings-sources = Data sources
settings-sources-lead = The app loads all data directly from the original sources and stores it on this device.
source-active = active
source-not-yet = not included yet
source-permission = permission pending
source-license = Licence: { $license }
source-off-cache = { $count } products in storage
source-off-clear = Clear storage
source-website = Website of { $name }
settings-rules = Rules
settings-rules-count = { $count } rules loaded, { $drafts } of them drafts.
settings-rules-drafts = Show draft rules (not reviewed, for testing)
settings-more = More
settings-device = Data on this device
settings-device-lead = Profiles, my foods, plates and the storage of the data sources are only on this device.
settings-delete = Delete all data
settings-delete-confirm = Really delete all data on this device? This cannot be undone.
settings-delete-final = Delete permanently

## About

about-link = About the app and basic notice
about-version = Version { $version }
about-license = Licence
about-license-text = Healthy Teller is free software under the GNU General Public License, version 3 or later. Rules and texts are under CC BY-SA 4.0.
about-source-code = Source code on GitHub
about-credits = Sources and fonts
about-credits-off = Product data: Open Food Facts, https://world.openfoodfacts.org, Open Database License (ODbL).
about-credits-fonts = Font: Atkinson Hyperlegible Next and Mono by the Braille Institute, SIL Open Font License 1.1.

## 22 Print list

print-create = Create print list
print-title = Food list for { $name }
print-date = created on { $date }
print-group-ok = Fits according to the data
print-group-caution = In moderation
print-group-avoid = Avoid
print-group-unknown = Cannot be evaluated yet
print-empty = The list is made from my foods. Add foods there first.
print-action = Print

## Open Food Facts search, file import, more information

search-online-title = Search Open Food Facts
search-online-action = Search online by name or brand
search-online-privacy = Only the search term leaves the device. It goes to Open Food Facts.
search-online-none = Open Food Facts found nothing for this.
import-label = Or import a product file
import-hint = A JSON file you downloaded from Open Food Facts yourself. It stays on this device.
import-read-error = The file could not be read.
card-more = More information from the source
card-nova = Group { $group } of 4
card-additives = Additives
card-ingredients = Ingredients
card-more-note = This information comes unchanged from the source. The app does not evaluate it yet.

## Moving to another device

transfer-title = Moving to another device
transfer-lead = Profiles, my foods, saved plates and settings can be moved as an encrypted file. Without the password nobody can read the file, not even the project.
transfer-export = Export
transfer-pass = Password
transfer-pass-repeat = Repeat password
transfer-pass-hint = At least { $min } characters. Without this password the file cannot be opened.
transfer-export-action = Create encrypted file
transfer-save-file = Save file
transfer-saved = Saved to { $path }
transfer-no-folder = There is no downloads folder on this device.
transfer-import = Import
transfer-file = Export file (.htx)
transfer-import-warning = Importing replaces profiles, my foods, saved plates and settings on this device.
transfer-import-action = Import
transfer-imported = Import complete.
transfer-pass-short = The password is too short.
transfer-pass-mismatch = The two passwords do not match.
transfer-not-export = This is not an export file from Healthy Teller.
transfer-wrong-pass = The password is wrong or the file is damaged.
transfer-crypto-error = Encryption failed.

## Birth date, my foods on the start page

profile-birth-date = Date of birth
profile-age-today = { $age } years old today
home-my-foods-empty = Add the foods you eat often and like once. Then they go on the plate with one tap.
home-my-foods-setup = Set up my foods
home-my-foods-edit = Edit my foods
home-my-foods-more = View and edit all ({ $count } more)
my-foods-add-title = Add food
my-foods-search = Search
my-foods-added = { $name } is now in my foods, with 100 g as portion. You can change the portion below.
my-foods-already = already in the list
my-foods-add-named = Add { $name }

## Camera scan

scan-start = Scan with the camera
scan-starting = Starting camera …
scan-hint = Hold the barcode steady in the picture. It is recognised automatically.
scan-no-camera = The camera could not be opened. Please allow access or enter the number.
scan-video-label = Camera picture for scanning the barcode

settings-language = Language

## 23 Comparison, 24 Questions for the doctor

card-compare = Compare with another food
compare-topbar = Comparison
compare-instead-of = Instead of { $name }
compare-pick = Compare with what?
compare-pick-lead = Choose a food from my foods or from the storage of this device.
compare-none = No second food is stored on this device yet. Search for another one first.
compare-per-portion = Per { $grams } g.
compare-note = The values come unchanged from the source. Unknown does not mean zero. The app does not choose for you.
compare-add = Put { $name } on the plate
compare-open = Open the card of { $name }
questions-link = Questions for the doctor
questions-topbar = Doctor's appointment
questions-title = Questions for the next appointment
questions-lead = The app collects open questions from the profile of { $name }. It does not decide them.
questions-none = There are no open questions from the profile at the moment. You can note your own questions below.
questions-kind-conflict = Conflict
questions-kind-missing = Limit missing
questions-kind-traces = Trace warning
questions-question = Question
questions-conflict-context = Opposite rules apply here for { $upper } and for { $lower }.
questions-conflict = How much { $nutrient } per day is right for me?
questions-missing-context = Because of { $condition } this is an important figure. No limit is entered in the profile yet.
questions-missing = Is there a limit for { $nutrient } per day for me?
questions-traces-context = The data mention a trace warning: { $allergens }.
questions-traces = Are foods with a trace warning for { $allergens } fine for me?
questions-include = Include in the report
questions-own = Own question
questions-print = Print report or save as PDF
questions-count = { $chosen } of { $total } questions selected
