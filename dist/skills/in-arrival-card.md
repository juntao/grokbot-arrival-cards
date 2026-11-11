---
skill: in-arrival-card
description: Fill the India e-Arrival Card on the official Bureau of Immigration page, from values resolved by arrival-card-fill. Stops on the filled form for the user's 'submit' and the CAPTCHA.
---

# Skill: in-arrival-card

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for a foreign national arriving in India
within 72 hours. Indian citizens don't file it.

The card is not a visa, and it doesn't cover India's separate health self-declaration (Air Suvidha) or customs.
Tell the user to check whether those apply to them: this skill doesn't file them.

## 2. Required inputs and access

- Every 🔐 VAULT and 🧠 RECALL field below has a value. The form is short: it doesn't ask for the flight, date of
  birth or visa number.
- The user's answer, from this conversation, to **countries visited in the last 6 days**.
- An address in India with a street or hotel name, plus its state and district.

## 3. Sequence of work

1. Open `https://indianvisaonline.gov.in/earrival/`. Check the address: paid look-alike sites exist, and this
   card is free.
2. Fill the single page in table order.
   - Purpose: read the list from the page and pick the closest. If none fits, choose OTHER and type it.
   - Date of Arrival: pick it from the date picker. If the date can't be selected, the window isn't open: stop and
     say when to try again.
   - Countries visited: select the ones the user listed, or tick "No Country Visited".
   - State, then District.
3. For companions, use **Add Member** (up to five). They share the arrival date, address and contact details.
4. The form has no review page. Stop on the filled form and follow step 7 of `arrival-card-fill`.
5. After "submit": tick the declaration, use **Take control** for the image CAPTCHA, and submit.
6. A message shows the File Number for each traveller and a PDF downloads for each. Save them.

For a repeat visitor, "Quick Fill From Existing Data" at the top can reuse an earlier card. It needs its own
CAPTCHA, so only use it if the user prefers.

### Card facts: India e-Arrival Card

- **Official URL (only this one):** https://indianvisaonline.gov.in/earrival/
- **Filing window:** Within 72 hours before arrival; the date picker enforces the exact window. Some airlines ask for it at least 24 hours before the flight.
- **Who files:** All foreign nationals, e-Visa holders included. It is not a visa.
- **Exempt:** Indian citizens. OCI cardholders: sources conflict, so treat them as needing to file unless the traveller has confirmed otherwise.
- **Last verified:** 2026-11-11: labels and validation from the official form's own code. Much shorter than guides describe: no flight number, date of birth, visa number, occupation or port of arrival. The Purpose list is served by the site and is read from the page. The separate Air Suvidha health self-declaration and customs are not part of this card.
- **Always the user's (hand over):**
  - The image CAPTCHA at submit (Take control).
  - The declaration tick and Submit (user replies 'submit'). There is no review page: the review is the filled form itself.

### Fields, page by page

Tier: 🔐 VAULT = traveller's Vault entry · 🧠 RECALL = memory → email → calendar → derived · ✍️ ASK = ask every time, never saved.

#### e-Arrival Card (single page)

'Quick Fill From Existing Data' (passport number, nationality, CAPTCHA) reuses an earlier card for repeat visitors. Otherwise fill the sections below. 'Add Member' adds up to five companions who share the arrival, address and contact details.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Full Name (as in passport) | `full_name` | 🔐 VAULT | yes |  |
| Nationality/Region | `nationality` | 🔐 VAULT | yes |  |
| Passport Number | `passport_no` | 🔐 VAULT | yes | 6-15 letters and digits |
| Purpose | `purpose` | 🧠 RECALL | yes | Read the list from the page; OTHER needs text |
| Date of Arrival(DD/MM/YYYY) | `arrival_date` | 🧠 RECALL | yes | Date picker, limited to the open window |
| Countries Visited in Last 6 Days | `recent_travel_6d` | ✍️ ASK | yes | Multi-select, or tick 'No Country Visited' |
| Address in India | `stay_address` | 🧠 RECALL | yes | Hotel name and street; a city name alone is not accepted |
| State / District | `destination_cities` | 🧠 RECALL | yes | State select, then District (filtered by state) |
| Email ID | `email` | 🔐 VAULT | yes |  |
| Contact Number | `mobile` | 🔐 VAULT | yes | ISD code + 5-15 digits |
| Emergency Contact Number | `emergency_contact` | 🔐 VAULT | no |  |
| Add Member: Full Name, Passport Number, Nationality/Region | `companions` | 🧠 RECALL | no |  |

Sources: https://indianvisaonline.gov.in/earrival/ · https://boi.gov.in/boi/

## 4. How to validate the result

A File Number is shown and `Arrival-Card-<file number>.pdf` downloads. It can be downloaded again from "Download DE
Card" with the file number.

## 5. What to return

File number(s), the saved PDF path, and what to show: the passport, with the PDF as backup. A mistake can't be
edited: file again. Repeat the reminder about the health self-declaration and customs.

## 6. What requires approval

- The declaration tick and the final submit.
