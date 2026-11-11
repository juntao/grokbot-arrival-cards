# Field matrix: which answers each card needs, and where they come from

Generated from `catalog/` by `cargo run -- render`. Do not edit by hand.

One answer covers every card that uses its key, so the bot never asks the same thing twice.

## 🔐 VAULT: identity, asked once, saved to the traveller's Vault entry

| Key | Question | Source / saved to | CN | JP | SG | TW |
|---|---|---|---|---|---|---|
| `full_name` | Full name as in passport | Vault Contact › Name |  |  | ✓ | ✓ |
| `surname` | Surname / family name | Vault hint `surname=` | ✓ | ✓ |  |  |
| `given_names` | Given name(s) | Vault hint `given_names=` | ✓ | ✓ |  |  |
| `other_names` | Other names used (former / alias), or 'none' | Vault hint `other_names=` | ✓ |  |  |  |
| `other_citizenship` | Other countries / regions of citizenship, or 'none' | Vault hint `other_citizenship=` | ✓ |  |  |  |
| `chinese_name` | Name in Chinese characters (optional) | Vault hint `chinese_name=` | ✓ |  |  | ✓ |
| `sex` | Sex as in passport (F / M / X) | Vault hint `sex=` | ✓ | ✓ | ✓ | ✓ |
| `dob` | Date of birth | Vault hint `dob=` | ✓ | ✓ | ✓ | ✓ |
| `nationality` | Nationality (passport country) | Vault hint `nationality=` | ✓ | ✓ | ✓ | ✓ |
| `birth_country` | Country of birth | Vault hint `birth_country=` | ✓ |  | ✓ | ✓ |
| `birth_city` | City of birth | Vault hint `birth_city=` | ✓ |  |  | ✓ |
| `passport_no` | Passport number | Vault hint `passport_no=` | ✓ | ✓ | ✓ | ✓ |
| `passport_country` | Passport issuing country | Vault hint `passport_country=` | ✓ |  |  |  |
| `passport_expiry` | Passport date of expiry | Vault hint `passport_expiry=` |  | ✓ | ✓ | ✓ |
| `passport_photo` | Photo of the passport data page | Vault hint `passport_photo=`. Collected at onboarding. Read once to fill the other identity fields; with the user's OK a copy is kept at /workspace/arrival-cards/identity/<GIVEN>_<SURNAME>_passport.jpg for China's upload step, and its path is the hint value. | ✓ |  |  |  |
| `home_city` | City of residence | Vault hint `home_city=` |  | ✓ | ✓ |  |
| `home_country` | Country of residence | Vault hint `home_country=` |  | ✓ |  | ✓ |
| `mobile` | Mobile number with country code | Vault Contact › Phone | ✓ |  | ✓ | ✓ |
| `email` | Email address for the card | Vault Contact › Email | ✓ |  | ✓ | ✓ |
| `occupation` | Occupation / job title | Vault hint `occupation=` |  | ✓ |  | ✓ |
| `visa` | Visa or residence permit for this country (number, or 'none') | Vault hint `visa=`. Stored per destination as visa_CN / visa_JP / visa_TW; 'none' when visa-exempt. | ✓ |  |  | ✓ |
| `vjw_login` | Visit Japan Web account (email + password) | Vault Login. A Vault Login for vjw.digital.go.jp, filled into the page without the Bot seeing the password. |  | ✓ |  |  |

## 🧠 RECALL: found in memory, email or calendar before asking; saved to memory

| Key | Question | Source / saved to | CN | JP | SG | TW |
|---|---|---|---|---|---|---|
| `arrival_date` | Arrival date | Flight e-ticket (local arrival date, not departure date) -> calendar flight event. → saved to trip log | ✓ | ✓ | ✓ | ✓ |
| `arrival_flight` | Arrival flight (airline code + number) | E-ticket / airline confirmation; check for later schedule-change emails -> calendar event title. → saved to trip log | ✓ | ✓ | ✓ | ✓ |
| `arrival_airport` | Airport / port of arrival | E-ticket arrival airport (IATA code). → saved to trip log | ✓ |  |  |  |
| `origin` | City / country of embarkation (last stop before arrival) | E-ticket departure airport of the arriving flight segment. → saved to trip log |  | ✓ | ✓ |  |
| `departure_date` | Date leaving the country | Onward or return flight e-ticket -> calendar -> hotel check-out date. → saved to trip log | ✓ |  | ✓ | ✓ |
| `departure_flight` | Departure flight (airline code + number) | Onward or return flight e-ticket -> calendar. → saved to trip log | ✓ |  |  | ✓ |
| `next_destination` | Next city / country after this one | Arrival airport of the onward flight; usually home. → saved to trip log |  |  | ✓ |  |
| `stay_type` | Accommodation type (hotel / residence / friends-relatives / transit / other) | Hotel or Airbnb booking -> 'own address' preference in memory. → saved to trip log |  |  | ✓ | ✓ |
| `stay_name` | Hotel / host name | Booking confirmation email (Booking.com, Agoda, Airbnb, Trip.com, hotel direct). → saved to trip log |  | ✓ | ✓ | ✓ |
| `stay_address` | Address of stay (with postal code) | Booking confirmation email; hotel website if the email lacks the postal code. → saved to trip log | ✓ | ✓ | ✓ |  |
| `stay_phone` | Phone number of the stay | Booking confirmation email -> hotel website. → saved to trip log |  | ✓ |  |  |
| `purpose` | Purpose of visit | Calendar (meetings/conference => business) -> invitation emails -> memory preference 'usual purpose'. → saved to memory preference | ✓ | ✓ | ✓ | ✓ |
| `length_of_stay` | Length of stay (days) | Derived: departure_date - arrival_date. → saved to trip log |  | ✓ |  |  |
| `destination_cities` | Cities to be visited in the country | Hotel cities and calendar event locations within the trip dates. → saved to trip log | ✓ |  |  |  |
| `entry_policy` | Entry basis (visa-free / visa / transit visa-free / residence permit) | Derived from nationality + visa field + length of stay; confirm on the form when unsure. → saved to trip log | ✓ |  |  | ✓ |
| `inviting_entity` | Inviting company / person in the country (name, contact) | Invitation letter or conference email; 'none' for tourism. → saved to trip log | ✓ |  |  |  |
| `host_contact` | Relative / friend you are visiting: name and mobile | Only when purpose is visiting relatives/friends: memory contacts -> calendar event attendees -> ask. → saved to trip log |  |  |  | ✓ |
| `local_phone` | Local phone number in the destination country (if any) | Memory (e.g. a China SIM / eSIM number) -> 'none'; optional. → saved to memory preference | ✓ |  |  |  |
| `transit_cities` | Cities transited inside the country | Domestic connecting flights / train tickets in email within the trip dates; usually none. → saved to trip log | ✓ |  |  |  |
| `companions` | Travel companions on this trip | Other passengers on the same e-ticket / booking -> memory 'usually travels with'. → saved to trip log | ✓ | ✓ |  |  |

## ✍️ ASK: declarations, asked fresh on every card, never saved

| Key | Question | Source / saved to | CN | JP | SG | TW |
|---|---|---|---|---|---|---|
| `health_symptoms` | Do you currently have fever, cough, breathing difficulty, rash or other symptoms of an infectious disease? | Ask every time; never saved |  |  | ✓ |  |
| `recent_travel_yf` | Countries / regions visited in the last 6 days before arrival (yellow-fever risk check) | Ask every time; never saved |  |  | ✓ |  |
| `recent_travel_2y` | Other countries or regions visited in the past two years (list them) | Ask every time; never saved. China asks this. The bot may *suggest* trips found in email/calendar as a starting list, but the user confirms. | ✓ |  |  |  |
| `passport_lost` | Has a passport of yours ever been lost or stolen? | Ask every time; never saved | ✓ |  |  |  |
| `signature` | Signature (drawn on the screen) | Ask every time; never saved. China only; drawn by the user during a Take control hand-over. | ✓ |  |  |  |
| `other_passport_name` | Have you ever used a passport under a different name to enter this country? | Ask every time; never saved |  |  | ✓ |  |
| `deported_before` | Ever been deported from, or refused entry to, this country? | Ask every time; never saved |  | ✓ |  |  |
| `criminal_conviction` | Ever been convicted of a crime in any country? | Ask every time; never saved |  | ✓ |  |  |
| `prohibited_items` | Carrying narcotics, firearms, swords, explosives or other prohibited/restricted items? | Ask every time; never saved |  | ✓ |  |  |
| `customs_family_count` | Number of family members travelling with you (customs) | Ask every time; never saved |  | ✓ |  |  |
| `customs_over_allowance` | Carrying goods beyond the duty-free allowance? | Ask every time; never saved |  | ✓ |  |  |
| `customs_commercial` | Carrying commercial goods or samples? | Ask every time; never saved |  | ✓ |  |  |
| `customs_for_others` | Carrying items on behalf of someone else? | Ask every time; never saved |  | ✓ |  |  |
| `customs_cash` | Carrying cash / securities / precious metals above the declaration threshold? | Ask every time; never saved |  | ✓ |  |  |
| `customs_unaccompanied` | Do you have unaccompanied baggage (shipped separately)? | Ask every time; never saved |  | ✓ |  |  |
| `customs_restricted` | Carrying meat products, vegetables, fruit, animals or plants (restricted items)? | Ask every time; never saved |  | ✓ |  |  |
| `customs_gold` | Carrying gold bullion or products of gold? | Ask every time; never saved |  | ✓ |  |  |
| `customs_goods_detail` | Itemised goods to declare (alcohol bottles, cigarettes, perfume, other goods with value) | Ask every time; never saved. Only when a customs question is answered Yes. |  | ✓ |  |  |

