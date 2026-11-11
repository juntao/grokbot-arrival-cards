# Field matrix: which answers each card needs, and where they come from

Generated from `catalog/` by `cargo run -- render`. Do not edit by hand.

One answer covers every card that uses its key, so the bot never asks the same thing twice.

## 🔐 VAULT: identity, asked once, saved to the traveller's Vault entry

| Key | Question | Source / saved to | CN | ID | IN | JP | KH | KR | MV | MY | NZ | PH | SG | TH | TW | VN |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `full_name` | Full name as in passport | Vault Contact › Name |  | ✓ | ✓ |  |  |  |  | ✓ |  |  | ✓ |  | ✓ |  |
| `surname` | Surname / family name | Vault hint `surname=` | ✓ |  |  | ✓ | ✓ | ✓ | ✓ |  | ✓ | ✓ |  | ✓ |  | ✓ |
| `given_names` | Given name(s) | Vault hint `given_names=` | ✓ |  |  | ✓ | ✓ | ✓ | ✓ |  | ✓ | ✓ |  | ✓ |  | ✓ |
| `other_names` | Other names used (former / alias), or 'none' | Vault hint `other_names=` | ✓ |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `other_citizenship` | Other countries / regions of citizenship, or 'none' | Vault hint `other_citizenship=` | ✓ |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `middle_name` | Middle name, or 'none' | Vault hint `middle_name=`. Thailand has a separate box; when the passport prints one given-names line, split it only if the user says so. |  |  |  |  |  |  |  |  |  | ✓ |  | ✓ |  |  |
| `chinese_name` | Name in Chinese characters (optional) | Vault hint `chinese_name=` | ✓ |  |  |  |  |  |  |  |  |  |  |  | ✓ |  |
| `sex` | Sex as in passport (F / M / X) | Vault hint `sex=` | ✓ | ✓ |  | ✓ | ✓ | ✓ | ✓ | ✓ |  | ✓ | ✓ | ✓ | ✓ | ✓ |
| `dob` | Date of birth | Vault hint `dob=` | ✓ | ✓ |  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `nationality` | Nationality (passport country) | Vault hint `nationality=` | ✓ | ✓ | ✓ | ✓ |  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `birth_country` | Country of birth | Vault hint `birth_country=` | ✓ | ✓ |  |  | ✓ |  |  | ✓ | ✓ | ✓ | ✓ |  | ✓ |  |
| `birth_city` | City of birth | Vault hint `birth_city=` | ✓ |  |  |  |  |  |  |  |  |  |  |  | ✓ |  |
| `passport_no` | Passport number | Vault hint `passport_no=` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `passport_country` | Passport issuing country | Vault hint `passport_country=` | ✓ |  |  |  | ✓ |  |  |  |  | ✓ |  |  |  | ✓ |
| `passport_issue` | Passport date of issue | Vault hint `passport_issue=` |  |  |  |  |  |  |  |  |  | ✓ |  |  |  |  |
| `passport_expiry` | Passport date of expiry | Vault hint `passport_expiry=` |  | ✓ |  | ✓ | ✓ | ✓ | ✓ | ✓ |  |  | ✓ |  | ✓ | ✓ |
| `passport_photo` | Photo of the passport data page | Vault hint `passport_photo=`. Collected at onboarding. Read once to fill the other identity fields; with the user's OK a copy is kept at /workspace/arrival-cards/identity/<GIVEN>_<SURNAME>_passport.jpg for China's upload step, and its path is the hint value. | ✓ |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `portrait_photo` | Passport-style photo of the traveller's face | Vault hint `portrait_photo=`. Maldives needs an upload. With the user's OK a copy is kept at /workspace/arrival-cards/identity/<GIVEN>_<SURNAME>_portrait.jpg and its path is the hint value; a clear crop from the passport photo page is not acceptable. |  |  |  |  |  |  | ✓ |  |  | ✓ |  |  |  |  |
| `home_address` | Home address (street, city, state or province) | Vault Contact › Address |  |  |  |  |  |  |  |  |  | ✓ |  |  |  |  |
| `emergency_contact` | Emergency contact phone number, with country code (optional) | Vault hint `emergency_contact=` |  |  | ✓ |  |  |  |  |  |  |  |  |  |  |  |
| `home_city` | City of residence | Vault hint `home_city=` |  |  |  | ✓ |  |  |  |  |  |  | ✓ | ✓ |  |  |
| `home_state` | State, province or prefecture of residence | Vault hint `home_state=` |  |  |  |  |  |  |  |  | ✓ |  |  |  |  |  |
| `home_country` | Country of residence | Vault hint `home_country=` |  |  |  | ✓ |  |  | ✓ |  | ✓ | ✓ |  | ✓ | ✓ |  |
| `mobile` | Mobile number with country code | Vault Contact › Phone | ✓ | ✓ | ✓ |  | ✓ |  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `email` | Email address for the card | Vault Contact › Email | ✓ | ✓ | ✓ |  | ✓ | ✓ | ✓ | ✓ | ✓ |  | ✓ | ✓ | ✓ | ✓ |
| `occupation` | Occupation / job title | Vault hint `occupation=` |  |  |  | ✓ | ✓ | ✓ |  |  | ✓ | ✓ |  | ✓ | ✓ |  |
| `visa` | Visa or residence permit for this country (number, or 'none') | Vault hint `visa=`. Stored per destination as visa_CN / visa_JP / visa_TW / visa_KR / visa_TH / visa_ID / visa_NZ / visa_VN; 'none' when visa-exempt. | ✓ | ✓ |  |  |  | ✓ | ✓ |  |  |  |  | ✓ | ✓ | ✓ |
| `etravel_login` | Philippines eTravel account (email + password) | Vault Login. A Vault Login for etravel.gov.ph, created on the first Philippines card and filled into the page without the Bot seeing the password. |  |  |  |  |  |  |  |  |  | ✓ |  |  |  |  |
| `vjw_login` | Visit Japan Web account (email + password) | Vault Login. A Vault Login for vjw.digital.go.jp, filled into the page without the Bot seeing the password. |  |  |  | ✓ |  |  |  |  |  |  |  |  |  |  |

## 🧠 RECALL: found in memory, email or calendar before asking; saved to memory

| Key | Question | Source / saved to | CN | ID | IN | JP | KH | KR | MV | MY | NZ | PH | SG | TH | TW | VN |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `arrival_date` | Arrival date | Flight e-ticket (local arrival date, not departure date) -> calendar flight event. → saved to trip log | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `arrival_flight` | Arrival flight (airline code + number) | E-ticket / airline confirmation; check for later schedule-change emails -> calendar event title. → saved to trip log | ✓ | ✓ |  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `arrival_airport` | Airport / port of arrival | E-ticket arrival airport (IATA code). → saved to trip log | ✓ | ✓ |  |  | ✓ |  | ✓ |  | ✓ | ✓ |  |  |  | ✓ |
| `origin` | City / country of embarkation (last stop before arrival) | E-ticket departure airport of the arriving flight segment. → saved to trip log |  |  |  | ✓ | ✓ |  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |  | ✓ |
| `departure_date` | Date leaving the country | Onward or return flight e-ticket -> calendar -> hotel check-out date. → saved to trip log | ✓ | ✓ |  |  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `departure_flight` | Departure flight (airline code + number) | Onward or return flight e-ticket -> calendar. → saved to trip log | ✓ |  |  |  |  | ✓ | ✓ |  | ✓ |  |  | ✓ | ✓ |  |
| `next_destination` | Next city / country after this one | Arrival airport of the onward flight; usually home. → saved to trip log |  |  |  |  | ✓ |  | ✓ |  |  |  | ✓ |  |  |  |
| `stay_type` | Accommodation type (hotel / residence / friends-relatives / transit / other) | Hotel or Airbnb booking -> 'own address' preference in memory. → saved to trip log |  | ✓ |  |  | ✓ | ✓ |  | ✓ |  | ✓ | ✓ | ✓ | ✓ | ✓ |
| `stay_name` | Hotel / host name | Booking confirmation email (Booking.com, Agoda, Airbnb, Trip.com, hotel direct). → saved to trip log |  | ✓ |  | ✓ | ✓ | ✓ | ✓ | ✓ |  | ✓ | ✓ | ✓ | ✓ | ✓ |
| `stay_address` | Address of stay (with postal code) | Booking confirmation email; hotel website if the email lacks the postal code. → saved to trip log | ✓ | ✓ | ✓ | ✓ |  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |  | ✓ |
| `stay_phone` | Phone number of the stay | Booking confirmation email -> hotel website. → saved to trip log |  |  |  | ✓ | ✓ | ✓ |  |  |  |  |  |  |  |  |
| `purpose` | Purpose of visit | Calendar (meetings/conference => business) -> invitation emails -> memory preference 'usual purpose'. → saved to memory preference | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `length_of_stay` | Length of stay (days) | Derived: departure_date - arrival_date. → saved to trip log |  |  |  | ✓ |  |  | ✓ |  | ✓ |  |  |  |  |  |
| `destination_cities` | Cities to be visited in the country | Hotel cities and calendar event locations within the trip dates. → saved to trip log | ✓ |  | ✓ |  |  |  |  |  |  |  |  |  |  |  |
| `entry_policy` | Entry basis (visa-free / visa / transit visa-free / residence permit) | Derived from nationality + visa field + length of stay; confirm on the form when unsure. → saved to trip log | ✓ | ✓ |  |  |  |  |  |  | ✓ |  |  |  | ✓ | ✓ |
| `inviting_entity` | Inviting company / person in the country (name, contact) | Invitation letter or conference email; 'none' for tourism. → saved to trip log | ✓ |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `host_contact` | Relative / friend you are visiting: name and mobile | Only when purpose is visiting relatives/friends: memory contacts -> calendar event attendees -> ask. → saved to trip log |  |  |  |  |  |  |  |  |  |  |  |  | ✓ |  |
| `local_phone` | Local phone number in the destination country (if any) | Memory (e.g. a China SIM / eSIM number) -> 'none'; optional. → saved to memory preference | ✓ |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `transit_cities` | Cities transited inside the country | Domestic connecting flights / train tickets in email within the trip dates; usually none. → saved to trip log | ✓ |  |  |  |  |  |  |  |  | ✓ |  |  |  |  |
| `seat_number` | Seat number on the arriving flight | Boarding pass or check-in confirmation email; optional, leave empty before check-in. → saved to trip log |  | ✓ |  |  |  |  |  |  |  |  |  |  |  |  |
| `booking_ref` | Airline booking reference (PNR) | E-ticket or booking confirmation email: the 6-character code. → saved to trip log |  |  |  |  |  |  | ✓ |  |  |  |  |  |  |  |
| `resident_status` | Do you live in the destination country, or intend to stay permanently? | Derived: No to both for a visitor whose home country differs and who holds no residence visa; confirm on the form when unsure. → saved to trip log |  |  |  |  |  |  |  |  | ✓ |  |  |  |  |  |
| `origin_departure_date` | Date the arriving flight departs from its origin | E-ticket: the departure date of the flight that lands in the destination (can be the day before arrival). → saved to trip log |  |  |  |  |  |  |  |  |  | ✓ |  |  |  |  |
| `companions` | Travel companions on this trip | Other passengers on the same e-ticket / booking -> memory 'usually travels with'. → saved to trip log | ✓ |  | ✓ | ✓ |  | ✓ |  |  |  | ✓ |  |  |  | ✓ |

## ✍️ ASK: declarations, asked fresh on every card, never saved

| Key | Question | Source / saved to | CN | ID | IN | JP | KH | KR | MV | MY | NZ | PH | SG | TH | TW | VN |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `health_symptoms` | Do you currently have fever, cough, breathing difficulty, rash or other symptoms of an infectious disease? | Ask every time; never saved |  | ✓ |  |  | ✓ |  |  |  |  | ✓ | ✓ | ✓ |  |  |
| `recent_travel_yf` | Have you recently been in or passed through a yellow-fever-risk country (Singapore: Africa or Latin America in the last 6 days)? Which ones? | Ask every time; never saved |  |  |  |  |  |  | ✓ |  |  |  | ✓ |  |  |  |
| `recent_travel_2y` | Other countries or regions visited in the past two years (list them) | Ask every time; never saved. China asks this. The bot may *suggest* trips found in email/calendar as a starting list, but the user confirms. | ✓ |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `passport_lost` | Has a passport of yours ever been lost or stolen? | Ask every time; never saved | ✓ |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `signature` | Signature (drawn on the screen) | Ask every time; never saved. China only; drawn by the user during a Take control hand-over. | ✓ |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `other_passport_name` | Have you ever used a passport under a different name to enter this country? | Ask every time; never saved |  |  |  |  |  |  |  |  |  |  | ✓ |  |  |  |
| `deported_before` | Ever been deported from, or refused entry to, this country? | Ask every time; never saved |  |  |  | ✓ |  |  |  |  |  |  |  |  |  |  |
| `criminal_conviction` | Ever been convicted of a crime in any country? | Ask every time; never saved |  |  |  | ✓ |  |  |  |  |  |  |  |  |  |  |
| `prohibited_items` | Carrying narcotics, firearms, swords, explosives or other prohibited/restricted items? | Ask every time; never saved |  |  |  | ✓ |  |  |  |  | ✓ |  |  |  |  |  |
| `customs_family_count` | Number of family members travelling with you (customs) | Ask every time; never saved |  |  |  | ✓ |  |  |  |  |  |  |  |  |  |  |
| `customs_over_allowance` | Carrying goods beyond the duty-free allowance? | Ask every time; never saved |  |  |  | ✓ |  |  |  |  | ✓ |  |  |  |  |  |
| `customs_commercial` | Carrying commercial goods or samples? | Ask every time; never saved |  |  |  | ✓ |  |  |  |  | ✓ |  |  |  |  |  |
| `customs_for_others` | Carrying items on behalf of someone else? | Ask every time; never saved |  |  |  | ✓ |  |  |  |  | ✓ |  |  |  |  |  |
| `customs_cash` | Carrying cash / securities / precious metals above the declaration threshold? | Ask every time; never saved |  |  |  | ✓ | ✓ |  |  |  | ✓ |  |  |  |  |  |
| `customs_unaccompanied` | Do you have unaccompanied baggage (shipped separately)? | Ask every time; never saved |  |  |  | ✓ |  |  |  |  |  |  |  |  |  |  |
| `customs_restricted` | Carrying meat products, vegetables, fruit, animals or plants (restricted items)? | Ask every time; never saved |  | ✓ |  | ✓ |  |  |  |  |  |  |  |  |  |  |
| `customs_gold` | Carrying gold bullion or products of gold? | Ask every time; never saved |  |  |  | ✓ |  |  |  |  |  |  |  |  |  |  |
| `customs_goods_detail` | Itemised goods to declare (alcohol bottles, cigarettes, perfume, other goods with value) | Ask every time; never saved. Only when a customs question is answered Yes. |  | ✓ |  | ✓ |  |  | ✓ |  |  | ✓ |  |  |  |  |
| `recent_travel_14d` | Countries / territories where you stayed within two weeks before arrival (list them) | Ask every time; never saved. Thailand asks this. The bot may suggest trips found in email/calendar as a starting list, but the user confirms. |  |  |  |  | ✓ |  |  |  |  |  |  | ✓ |  |  |
| `yellow_fever_cert` | Are you vaccinated against yellow fever, with a certificate (and its vaccination date)? | Ask every time; never saved. Cambodia always asks. Thailand asks only when a visited country, the boarding country or the nationality is on its yellow-fever list. |  |  |  |  | ✓ |  | ✓ |  |  |  |  | ✓ |  |  |
| `recent_travel_21d` | Country of origin, transit and other countries visited within 21 days before arrival (list them) | Ask every time; never saved. Indonesia asks this; at least the country the flight left from. The bot may suggest trips found in email/calendar, but the user confirms. |  | ✓ |  |  |  |  |  |  |  |  |  |  |  |  |
| `other_passport_nationality` | Have you ever used a passport of a different nationality to enter this country? | Ask every time; never saved |  | ✓ |  |  |  |  |  |  |  |  |  |  |  |  |
| `baggage_count` | Number of bags arriving with you (checked and carry-on) | Ask every time; never saved |  | ✓ |  |  |  |  | ✓ |  |  |  |  |  |  |  |
| `customs_declarable` | Do you carry goods that must be declared to Customs (restricted items, cash over the limit, alcohol/tobacco or purchases over the allowance, trade goods)? | Ask every time; never saved |  | ✓ |  |  | ✓ |  | ✓ |  |  | ✓ |  |  |  |  |
| `imei_devices` | Bringing phones or tablets bought abroad to use on local mobile networks (IMEI registration)? | Ask every time; never saved. Indonesia: not required for foreign visitors staying under 90 days, but the question must still be answered. |  | ✓ |  |  |  |  |  |  |  |  |  |  |  |  |
| `ebola_exposure` | Ebola check: been in an affected country, in contact with a case or a funeral, or have symptoms? | Ask every time; never saved. Maldives shows this block only for certain origins. |  |  |  |  |  |  | ✓ |  |  |  |  |  |  |  |
| `recent_travel_30d` | Countries or territories you have been to in the last 30 days, including in transit (list them) | Ask every time; never saved. New Zealand asks this. The bot may suggest trips found in email/calendar, but the user confirms. |  |  |  |  |  |  |  |  | ✓ | ✓ |  |  |  |  |
| `medical_purpose` | Are you coming for medical treatment or consultation, or to give birth? | Ask every time; never saved |  |  |  |  |  |  |  |  | ✓ |  |  |  |  |  |
| `deported_any_country` | Have you ever been deported, removed, excluded or refused entry from any country? | Ask every time; never saved |  |  |  |  |  |  |  |  | ✓ |  |  |  |  |  |
| `prison_12_months` | Have you ever been sentenced to 12 months or more in prison? | Ask every time; never saved |  |  |  |  |  |  |  |  | ✓ |  |  |  |  |  |
| `bio_food` | Bringing any food: cooked, uncooked, fresh, preserved, packaged or dried? (Which kinds?) | Ask every time; never saved |  |  |  |  |  |  |  |  | ✓ |  |  |  |  |  |
| `bio_animal_products` | Bringing animals or animal products: meat, dairy, fish, honey, eggs, feathers, shells, wool, skins, bones or insects? | Ask every time; never saved |  |  |  |  |  |  |  |  | ✓ |  |  |  |  |  |
| `bio_plant_products` | Bringing plants or plant products: fruit, flowers, seeds, wood, nuts, vegetables, bamboo or straw? | Ask every time; never saved |  |  |  |  |  |  |  |  | ✓ |  |  |  |  |  |
| `bio_biological` | Bringing biological products: animal medicines, cultures, organisms, soil or water? | Ask every time; never saved |  |  |  |  |  |  |  |  | ✓ |  |  |  |  |  |
| `bio_equipment` | Bringing equipment or clothing used with animals, plants or water (gardening, beekeeping, fishing, water sport, diving)? | Ask every time; never saved |  |  |  |  |  |  |  |  | ✓ |  |  |  |  |  |
| `bio_outdoor_gear` | Bringing used outdoor gear: footwear, tents, camping, hunting, hiking, golf or sports equipment? (Used on farms or in the wild?) | Ask every time; never saved |  |  |  |  |  |  |  |  | ✓ |  |  |  |  |  |
| `bio_farm_contact` | In the past 30 days, visited wilderness areas, had contact with animals (not pet cats and dogs), or visited farms or plant or animal processing sites? | Ask every time; never saved |  |  |  |  |  |  |  |  | ✓ |  |  |  |  |  |
| `customs_medicine` | Carrying more than 3 months' supply of medicine, or medicine not prescribed to you? | Ask every time; never saved |  |  |  |  |  |  |  |  | ✓ |  |  |  |  |  |
| `customs_alcohol` | Carrying alcohol over the allowance (NZ: 3 bottles of spirits and 4.5 litres of wine or beer)? | Ask every time; never saved |  |  |  |  |  |  |  |  | ✓ |  |  |  |  |  |
| `customs_tobacco` | Carrying tobacco over the allowance (NZ: 50 cigarettes or 50 grams)? | Ask every time; never saved |  |  |  |  |  |  |  |  | ✓ |  |  |  |  |  |
| `baggage_known` | Do you know the contents of your baggage? | Ask every time; never saved |  |  |  |  |  |  |  |  | ✓ |  |  |  |  |  |
| `recent_travel_6d` | Countries visited in the last 6 days before arrival (list them, or none) | Ask every time; never saved. India asks this. |  |  | ✓ |  |  |  |  |  |  |  |  |  |  |  |
| `sick_contact` | In the past 30 days, any exposure to a person who is sick or known to have a communicable disease? | Ask every time; never saved |  |  |  |  |  |  |  |  |  | ✓ |  |  |  |  |

