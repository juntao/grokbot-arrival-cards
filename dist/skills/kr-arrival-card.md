---
skill: kr-arrival-card
description: Walk the Korea e-Arrival Card on the official site, field by field, from values resolved by arrival-card-fill, reading the email verification code from Gmail. Stops at Check information for the user's 'submit'.
---

# Skill: kr-arrival-card

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for a foreigner arriving in Korea within
3 days (Korea time).

First check whether the traveller needs it at all. A traveller with a **valid K-ETA** who isn't entering on a
visa doesn't file this card: tell them to update their K-ETA record at k-eta.go.kr instead, and stop. Ask if you
don't know. K-ETA is a separate, paid application and this skill doesn't do it.

## 2. Required inputs and access

- Every 🔐 VAULT and 🧠 RECALL field below has a value. The card has no health or customs questions.
- A **contact number in Korea**: the traveller's Korean number, or the hotel's phone from the booking.
- Gmail, to read the verification code, when the card email is the connected mailbox.

## 3. Sequence of work

1. Open `https://www.e-arrivalcard.go.kr/` and set the language to English. Under **Individual**, choose
   **Submit**.
2. **Consent and email.** Tick the three boxes. Type the email in both boxes; the second one blocks paste. Confirm.
3. **Passport information.** The form takes about 15 seconds to open, under a passport-photo popup. Close the
   popup and enter the fields by hand, in table order. For several travellers, use **Add declarant** (up to 9; they
   share the arrival date and flight) and repeat.
4. **Travel itinerary.**
   - Arrival: AIR, the date from the datepicker, then the airline from **Check airline** and the flight number.
     Pad a short number to 3 digits (JQ53 becomes JQ053).
   - Departure: AIR, the date, and the flight number if known. An approximate date is accepted.
   - Purpose and Job: pick from the lists; note a mapped job as "chosen for you".
   - Place of stay: Hotel or Residential. Open the address-search popup, search the hotel's road name or
     building in English, and pick the matching row: the base address can't be typed. Then type the hotel name
     in the detailed address.
   - Contact Information: the number in Korea, digits only.
5. Go to **Check information**. Stop there and follow step 7 of `arrival-card-fill`.
6. After "submit": request the verification code and read it from Gmail (it's valid for 5 minutes; three wrong
   tries force a resend). If the email isn't the connected mailbox, ask the user for the code. Hand over for the
   CAPTCHA popup with Take control, then submit.
7. **Save the PDF with the issue number immediately.** The site's own notice says confirmation emails to Gmail
   and Outlook are sometimes delayed or lost.

### Card facts: Korea e-Arrival Card

- **Official URL (only this one):** https://www.e-arrivalcard.go.kr/
- **Filing window:** From 3 days before arrival (Korea time), arrival day included: the form accepts arrival dates from today to today+2. Lapses 72 hours after submission. Paper cards are still accepted on arrival.
- **Who files:** All foreign nationals entering Korea, including visa holders and travellers exempt from K-ETA.
- **Exempt:** Valid K-ETA holders (unless entering on a visa): they update their K-ETA record instead. Holders of a Korean residence card, permanent resident card or overseas Korean resident card; group-visa tourists; crew; airside transit passengers; Korean citizens.
- **Last verified:** 2026-11-11: form HTML, inline validators, guides and FAQ of e-arrivalcard.go.kr read directly. No health or customs questions on the card (Q-CODE and customs are separate and only for symptoms / controlled areas / goods over the allowance).
- **Always the user's (hand over):**
  - The CAPTCHA popup at submit.
  - The email verification code at submit (5-minute timer, three wrong tries forces a resend): read from Gmail when the representative email is the connected mailbox, else the user types it.
  - The final submit (user replies 'submit').

### Fields, page by page

Tier: 🔐 VAULT = traveller's Vault entry · 🧠 RECALL = memory → email → calendar → derived · ✍️ ASK = ask every time, never saved.

#### 1 · Consent and email

Home → Individual 'Submit'. Tick the three boxes (personal information, terms of use, 14 or older). The email is typed twice and the confirm box blocks paste: type it. The representative must be 14 or older; children are added as extra declarants.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Email / Confirm email | `email` | 🔐 VAULT | yes | Typed twice, max 100 characters. Confirmation emails to Gmail, Outlook and Hotmail are sometimes delayed: always save the PDF. |

#### 2 · Passport information

The form opens after about 15 seconds under a passport-photo popup: close it and enter manually. 'Add declarant' adds up to 9 people who share the arrival date and flight. A warning about duplicate details means an earlier card will be replaced.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Surname (as shown on passport) | `surname` | 🔐 VAULT | yes |  |
| Given name (as shown on passport) | `given_names` | 🔐 VAULT | yes |  |
| Date of birth | `dob` | 🔐 VAULT | yes | Year / Month / Day selects |
| country/region | `nationality` | 🔐 VAULT | yes | Searchable dropdown: type the country name and pick it |
| Passport number | `passport_no` | 🔐 VAULT | yes |  |
| Sex | `sex` | 🔐 VAULT | yes | Female / Male / Third gender |
| Passport expiration date | `passport_expiry` | 🔐 VAULT | yes | Year / Month / Day selects; must be after the arrival date |

#### 3 · Travel itinerary

There is no port-of-entry field, and 'previous departure place' / 'next destination' were removed in March 2026.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Means of arrival | `arrival_flight` | 🧠 RECALL | yes | AIR / SEA |
| Date of arrival | `arrival_date` | 🧠 RECALL | yes | YYYY-MM-DD datepicker; today to today+2 (Korea time) |
| Flight number | `arrival_flight` | 🧠 RECALL | yes | 'Check airline' picker, then code + 3-4 digits, e.g. KE000; pad short numbers (JQ53 → JQ053). 'Search travel information' looks the flight up. |
| Means of departure | `departure_flight` | 🧠 RECALL | yes | AIR / SEA |
| Date of departure | `departure_date` | 🧠 RECALL | yes | YYYY-MM-DD; an approximate date is accepted |
| Flight number (departure) | `departure_flight` | 🧠 RECALL | no |  |
| Purpose of entry | `purpose` | 🧠 RECALL | yes | Tourism (individual) · Tourism (group) · Business · Diplomacy/official duties · Treatment/Medical care · Visit (Family/relatives/friends, etc.) · Meeting/event · Employment · Studies · Sports game · Others (+ text) |
| Visa Information Lookup | `visa` | 🔐 VAULT | if Entering on a Korean visa | Visa number + Check; fills address and contact from the visa |
| Intended place of stay | `stay_type` | 🧠 RECALL | yes | Hotel / Residential / Same-day Transit (only when leaving the same or next day; skips the address) |
| Base address (address-search popup) | `stay_address` | 🧠 RECALL | yes | Not typed: open the KOR/ENG address search, search the road name or building, pick the row. It fills the 5-digit postal code. |
| Detailed address | `stay_name` | 🧠 RECALL | yes | Hotel name and room/unit, max 200 characters, Korean or English |
| Contact Information | `stay_phone` | 🧠 RECALL | yes | Digits only: a contact number in Korea. Use the traveller's Korean number if they have one, else the hotel's phone. |
| Job | `occupation` | 🔐 VAULT | yes | Office worker · Self-employed · Student · Unemployed · Household activities · Public official · Agriculture and livestock industry · Others (+ text); map the Vault value to the closest |
| Declarant Email | `companions` | 🧠 RECALL | if An added declarant wants their own copy |  |

#### 4 · Check information (stop here)

Screenshot, send the summary, wait for 'submit'. Then: verification code from Gmail (or the user), hand over for the CAPTCHA, submit. Save the PDF with the issue number straight away.

Sources: https://www.e-arrivalcard.go.kr/portal/main/index.do?locale=EN · https://www.e-arrivalcard.go.kr/portal/guide/eacTargetGuide.do

## 4. How to validate the result

The completion page shows an issue number for each traveller and offers the PDF. If a duplicate-details warning
appeared, the earlier card was replaced: say so.

## 5. What to return

Issue number(s), the saved PDF path, and what to show: the passport only, because the card is matched to it.
The card lapses 72 hours after submission. To change the purpose, stay or departure later, use **Check and Edit**
with the issue number, passport number and date of birth.

## 6. What requires approval

- The final submit.
- Proceeding past a duplicate-details warning, because it deletes an earlier card.
