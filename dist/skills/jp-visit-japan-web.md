---
skill: jp-visit-japan-web
description: Register a trip in Visit Japan Web (immigration Disembarkation Card + customs declaration, one QR), signing in with the Vault login, from values resolved by arrival-card-fill. Stops before each Register for the user's 'submit'.
---

# Skill: jp-visit-japan-web

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for an arrival in Japan. VJW is optional,
but it saves time at both counters.

## 2. Required inputs and access

- A Vault **Login** for `vjw.digital.go.jp`. If there isn't one, ask approval to create an account with the card
  email. Let Vault generate the password: at least 10 characters with upper case, lower case, a digit and a symbol.
- Gmail, for the account confirmation code.
- Every 🔐 VAULT and 🧠 RECALL field below has a value. All 3 immigration and 8 customs ✍️ ASK questions have
  answers from this conversation.

## 3. Sequence of work

1. Open `https://www.vjw.digital.go.jp/` and set the language to English. Sign in with the Vault login.
   - If 2-factor authentication is on, use Take control for the authenticator code.
   - For a new account, the "I am human" check is the user's (Take control). Read the email confirmation code
     from Gmail (`from:vjw newer_than:1h`, or the subject "Visit Japan Web").
2. **Your details.** Skip this if it's already registered and matches the Vault, but fix any mismatch.
   - Answer No to "Japanese passport" and No to "re-entry permit".
   - Use **manual entry**, because there's no camera on the cloud computer.
   - Type names in UPPER CASE, and the passport number twice.
   - Leave "Contact information in Japan" blank unless the user always stays at the same address.
   - Register each companion here as a family member, up to 10.
3. **Register planned entry/return.**
   - Trip name: the trip id.
   - Arrival date: mm/dd/yyyy.
   - Airline from the dropdown, and the flight number as digits only. For a codeshare, use the **operating**
     carrier. If the airline isn't listed, choose "Other" and type the full code.
   - Postal code as digits, then press the auto-fill button for the prefecture and city. Then the address in UPPER
     CASE romaji, the hotel name, and the hotel phone as digits.
   - Tick the travelling family members.
4. **Disembarkation Card for Foreigner.** Fill home country and city, purpose, last flight and length of stay
   (years / months / days). Answer the 3 Yes/No questions with the user's own answers. Stop before **Register**
   and follow step 7 of `arrival-card-fill`. Register after "submit".
5. **Customs declaration.**
   - Occupation: the closest dropdown option.
   - Date and flight.
   - Point of embarkation, from the type-ahead list.
   - Family counts by age band.
   - The 8 questions, answered from the user's answers. If any of ①–⑥ is Yes, fill the goods detail the user gave.
   - Stop before **Register**, then register after "submit". If ⑦ is Yes, tell the user they must also file a
     means-of-payment declaration at the counter.
6. Open the QR screen ("Display QR code"). Screenshot the combined immigration and customs QR for each traveller
   and save it.

### Card facts: Visit Japan Web (immigration + customs QR)

- **Official URL (only this one):** https://www.vjw.digital.go.jp/
- **Filing window:** Any time before landing; register at least 6 h before departure. Up to 5 planned trips per account.
- **Who files:** All arrivals (optional: paper ED card and customs form still accepted, but VJW gives one QR for both counters).
- **Exempt:** Nobody needs it; it is optional. Japanese passport holders skip the Disembarkation Card.
- **Last verified:** 2026-11-11: Digital Agency VJW manual v2.00 + Japan Customs C5360-B + 2026 guides; screens behind login not walked live.
- **Always the user's (hand over):**
  - Account creation: the 'I am human' CAPTCHA and the email confirmation code (the code can be read from Gmail).
  - Sign-in 2-factor (authenticator app) if the user enabled it.
  - Passport camera scan for tax-free shopping (phone only); skip it unless the user asks.
  - Final 'Register' on each of the immigration and customs declarations (user replies 'submit').

### Fields, page by page

Tier: 🔐 VAULT = traveller's Vault entry · 🧠 RECALL = memory → email → calendar → derived · ✍️ ASK = ask every time, never saved.

#### 1 · Sign in (or create account)

Use the Vault Login for vjw.digital.go.jp. No login yet: ask approval to create one with the card email; the password is generated and saved by Vault. The confirmation code arrives by email; read it from Gmail if it is the connected mailbox, else hand over.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Email address / Password | `vjw_login` | 🔐 VAULT | yes | Vault Login fill. New-account password: ≥10 chars with upper, lower, digit, symbol. |

#### 2 · Your details (once per account, per family member)

Answer 'No' to 'Japanese government-issued passport?' and 'No' to re-entry permit for a foreign visitor. Choose manual entry (not camera scan) on the cloud computer.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Surname | `surname` | 🔐 VAULT | yes | UPPER CASE Latin, max 39 chars |
| Given name | `given_names` | 🔐 VAULT | yes | UPPER CASE, include middle names |
| Date of birth | `dob` | 🔐 VAULT | yes | mm/dd/yyyy (or calendar picker) |
| Nationality | `nationality` | 🔐 VAULT | yes | Dropdown, country name |
| Gender | `sex` | 🔐 VAULT | yes | Dropdown |
| Passport number / Confirm passport number | `passport_no` | 🔐 VAULT | yes | Upper-case letters and digits, typed twice |
| Passport expiration date | `passport_expiry` | 🔐 VAULT | yes | Date |
| Contact information in Japan (optional default) | `stay_address` | 🧠 RECALL | no | Leave blank unless the user always stays at the same address in Japan |

#### 3 · Register planned entry/return

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Trip name | `arrival_date` | 🧠 RECALL | no | Use the trip id, e.g. JP-20261203 |
| Planned arrival date in Japan | `arrival_date` | 🧠 RECALL | yes | mm/dd/yyyy |
| Airline company name | `arrival_flight` | 🧠 RECALL | yes | Dropdown of airlines; 'Other airline companies' if not listed. Operating carrier for codeshares. |
| Flight number | `arrival_flight` | 🧠 RECALL | yes | Digits only for a listed airline; letters+digits (e.g. ZZ1234) for 'Other' |
| Postal code / Prefecture / City | `stay_address` | 🧠 RECALL | yes | Postal code digits only (no hyphen), then the auto-fill button for prefecture and city |
| Address | `stay_address` | 🧠 RECALL | yes | UPPER CASE romaji, e.g. 3-7-1-2 NISHISHINJUKU |
| Hotel name / place of stay | `stay_name` | 🧠 RECALL | yes | UPPER CASE |
| Contact phone number | `stay_phone` | 🧠 RECALL | yes | Digits only, no hyphens |
| Select family members traveling with you | `companions` | 🧠 RECALL | if Family members are registered on the account |  |

#### 4 · Disembarkation Card for Foreigner (immigration)

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Home address: Country name | `home_country` | 🔐 VAULT | yes | UPPER CASE country name |
| Home address: City name | `home_city` | 🔐 VAULT | yes | UPPER CASE |
| Purpose of visit | `purpose` | 🧠 RECALL | yes | Sightseeing / Business / Visiting relatives / … / Other (+ free text) |
| Last flight No./Vessel | `arrival_flight` | 🧠 RECALL | yes | Pre-filled from the trip; full code e.g. SQ638 |
| Intended length of stay | `length_of_stay` | 🧠 RECALL | yes | Years 0-5 / Months 0-12 / Days 0-180; not all zero |
| Have you ever been deported from Japan, departed under a departure order, or been denied entry to Japan? | `deported_before` | ✍️ ASK | yes | Yes / No |
| Any history of being convicted of a crime (not only in Japan)? | `criminal_conviction` | ✍️ ASK | yes | Yes / No |
| Do you presently have narcotics, controlled substances, firearms, crossbows, swords, explosives or other such items? | `prohibited_items` | ✍️ ASK | yes | Yes / No |

#### 5 · Customs declaration (Declaration of Personal Effects and Unaccompanied Articles)

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Occupation | `occupation` | 🔐 VAULT | yes | Dropdown (Employee, Self-employed, Student, …); map the Vault value to the closest option |
| Date of Arrival in Japan | `arrival_date` | 🧠 RECALL | yes |  |
| Flight No./Name of Vessel | `arrival_flight` | 🧠 RECALL | yes |  |
| Point of embarkation | `origin` | 🧠 RECALL | yes | Type-ahead city list, e.g. SINGAPORE |
| Family members traveling with you (Over 20 / 6-19 / Under 6) | `customs_family_count` | ✍️ ASK | yes | +/- counters |
| ① Narcotic drugs, firearms, explosives or other prohibited items? | `prohibited_items` | ✍️ ASK | yes | Yes / No |
| ② Meat products, vegetables, fruits, animals, plants or other restricted items? | `customs_restricted` | ✍️ ASK | yes | Yes / No |
| ③ Gold bullion or products of gold? | `customs_gold` | ✍️ ASK | yes | Yes / No |
| ④ Goods (purchases, souvenirs, gifts) exceeding the duty-free allowance? | `customs_over_allowance` | ✍️ ASK | yes | Yes / No |
| ⑤ Commercial goods or samples? | `customs_commercial` | ✍️ ASK | yes | Yes / No |
| ⑥ Items you have been requested by someone else to bring into Japan? | `customs_for_others` | ✍️ ASK | yes | Yes / No |
| ⑦ Cash, cheques or securities over ¥1,000,000 equivalent, or precious metals over 1 kg? | `customs_cash` | ✍️ ASK | yes | Yes / No (Yes requires a separate means-of-payment declaration at the counter) |
| ⑧ Unaccompanied articles (shipped separately)? | `customs_unaccompanied` | ✍️ ASK | yes | Yes / No + number of packages |
| Personal effects / other goods detail | `customs_goods_detail` | ✍️ ASK | if Any of ①-⑥ answered Yes | Alcohol (bottles), cigarettes (pieces), cigars, heat-not-burn (boxes), perfume (oz), other goods: name, quantity, value, currency (≤8 rows) |

Sources: https://www.digital.go.jp/assets/contents/node/basic_page/field_ref_resources/3e9afaa3-b2e7-4f6d-b07d-9697f39d97a3/73099bd6/20220926_en_visit_japan_web_manual_01.pdf · https://www.customs.go.jp/kaisei/youshiki/form_C/C5360-B.pdf · https://www.moj.go.jp/isa/content/930002134.pdf

## 4. How to validate the result

- Both the Disembarkation Card and the customs declaration show **Registered** (or "Completed") for each traveller.
- A QR code is shown. If anything is edited later, the old QR becomes invalid: take a new screenshot.

## 5. What to return

Registration status for each traveller, QR screenshots saved (paths), and a reminder: show the QR at the
immigration kiosk and again at customs. Paper forms remain a fallback. JESTA, the pre-travel authorisation, is
planned for FY2028 and isn't needed yet.

## 6. What requires approval

- Creating a VJW account.
- Each **Register** (immigration, then customs).
- Any Yes answer to a customs question, which the user must give explicitly.
