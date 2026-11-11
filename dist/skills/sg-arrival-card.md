---
skill: sg-arrival-card
description: Walk the Singapore SG Arrival Card + health declaration on the official ICA site, field by field, from values resolved by arrival-card-fill. Stops at Review for the user's 'submit'.
---

# Skill: sg-arrival-card

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for an arrival in Singapore inside the
filing window. Never run it directly from a user request without the gap form first.

## 2. Required inputs and access

- Every 🔐 VAULT and 🧠 RECALL field in the table below has a value, and every ✍️ ASK field has an answer the user
  gave in this conversation.
- Browser on the cloud computer. No login is needed.

## 3. Sequence of work

1. Open `https://eservices.ica.gov.sg/sgarrivalcard/` in a new tab. Check the address bar says
   `eservices.ica.gov.sg`.
2. Choose **Fill in Manually** under the foreign-visitor card, not the resident one.
3. Fill **Traveller Details** in table order. Then:
   - Choose radios by their visible text, because their ids change every session.
   - Dates are three boxes: type DD, MM, YYYY.
   - For look-ups (nationality, places, cities, flight code, hotel), type the start of the value and click the
     matching row. A typed value that isn't picked from the list doesn't count.
   - If the page shows a health or outbreak question not in the table, stop. Ask the user that question
     word-for-word and use their answer.
4. For several travellers, use **Add traveller** and repeat step 3 for each one.
5. Press **Next** and fill **Visit Details**. Use the accommodation branch that matches `stay_type`. If the hotel
   isn't in the list, choose OTHERS and type the hotel name.
6. Press **Next** to reach **Review**. Stop there and follow step 7 of `arrival-card-fill`: screenshot, summary,
   wait for "submit".
7. After "submit": tick the declaration, press **Submit**, and capture the confirmation page. ICA emails an e-Pass
   to the card email. Find it in Gmail (`from:ica.gov.sg newer_than:1h`) and forward it if it went to a different
   address.

### Card facts: SG Arrival Card + electronic Health Declaration (ICA)

- **Official URL (only this one):** https://eservices.ica.gov.sg/sgarrivalcard/
- **Filing window:** Within 3 days before arrival, including the arrival day (30 Jun arrival → from 28 Jun). Up to 3 trips per submission.
- **Who files:** Every traveller entering Singapore, citizens and residents included (residents file the resident version).
- **Exempt:** Transit/transfer passengers who do not clear immigration; residents arriving via Woodlands or Tuas land checkpoints.
- **Last verified:** 2026-11-11: live form i18n (eservices.ica.gov.sg/arrivalcard/assets/i18n/en.json), MOH health-declaration notices; field map from a live walk 2026-10-31. Aug 2026 refresh added passport-MRZ scan autofill (not used by the bot).
- **Always the user's (hand over):**
  - Any CAPTCHA shown before submission.
  - The declaration tick and Submit on the Review step (user replies 'submit').

### Fields, page by page

Tier: 🔐 VAULT = traveller's Vault entry · 🧠 RECALL = memory → email → calendar → derived · ✍️ ASK = ask every time, never saved.

#### Landing

Choose the foreign-visitor 'Fill in Manually' button (the resident card has one too; pick the visitor one). Radio ids change every session: choose radios by their visible text. Dates are three typed boxes (DD / MM / YYYY). Look-up fields accept only listed values: type, then pick the row.

#### 1 · Traveller Details

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Date of Arrival | `arrival_date` | 🧠 RECALL | yes | Radio of the open dates, e.g. '03 DECEMBER 2026' |
| Full Name (In Passport) | `full_name` | 🔐 VAULT | yes |  |
| Passport Number | `passport_no` | 🔐 VAULT | yes |  |
| Date of Passport Expiry | `passport_expiry` | 🔐 VAULT | yes | DD / MM / YYYY |
| Sex as indicated in Passport | `sex` | 🔐 VAULT | yes | MALE / FEMALE / OTHERS |
| Date of Birth | `dob` | 🔐 VAULT | yes | DD / MM / YYYY |
| Nationality/Citizenship | `nationality` | 🔐 VAULT | yes | Look-up by demonym, e.g. SINGAPOREAN, AUSTRALIAN |
| Country/Place of Birth | `birth_country` | 🔐 VAULT | yes | Look-up, country name in caps |
| Place of Residence | `home_city` | 🔐 VAULT | yes | Look-up 'COUNTRY, STATE, CITY (COUNTRY)': type the home city, pick 'CITY (COUNTRY)' |
| Email Address | `email` | 🔐 VAULT | yes | The e-Pass confirmation goes here |
| Country Code + Mobile Number | `mobile` | 🔐 VAULT | yes |  |
| Have you ever used a passport under different name to enter Singapore? | `other_passport_name` | ✍️ ASK | yes | YES / NO; YES adds 'Name different from current passport' |
| Do you currently have fever, cough, shortness of breath, headache or vomiting? | `health_symptoms` | ✍️ ASK | yes | YES / NO (health declaration) |
| Have you visited any of the listed countries in Africa or Latin America in the past 6 days prior to your arrival in Singapore? | `recent_travel_yf` | ✍️ ASK | yes | YES / NO; show the user the site's country list. MOH may add temporary outbreak questions: read the page and ask any extra question verbatim. |

#### 2 · Visit Details (Trip Information)

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Last City/Port of Embarkation before Singapore | `origin` | 🧠 RECALL | yes | Look-up, city in caps |
| Purpose of Travel | `purpose` | 🧠 RECALL | yes | Business/Meeting/Conference… · Education/Training · Employment · Official visit · Holiday/Sightseeing/Leisure · Medical Care · Sports Event · Religion · To take up residence · Visiting Friends/Relatives · 1-day Transit/VFTF · Others |
| Date of Departure from Singapore | `departure_date` | 🧠 RECALL | yes | DD / MM / YYYY |
| Next City/Port of Disembarkation after Singapore | `next_destination` | 🧠 RECALL | yes | Look-up, city in caps |
| Mode of Travel / Mode of Transport | `arrival_flight` | 🧠 RECALL | yes | AIR → COMMERCIAL FLIGHT (or Private/Cargo/Others); LAND → vehicle; SEA → vessel |
| Flight Code + Flight Number | `arrival_flight` | 🧠 RECALL | yes | Code look-up by '(SQ)', number digits |
| Type of Accommodation in Singapore | `stay_type` | 🧠 RECALL | yes | HOTEL / RESIDENTIAL / TRANSIT / DAY TRIP / OTHERS |
| Name of Hotel | `stay_name` | 🧠 RECALL | if HOTEL | Look-up by the first words of the name; 'OTHERS' + typed name if not listed |
| Postal Code / Block / Street / Floor / Unit | `stay_address` | 🧠 RECALL | if RESIDENTIAL | Split '10 Bayfront Avenue #12-05, Singapore 018956' into block 10, street, floor 12, unit 05, postal 018956 |

#### 3 · Review (stop here)

Screenshot, send the summary, wait for 'submit'. Then tick the declaration (true information; false health declarations are an offence under the Infectious Diseases Act) and Submit. The e-Pass email follows; forward it.

Sources: https://www.ica.gov.sg/enter-transit-depart/entering-singapore/sg-arrival-card · https://moh.gov.sg/newsroom/health-declaration-supports-public-health-surveillance-efforts-at-borders · https://www.changiairport.com/en/fly/arrival-guide/immigration.html

## 4. How to validate the result

- The confirmation page shows a submission reference, and the e-Pass email arrived.
- The date of arrival on the confirmation matches the flight's **local** arrival date.

## 5. What to return

Reference number, travellers covered, the e-Pass email status, and that the passport alone is shown at the gate.
The card is linked to the passport, so there's no QR to present.

## 6. What requires approval

- The final **Submit**.
- Any answer to a health question that the user didn't give in this chat.
