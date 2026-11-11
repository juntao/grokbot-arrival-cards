---
skill: my-arrival-card
description: Walk the Malaysia Digital Arrival Card (MDAC) on the official immigration site, field by field, from values resolved by arrival-card-fill. Stops on the filled form for the user's 'submit' and the slider puzzle.
---

# Skill: my-arrival-card

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for a foreigner arriving in Malaysia
within 3 days.

First check whether the traveller needs it. Singapore citizens, Malaysian permanent residents and a few pass
holders are reported exempt (see the card facts below). If the traveller is one of them, say so and confirm before
filing anyway.

## 2. Required inputs and access

- Every 🔐 VAULT and 🧠 RECALL field below has a value. The card has no health or customs questions.
- A plain email address. The site rejects addresses with a `+` alias.
- The user is available for the slider puzzle at the end.

## 3. Sequence of work

1. Open `https://imigresen-online.imi.gov.my/mdac/main` and click **REGISTER**. The site is slow at times: wait
   and retry once before reporting a problem.
2. Fill the single page **in table order**. The order matters:
   - Nationality reloads the phone-code list, so pick it before the phone code.
   - Date of Arrival resets Date of Departure, so pick arrival first.
   - State loads the City list, and City sets the postcode options.
3. **Type every value.** Paste is blocked on almost every box. Dates are read-only: pick them from the calendar.
4. Name: letters and spaces only. Remove hyphens, apostrophes and accents, and list the change as "chosen for
   you".
5. Address: the hotel name and street. The site strips `.` `#` `&` and `:` as you type, so check what the box
   shows afterwards.
6. If the accommodation block isn't shown for this nationality, skip it.
7. The form has no review page. Stop on the filled form and follow step 7 of `arrival-card-fill`.
8. After "submit": use **Take control** for the slider puzzle, then press **Submit**.
9. Find the email from `mdac@imi.gov.my` in Gmail and read the PIN at the bottom. Open **Check Registration**,
   enter the passport number, nationality and PIN, and download the PDF.

For several travellers, repeat the whole form for each one: the web form takes one traveller per submission.

### Card facts: Malaysia Digital Arrival Card (MDAC)

- **Official URL (only this one):** https://imigresen-online.imi.gov.my/mdac/main
- **Filing window:** Within 3 days before arrival, arrival day included: the arrival datepicker offers only today, +1 and +2 days. A new card is needed for each entry.
- **Who files:** Nearly all foreign nationals entering Malaysia by air, land or sea. One submission per traveller, infants included.
- **Exempt:** Per press and embassy notices, not the form itself: Singapore citizens; Malaysian permanent residents; MACS holders; diplomatic and official passports; airside transit; Brunei GCI holders; Thailand Border Pass and Indonesia PLB holders; crew. Confirm on imi.gov.my when it matters.
- **Last verified:** 2026-11-11: registration form HTML and inline validators read directly; field ids from a live walk 2026-10-31. No health or customs questions on the form. Exemption list is from third-party sources.
- **Always the user's (hand over):**
  - The slider puzzle ('Drag To Verify'): Submit stays disabled until it is solved. Always the user's (Take control).
  - The final Submit (user replies 'submit'). There is no review page: the review is the filled form itself.

### Fields, page by page

Tier: 🔐 VAULT = traveller's Vault entry · 🧠 RECALL = memory → email → calendar → derived · ✍️ ASK = ask every time, never saved.

#### Home

Click REGISTER. One page, two sections. Paste is blocked on almost every text box: type each value. Text is upper-cased by the site. Dates are read-only datepickers (DD/MM/YYYY): pick from the calendar. Fill in the order below, because Nationality reloads the phone codes, State loads the cities, and Date of Arrival resets Date of Departure.

#### 1 · Personal Information

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Name | `full_name` | 🔐 VAULT | yes | Max 60; letters A-Z and spaces only. Drop hyphens, apostrophes and accents (O'NEIL-SMITH → ONEIL SMITH) and list it as 'chosen for you'. |
| Passport No. | `passport_no` | 🔐 VAULT | yes | Max 12, letters and digits |
| Nationality / Citizenship | `nationality` | 🔐 VAULT | yes | Select, 'SGP - SINGAPORE' style |
| Date of Birth | `dob` | 🔐 VAULT | yes | Datepicker DD/MM/YYYY |
| Place of Birth | `birth_country` | 🔐 VAULT | yes | Select, country (same list) |
| Sex | `sex` | 🔐 VAULT | yes | MALE / FEMALE |
| Date of Passport Expiry | `passport_expiry` | 🔐 VAULT | yes | Datepicker DD/MM/YYYY |
| Email Address / Confirm Email Address | `email` | 🔐 VAULT | yes | Typed twice. A '+' alias (name+tag@…) is rejected and the box is cleared: use a plain address. |
| Country / Region Code + Mobile No. | `mobile` | 🔐 VAULT | yes | Select '( 65 ) SINGAPORE' style; number digits only, max 12 |

#### 2 · Traveling Information

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Date of Arrival | `arrival_date` | 🧠 RECALL | yes | Datepicker; today to today+2 |
| Date of Departure | `departure_date` | 🧠 RECALL | yes | Datepicker; on or after arrival |
| Flight / Vessel / Transportation No. | `arrival_flight` | 🧠 RECALL | yes | e.g. MH606; for land, the bus or car plate number |
| Mode of Travel | `arrival_flight` | 🧠 RECALL | yes | AIR / LAND / SEA |
| Last Port of Embarkation before Malaysia | `origin` | 🧠 RECALL | yes | Select, country level (not the airport) |
| Accommodation of Stay | `stay_type` | 🧠 RECALL | yes | HOTEL/MOTEL/REST HOUSE · RESIDENCE OF FRIENDS/RELATIVES · OTHERS. The accommodation block may be hidden for some nationalities: fill it when shown. |
| Address (In Malaysia) | `stay_name` | 🧠 RECALL | yes | Hotel name and street, max 100. The site strips . # & and : as you type. |
| State / City / Postcode | `stay_address` | 🧠 RECALL | yes | State select (16), then City select (loads per state), then 5-digit postcode |

#### 3 · Verify and submit (stop here)

Screenshot the filled form, send the summary, wait for 'submit'. Then hand over for the slider puzzle, and press Submit. An email from mdac@imi.gov.my carries a PIN; 'Check Registration' (passport number, nationality, PIN) downloads the PDF. There is no edit function: to correct a card, submit a new one.

Sources: https://imigresen-online.imi.gov.my/mdac/main · https://www.imi.gov.my/index.php/en/main-services/mdac/

## 4. How to validate the result

The site confirms the registration, and the PIN email arrives. If the email doesn't arrive within a few minutes,
check spam, then say so: without the PIN the PDF can't be downloaded, but the registration is still tied to the
passport.

## 5. What to return

Confirmation, the saved PDF path (or that the PIN email is still missing), and what to show: the passport. An
advance MDAC is needed to use the autogates. A mistake can't be edited: submit a new card.

## 6. What requires approval

- The final **Submit**.
- Filing for a traveller who appears to be exempt.
