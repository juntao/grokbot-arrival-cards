---
skill: th-arrival-card
description: Walk the Thailand Digital Arrival Card (TDAC) on the official site, field by field, from values resolved by arrival-card-fill, after the user passes the Cloudflare check. Stops at Preview for the user's 'submit'.
---

# Skill: th-arrival-card

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for a non-Thai traveller arriving in
Thailand within 3 days.

## 2. Required inputs and access

- Every 🔐 VAULT and 🧠 RECALL field below has a value, and the user has listed the **countries they stayed in
  during the two weeks before arrival** in this conversation.
- **Everything must be ready before you start.** The site can't save progress and sessions time out, so don't open
  it with questions still open.
- The user is available for the Cloudflare check on the home page.

## 3. Sequence of work

1. Open `https://tdac.immigration.go.th/`. The home page shows a Cloudflare "verify you are human" check, and the
   **Arrival Card** button stays disabled until it passes. Use **Take control** and ask the user to complete the
   check, then say "continue". Never try to get around it.
2. Click **Arrival Card**. Skip "Retrieve with Passport" and enter by hand.
3. **Personal Information**, in table order. Autocomplete boxes need 3 typed characters, then a click on the row.
   The date of birth is three selects: year first.
4. **Trip & Accommodation Information.**
   - Country where you boarded: the country of the flight that lands in Thailand, not where the trip began.
   - Purpose, mode of transport and accommodation type: read the options from the page and pick the closest. If
     none fits, choose "OTHERS (PLEASE SPECIFY)" and type it.
   - Departure: fill all of the departure boxes or none. With no onward ticket, leave them all empty.
   - Accommodation: province first. District and sub-district are needed unless the type is HOTEL. The address is
     the hotel name and street, in English characters.
5. **Health Declaration.** Select the countries the user listed. If the yellow-fever question or the symptom list
   appears and the user hasn't answered it, stop and ask, showing the page's exact options.
6. For more travellers, choose **Add Other Travelers** (up to 10) and use "Same as Previous Traveler" for the trip
   and stay. Otherwise choose **Preview**.
7. On **Preview**, type the email. Stop there and follow step 7 of `arrival-card-fill`.
8. After "submit": tick the terms, press **Submit**, and download the PDF with the QR code.

If the session times out or the site returns an error on submit (it documents HTTP 429 and "0 Unknown Error"),
refresh and start again from step 1, which needs the user for the check again. Tell them why.

### Card facts: Thailand Digital Arrival Card (TDAC)

- **Official URL (only this one):** https://tdac.immigration.go.th/
- **Filing window:** Within 3 days before arrival, arrival day included: the form accepts arrival dates from today to today+3. Cannot be saved part-way: complete it in one go. A later submission replaces an earlier one.
- **Who files:** All non-Thai nationals entering by air, land or sea, including infants and every visa type.
- **Exempt:** Transit passengers who do not pass immigration; crew. Border Pass holders per Thai embassy notices.
- **Last verified:** 2026-11-11: labels and validators from the site's own en.json bundle and form code, User Guide v1.21 (app 2026.10.00). Dropdown option lists are served by the site and were not all verified; the form behind the Cloudflare check has not been walked live.
- **Always the user's (hand over):**
  - The Cloudflare 'verify you are human' check on the home page: the 'Arrival Card' button stays disabled until it passes. Always the user's (Take control); never try to get around it.
  - The final Submit (user replies 'submit').

### Fields, page by page

Tier: 🔐 VAULT = traveller's Vault entry · 🧠 RECALL = memory → email → calendar → derived · ✍️ ASK = ask every time, never saved.

#### Home

Hand over for the Cloudflare check, then click 'Arrival Card'. Entry is English only. Dates are yyyy/mm/dd. Dropdowns autocomplete after 3 typed characters: type, then pick the row. Skip 'Retrieve with Passport' and enter manually.

#### 1 · Personal Information

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Family Name | `surname` | 🔐 VAULT | yes | A-Z and spaces, max 60; '-' if none |
| First Name | `given_names` | 🔐 VAULT | yes | A-Z and spaces, max 60 |
| Middle Name | `middle_name` | 🔐 VAULT | no |  |
| Passport No. | `passport_no` | 🔐 VAULT | yes |  |
| Nationality/Citizenship | `nationality` | 🔐 VAULT | yes | Autocomplete 'CODE: NAME', e.g. 'SGP: SINGAPOREAN' |
| Date of Birth | `dob` | 🔐 VAULT | yes | yyyy / mm / dd selects; month and day unlock after the year |
| Occupation | `occupation` | 🔐 VAULT | yes | Free text, max 100, no digits |
| Gender | `sex` | 🔐 VAULT | yes | Radio: MALE / FEMALE / UNDEFINED |
| Visa No. | `visa` | 🔐 VAULT | no |  |
| Country/Territory of Residence | `home_country` | 🔐 VAULT | yes | Autocomplete 'CODE: NAME' |
| City/State of Residence | `home_city` | 🔐 VAULT | yes | Enabled after the country; text with suggestions |
| Phone No. | `mobile` | 🔐 VAULT | yes | Phone code (digits) + number (digits) |

#### 2 · Trip & Accommodation Information

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Date of Arrival | `arrival_date` | 🧠 RECALL | yes | yyyy/mm/dd; today to today+3 |
| Country/Territory where you Boarded | `origin` | 🧠 RECALL | yes | Autocomplete: the country of the flight that lands in Thailand |
| Purpose of Travel | `purpose` | 🧠 RECALL | yes | Dropdown read from the page (HOLIDAY, BUSINESS, MEETING, EDUCATION, … OTHERS (PLEASE SPECIFY) + text) |
| Mode of Travel / Mode of Transport | `arrival_flight` | 🧠 RECALL | yes | AIR → COMMERCIAL FLIGHT; LAND / SEA options read from the page |
| Flight No./Vehicle No. | `arrival_flight` | 🧠 RECALL | yes | The flight that lands in Thailand, e.g. TG414 |
| Date of Departure | `departure_date` | 🧠 RECALL | no |  |
| Departure Mode of Travel / Mode of Transport / Flight No. | `departure_flight` | 🧠 RECALL | no | All-or-nothing: once one departure box is filled the rest become required; 'Clear' resets them |
| I am a transit passenger, I don't stay in Thailand. | `stay_type` | 🧠 RECALL | if Departure is the day after arrival and the traveller does not stay | Checkbox; skips accommodation |
| Type of Accommodation in Thailand | `stay_type` | 🧠 RECALL | yes | HOTEL · YOUTH HOSTEL · GUEST HOUSE · FRIEND'S HOUSE · APARTMENT · OTHERS (PLEASE SPECIFY) |
| Province | `stay_address` | 🧠 RECALL | yes | Autocomplete, e.g. BANGKOK, PHUKET |
| District, Area / Sub-District, Sub-Area / Post Code | `stay_address` | 🧠 RECALL | if Required unless the type is HOTEL | Autocompletes; the post code fills itself |
| Address | `stay_name` | 🧠 RECALL | yes | Hotel name and street address, English characters only, max 215 |

#### 3 · Health Declaration

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Please list the name of the countries/territories where you stayed within two weeks before arrival | `recent_travel_14d` | ✍️ ASK | yes | Multi-select autocomplete, up to 30 |
| Do you have Yellow Fever Vaccination Certificate? | `yellow_fever_cert` | ✍️ ASK | if Shown only for yellow-fever-list countries | Radio + Date of Vaccination; certificate upload optional |
| Please select the following symptoms listed you have had in past two weeks. | `health_symptoms` | ✍️ ASK | if Shown only with the yellow-fever question | Checkboxes read from the page, including a no-symptoms option: ask the user with the page's exact list |

#### 4 · Preview (stop here)

'Add Other Travelers' (up to 10; 'Same as Previous Traveler' copies trip and stay) before Preview. Screenshot, send the summary, wait for 'submit'. Then tick the terms and Submit. Download the PDF with the QR code; the same arrives by email.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Please specify an email to send Thailand Digital Arrival Card | `email` | 🔐 VAULT | yes | One email, max 50 |

Sources: https://tdac.immigration.go.th/manual/en/index.html · https://tdac.immigration.go.th/manual/en/faq.html

## 4. How to validate the result

The page shows a "TH Digital Arrival Card No." and the PDF downloads. The same PDF arrives by email, and the
email says whether the traveller can use the automated gates.

## 5. What to return

Card number(s), the saved PDF path, the gate status from the email, and what to show: passport plus the QR code.
A wrong name, nationality or date of birth can't be edited: submit a new card, and the latest one counts. Other
fields can be changed with **Update Arrival Card** on the home page.

## 6. What requires approval

- The final **Submit**.
- Any answer on the health declaration that the user didn't give in this chat.
