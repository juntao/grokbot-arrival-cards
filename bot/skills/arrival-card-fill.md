---
skill: arrival-card-fill
description: Fill out an official online arrival card for any of 14 countries: China, Singapore, Japan, Taiwan, Korea, Thailand, Malaysia, Indonesia, Cambodia, the Philippines, India, the Maldives, New Zealand or Vietnam. First search Vault, memory, email and calendar. Then ask only for what's missing in one form, save the answers back, and drive the official site in the browser up to the final Submit.
---

# Skill: arrival-card-fill

## 1. When to use it

The user asks to fill out, file, prepare or "do" an arrival card, entry card, SG Arrival Card, SGAC, Visit Japan Web,
VJW, TWAC, Korea e-Arrival Card, TDAC, MDAC, All Indonesia, eTravel, IMUGA, NZTD, Vietnam pre-arrival or "China entry card". This also applies when they ask what a trip still
needs before landing in any of the 14 supported countries, and when a routine created by this skill fires on the day a card's window opens.

Don't use it for visas or e-visas, the UK ETA, K-ETA or other countries. For those, say they're not supported yet.
A traveller with a valid K-ETA doesn't file the Korea card: see `kr-arrival-card`.

## 2. Required inputs and access

- **Gmail** connected app. Without it, skip the email search and say so on the form.
- **Google Calendar** connected app. Without it, skip the calendar search and say so.
- **Vault**: one Contact info entry per traveller, named `Travel identity · <GIVEN> <SURNAME>` (format below), and a
  Login for `vjw.digital.go.jp` (Japan) or `etravel.gov.ph` (Philippines).
- **Browser** on the cloud computer.
- The trip: which country or countries, and roughly when. If the user didn't say, take the next trip found in
  calendar and email.

### The Vault identity entry (canonical format)

The Vault has Contact info with four native fields (name, address, phone, email), plus a nickname and **Hints**.
Use them like this, one entry per traveller:

| Vault slot | Holds |
|---|---|
| Nickname | `Travel identity · GIVEN SURNAME` |
| Name | Full name exactly as printed in the passport (Latin letters) |
| Address | Home address: street, city, postcode, country (optional; cards use `home_city` / `home_country`) |
| Phone | Mobile with country code, e.g. `+65 9123 4567` |
| Email | The email the traveller wants on arrival cards |
| Hints | One `key=value` per line, using exactly the keys below |

Hints keys, written in this order, and only the ones known:

```
use=arrival cards; travel identity of GIVEN SURNAME
surname=TAN
given_names=MEI LING
sex=F                      # F | M | X (as in passport)
dob=1990-04-23
nationality=SGP            # ISO 3166 alpha-3
birth_country=SGP
birth_city=SINGAPORE
passport_no=K1234567X
passport_country=SGP
passport_expiry=2031-05-31
home_city=SINGAPORE
home_country=SGP
occupation=SOFTWARE ENGINEER
employer=ACME PTE LTD
chinese_name=陈美玲         # optional, used by TW
visa_CN=none               # or the visa number; per destination: visa_JP, visa_TW, visa_KR, visa_TH, visa_ID, visa_NZ, visa_VN
```

If a newer Grok Bot version offers a dedicated identity or passport entry type in Vault, use that for the passport
fields and keep the rest as above.

## 3. Sequence of work

### Step 1: Identify the trip

1. Read memory for an open trip log (`Trip <id>`) that matches the request.
2. Search **calendar** from today to 60 days ahead. Look for events whose title or location contains the
   destination country or city names, "flight", "✈", an airline code plus number, or "hotel".
3. Search **Gmail** with the queries below, newest first, limited to the last 180 days. Open only messages that
   match the trip's dates or destination.
   - Flights: `(e-ticket OR "booking confirmation" OR itinerary OR "boarding pass" OR "flight confirmation") (SIN OR Singapore OR PVG OR PEK OR PKX OR Shanghai OR Beijing OR NRT OR HND OR KIX OR Tokyo OR Osaka OR TPE OR Taipei OR ICN OR GMP OR Seoul OR PUS OR Busan OR BKK OR DMK OR Bangkok OR HKT OR Phuket OR CNX OR KUL OR "Kuala Lumpur" OR PEN OR Penang OR CGK OR Jakarta OR DPS OR Denpasar OR Bali OR PNH OR KTI OR "Phnom Penh" OR "Siem Reap" OR MNL OR Manila OR CEB OR Cebu OR DEL OR Delhi OR BOM OR Mumbai OR BLR OR MLE OR Maldives OR AKL OR Auckland OR CHC OR ZQN OR SGN OR "Ho Chi Minh" OR HAN OR Hanoi OR DAD OR "Da Nang" OR CXR OR PQC OR "Phu Quoc")`
   - Stays: `(hotel OR booking.com OR agoda OR airbnb OR expedia OR trip.com OR reservation) (confirmation OR confirmed OR itinerary)`
   - Business purpose: `(invitation OR "invitation letter" OR agenda OR conference) <destination>`
4. Build the trip: the list of entries in order (country, arrival date, airport, flight), plus the stay per country
   and the onward or return flight. Give it an id `<CC>-<YYYYMMDD>`, for example `JP-20261203`.
5. If more than one trip fits, put the choice as question 0 on the form. Don't guess.

### Step 2: Resolve every field for each card and each traveller

For each card in the trip, take its field table from the card's skill (`cn-arrival-card`, `sg-arrival-card`,
`jp-visit-japan-web`, `tw-arrival-card`, `kr-arrival-card`, `th-arrival-card`,
`my-arrival-card`, `id-arrival-card`, `kh-arrival-card`, `ph-etravel`, `in-arrival-card`,
`mv-traveller-declaration`, `nz-traveller-declaration`, `vn-prearrival`). For each traveller, resolve every field through its tier:

| Tier | Meaning | Look in, in order | Saved to |
|---|---|---|---|
| 🔐 **VAULT** | Identity, stable for years | Vault entry → passport photo (kept copy, or attached in this chat) | Vault entry |
| 🧠 **RECALL** | Trip facts and preferences | Trip log in memory → email → calendar → derive (rules in the table) → memory preference | Trip log (per trip) or memory (preference) |
| ✍️ **ASK** | Declarations, answered fresh each time | Nothing. Only an answer the user gave in **this** chat for **this** trip | Never saved |

If a traveller has **no** Vault entry yet, offer `arrival-card-onboarding` first. A passport photo fills most
🔐 fields at once. If the user prefers, just put the missing 🔐 fields on this form.

Travellers: the user, plus anyone named on the same e-ticket or booking, plus anyone memory says "usually travels
with me". Confirm the traveller list on the form when it's more than one person.

Record each field as `value` + `source` (vault, memory, email: subject and date, calendar: event and date, derived,
or the user), or as **missing**.

### Step 3: Build and send the gap form

Questions go on the form in this order:

1. **Missing VAULT fields**, grouped per traveller, marked 🔐 with *"saved to your Vault for next time"*.
2. **Missing RECALL fields**, marked 🧠. Show the best guess as the default when there is one, e.g.
   *"Purpose: [Tourism] / Business / Visiting family / Transit / Other"*.
3. **Every ASK field**, marked ✍️ *"declaration, asked every trip"*. Show the exact wording and options used on
   the official site. Never pre-select an answer.
4. **Low-confidence values.** Two candidate hotels, a flight changed in a later email, a stale passport expiry:
   *"Confirm: arriving on SQ 638 on 3 Dec? [Yes] / No, it's …"*.

Put one short "**Already known**" block above the form. It lists each found value with its source, the passport
shown as `•••••789`, so the user can correct anything. Example:

```
Arrival card: Japan (Visit Japan Web) · trip JP-20261203 · travellers: Mei Ling TAN

Already known
  🔐 Name, passport •••••67X (exp 2031-05-31), DOB, nationality: Vault
  🧠 Flight SQ 638, SIN→NRT, 3 Dec 2026 07:35: email "Your e-ticket" 2 Nov
  🧠 Hotel Park Hyatt Tokyo, 3-7-1-2 Nishishinjuku, +81 3-5322-1234: email "Booking confirmed" 4 Nov
  🧠 Departure 8 Dec 2026: calendar "Fly home"

Please answer (reply with the numbers):
  1. 🔐 Occupation (saved to your Vault for next time):
  2. 🧠 Purpose of visit [Tourism] / Business / Visiting relatives / Other:
  3. ✍️ Have you ever been deported from or refused entry to Japan? Yes / No
  4. ✍️ Ever convicted of a crime in any country? Yes / No
  ...
```

If nothing is missing and there are no ASK fields, skip the form. Show "Already known" and ask "Fill it in now?".

### Step 4: Save the answers back

Do this immediately after the user replies, before opening the browser:

- **VAULT answers**: update the traveller's Contact info entry. Put values in the native slots or as `key=value`
  hint lines, and create the entry if it doesn't exist. Tell the user in one line what was saved, with no passport
  number in clear. Never delete an existing line. A changed value replaces its line.
- **RECALL answers**: write the trip log to memory as
  `Trip <id> (<date>): <card> <field>=<value>; …`. A *lasting* preference ("I always travel for business", "my
  occupation on cards is 'Engineer'") also goes in as a memory preference.
- **ASK answers**: keep them only in this conversation. Never in memory, Vault or files.

### Step 5: Check the filing window

| Card | Opens | Notes |
|---|---|---|
| SG | 3 days before arrival (arrival day included) | Earlier submissions are refused |
| TW | 7 days before arrival (arrival day included) | A later submission overrides an earlier one |
| KR | 3 days before arrival, Korea time (arrival day included) | Lapses 72 h after submission, so don't file earlier than needed |
| TH | 3 days before arrival (arrival day included) | Can't be saved part-way; needs the user for the Cloudflare check |
| MY | 3 days before arrival (arrival day included) | Needs the user for the slider puzzle; one traveller per submission |
| ID | 3 days before arrival | Not a visa; needs the user for the image CAPTCHA |
| KH | 7 days before arrival | Air arrivals only; needs the hotel's phone number |
| PH | 72 hours before arrival | Needs an eTravel account; the airline checks the QR code before boarding |
| IN | 72 hours before arrival | Short form; needs the user for the image CAPTCHA |
| MV | 96 hours before the arriving flight | Needs a face photo; earlier filings are rejected |
| NZ | 24 hours before the trip to New Zealand starts | About 20 declaration questions; 30-minute session limit |
| VN | 3 days before arrival (arrival day included) | Needs the user for the CAPTCHA at the start; asks for the visa or exemption |
| CN | Any time before arrival | Can also be done at the port |
| JP | Any time | Register at least 6 h before departure |

If a card isn't open yet, say so and offer to create a routine that runs this skill for the trip id at 09:00 local
on the day it opens. ASK fields are asked again then, because the user's answers might change.

### Step 6: Fill in the official site

Load the card's site skill and follow it page by page. General rules:

- Open **only** the official URL from the card skill, in a new tab. Never click ads or sponsored results.
- Fill from the resolved values. Transform values to the site's format (date format, upper case, ISO country
  name), as the card skill says.
- When a value isn't on a site's fixed option list, pick the closest option and list it as "chosen for you" in
  the review summary. If no option is close, stop and ask.
- On a validation error, read it, fix the value, and retry once. If it still fails, ask the user.
- For a CAPTCHA, Cloudflare check, passkey, 2-factor code not in Gmail, or anything needing a human: **Take control**
  hand-over. Say exactly what the user should do, then wait for "continue".
- For several travellers, use the site's "add traveller / group" feature when it has one. Otherwise file one card
  per traveller.

### Step 7: Review and submit

On the site's review page, take a screenshot and send the user a compact summary: every field and value, the
passport as `•••••789`, and what was "chosen for you". Then ask: **"Reply 'submit' to file it, or tell me what to
change."** Press Submit or Register only after that reply.

### Step 8: Record the result

- Save the confirmation page, PDF and QR to `/workspace/arrival-cards/<trip-id>/<CC>-<traveller>.png|pdf`.
- If a confirmation email arrives (most cards send one), forward it to the user.
- Write to memory: `Trip <id>: <CC> card submitted <date-time>, ref <reference>`.

## 4. How to validate the result

- The site shows a confirmation page or reference number, or (JP) the QR codes for immigration and customs.
- Every required field in the card's table has a value or an explicit user answer. Nothing was invented.
- Arrival date and flight on the card match the latest flight email (check for schedule-change emails).
- The passport expires more than 6 months after arrival. If not, warn before submitting.
- The Vault entry now holds every VAULT field the user answered.

## 5. What to return

1. The trip line: country, card, traveller(s), arrival.
2. The status: submitted (with reference), waiting on the user (what for), or scheduled (when).
3. Where the proof is saved, and whether a confirmation email was forwarded.
4. One line on what was saved to the Vault and memory this time.
5. What to show at the border: passport, plus the QR code (JP, CN, TH, ID, PH, MV, VN) or nothing extra (SG, TW, KR, MY, NZ: it's linked to the
   passport).

## 6. What requires approval

- The final **Submit / Register / Confirm** on any card. The user replies "submit" after the review summary.
- Creating a Visit Japan Web account, or any other account.
- Overwriting an existing Vault value the user didn't just give.
- Any payment, which should never happen: all fourteen cards are free.
