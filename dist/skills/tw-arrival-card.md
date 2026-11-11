---
skill: tw-arrival-card
description: Walk the Taiwan Arrival Card (TWAC) on the official NIA site, field by field, from values resolved by arrival-card-fill, reading the email verification code from Gmail. Stops at Review for the user's 'submit'.
---

# Skill: tw-arrival-card

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for an arrival in Taiwan within 7 days.

## 2. Required inputs and access

- Every 🔐 VAULT and 🧠 RECALL field below has a value, including **`departure_flight`**. The site requires the
  flight leaving Taiwan, so never invent it. If there's no onward ticket, it must have been asked on the form.
- Gmail, to read the verification code, when the card email is the connected mailbox.

## 3. Sequence of work

1. Open `https://twac.immigration.gov.tw/`. Click **Submit Arrival Card**, then **START** on the notice page.
2. **Email Verification.** Type the card email, then press **SEND VERIFICATION CODE**. Search Gmail for
   `from:niasys@immigration.gov.tw subject:"TAIWAN ARRIVAL CARD" newer_than:1h`. The code is 6 letters, mixed
   case, and is valid for 10 minutes. Type it and press **VERIFY**. If the email isn't the connected mailbox, ask the
   user for the code. If a visible Turnstile challenge appears, use Take control.
3. **Traveler Information.**
   - Dates are DD/MM/YYYY.
   - For country pickers, type the ISO3 code and pick the `ISO3,NAME` row.
   - For the phone code picker, pick the `65 SGP` style row.
   - Map occupation to the closest entry on the fixed list, and note it as "chosen for you".
   - Visa type is Visa-Exempt for visa-exempt nationalities. Otherwise use the visa from the Vault's `visa_TW=` line.
4. Press **NEXT**. If a dialog names empty fields, fill them, press **CONFIRMATION**, and retry.
5. **Itinerary.**
   - Arrival: AIR, then the flight code from the picker (`SQ : SINGAPORE AIRLINES` style) and the digits.
   - Departure: AIR, then the same pattern for the exit flight.
   - Purpose: from the option list. "Visit Relative" requires the relative's name and mobile.
   - Accommodation: Hotel Name with only letters, digits, spaces and `. , -`, in upper case.
6. Press **NEXT** to reach **Review**. Stop there and follow step 7 of `arrival-card-fill`.
7. After "submit": tick the declaration, press **SUBMIT**, then capture the confirmation and the confirmation email.

### Card facts: Taiwan Arrival Card (TWAC)

- **Official URL (only this one):** https://twac.immigration.gov.tw/
- **Filing window:** Within 7 days before arrival, including the arrival day (official FAQ; paper cards ended 1 Oct 2025). One entry per card; a later submission overrides an earlier one. Up to 10 travellers per web submission.
- **Who files:** Visa-exempt and visa-holding foreign visitors.
- **Exempt:** ARC, Resident Visa or Diplomatic ID holders; airside transit passengers who do not pass immigration. No health or travel-history questions on the card (Taiwan CDC declaration only for travellers with symptoms in the past 14 days).
- **Last verified:** 2026-11-11: official FAQ + group-upload template in the JS bundle; 2026-11-02: live site walk of twac.immigration.gov.tw + JS bundle option lists.
- **Always the user's (hand over):**
  - Email verification code (6 letters, valid 10 min) from niasys@immigration.gov.tw: read from Gmail when the card email is the connected mailbox, else the user types it.
  - Cloudflare Turnstile if it shows a visible challenge.
  - The declaration tick and SUBMIT on the Review step (user replies 'submit').

### Fields, page by page

Tier: 🔐 VAULT = traveller's Vault entry · 🧠 RECALL = memory → email → calendar → derived · ✍️ ASK = ask every time, never saved.

#### Landing → Notice → 1 · Email Verification

Click 'Submit Arrival Card', then START on /notice. Enter the email, press SEND VERIFICATION CODE, read the code (Gmail search: from:niasys@immigration.gov.tw subject:"TAIWAN ARRIVAL CARD" newer_than:1h), type it, press VERIFY.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Email | `email` | 🔐 VAULT | yes |  |

#### 2 · Traveler Information

Dates are DD/MM/YYYY (react-datepicker). Country lists are react-select searches: type the ISO3 code and pick 'AUS,AUSTRALIA' style rows. NEXT shows a dialog naming any empty required field.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Date of Entry | `arrival_date` | 🧠 RECALL | yes | DD/MM/YYYY |
| Expected Departure Date | `departure_date` | 🧠 RECALL | yes | DD/MM/YYYY, ≥ entry date |
| English Name | `full_name` | 🔐 VAULT | yes | Given names then surname, letters and spaces only (site upper-cases) |
| Chinese Name | `chinese_name` | 🔐 VAULT | no |  |
| Passport Number | `passport_no` | 🔐 VAULT | yes |  |
| Passport Expiry | `passport_expiry` | 🔐 VAULT | yes | DD/MM/YYYY; must be valid ≥6 months |
| Gender | `sex` | 🔐 VAULT | yes | Radio Female / Male |
| Date of Birth | `dob` | 🔐 VAULT | yes | DD/MM/YYYY |
| Nationality | `nationality` | 🔐 VAULT | yes | react-select 'ISO3,NAME' |
| Country / Place of Birth | `birth_country` | 🔐 VAULT | yes | react-select 'ISO3,NAME' |
| City / State of birth | `birth_city` | 🔐 VAULT | no |  |
| Place of Residence | `home_country` | 🔐 VAULT | yes | react-select 'ISO3,NAME' |
| Visa type | `entry_policy` | 🧠 RECALL | yes | Radio: Visa-Exempt / Visa / Landing visa / Travel authorization; a visa number is then required |
| Visa / permit number | `visa` | 🔐 VAULT | if Visa type is not Visa-Exempt |  |
| Country / Region Code + Mobile Number | `mobile` | 🔐 VAULT | yes | Code react-select '65 SGP'; number digits only |
| Occupation | `occupation` | 🔐 VAULT | yes | Fixed list (e.g. 10 BUSINESS/DIRECTOR/CEO, 2 CLERK/EMPLOYEE/STAFF, 29 ACCOUNTANT/CPA, 24 Other → job title box); map the Vault value to the closest |

#### 3 · Itinerary

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Arrival mode | `arrival_flight` | 🧠 RECALL | yes | Radio AIR / SEA |
| Entry flight code + flight number | `arrival_flight` | 🧠 RECALL | yes | react-select 'CI : CHINA AIRLINES' style, then digits (max 10) |
| Departure mode | `departure_flight` | 🧠 RECALL | yes | Radio AIR / SEA (required) |
| Exit flight code + flight number | `departure_flight` | 🧠 RECALL | if Departure mode AIR | Required with AIR; never invented: ask if no onward ticket |
| Purpose of Visit | `purpose` | 🧠 RECALL | yes | 1 Sightseeing/Travel/Leisure, 16 Business, 213 Study, 60 Exhibition, 3 Visit Relative (+ relative's name & phone), 33 Others (+ text) |
| Relatives Name / Relatives Mobile No. | `host_contact` | 🧠 RECALL | if Purpose = Visit Relative |  |
| Accommodation type | `stay_type` | 🧠 RECALL | yes | Radio Residential Address / Hotel Name / Transfer |
| Address / hotel name | `stay_name` | 🧠 RECALL | yes | Letters, digits, spaces and . , - only, upper case; a residential address is geocoded by the site |

#### 4 · Review (stop here)

Screenshot, send the summary, wait for 'submit'. Then tick the declaration and press SUBMIT. Confirmation arrives by email; forward it.

Sources: https://twac.immigration.gov.tw/

## 4. How to validate the result

The confirmation step shows success, and the confirmation email arrives from the NIA. A later submission for the
same entry overrides the earlier one, so a correction means submitting again.

## 5. What to return

Confirmation status, travellers covered, the forwarded confirmation email, and that the passport is shown at
immigration. e-Gate enrolment is a separate counter.

## 6. What requires approval

- The final **SUBMIT**.
- Typing a verification code the user read out. Do it only for this card.
