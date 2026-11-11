---
skill: kh-arrival-card
description: Walk the Cambodia e-Arrival Card on the official site, field by field, from values resolved by arrival-card-fill. Stops at Confirm Declaration for the user's 'submit'.
---

# Skill: kh-arrival-card

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for a foreigner flying into Cambodia
within 7 days. The form lists airports only: for a land or sea arrival, say it isn't covered and stop.

This card is not a visa. If the traveller needs a visa, the e-visa is a separate application this skill
doesn't do.

## 2. Required inputs and access

- Every 🔐 VAULT and 🧠 RECALL field below has a value, including the **hotel's phone number**, which the form
  requires. Take it from the booking email or the hotel's website; never invent it.
- The user's own answers, from this conversation, to the three health questions and the two customs questions.

## 3. Sequence of work

1. Open `https://arrival.gov.kh/`. Click **Submit e-Arrival**, then **Sign In as Guest**. Wait 6 to 8 seconds
   for the form.
2. **Profile**, in table order.
   - Pickers: click the box, type in the dialog's Search box, click the row.
   - Dates: three dropdowns each.
   - Occupation: the closest of the 17 options; note it as "chosen for you".
3. Press **Next**. A dialog reads back the passport number and country. Check both against the Vault, then
   **Correct**.
4. **Travel Information.** Wait for the page to load. The occupation, email and phone boxes are empty again:
   fill them a second time. Then the trip and the stay, in table order. Pick "Next country" from its list, not with
   the "Same as last" box.
5. **Health Information.** Answer the three questions with the user's answers. Read each question's full wording
   from the page first; if it differs from what the user was asked, ask again.
6. **Customs Declaration.** Tick Yes or No for both questions, then the "true and correct" box, then **Next**.
7. **Confirm Declaration.** Stop there and follow step 7 of `arrival-card-fill`.
8. After "submit": tick "I have read and agreed to the declaration." and press **Submit**. Save what the
   confirmation page shows.

If a step shows "This field is required." under a box, fill that box and press Next again.

### Card facts: Cambodia e-Arrival Card

- **Official URL (only this one):** https://arrival.gov.kh/
- **Filing window:** Within 7 days before arrival: the form offers the 7 days from today as arrival dates.
- **Who files:** Foreign travellers arriving by air at Phnom Penh (Techo), Siem Reap Angkor or Sihanoukville airports.
- **Exempt:** Not established from official sources: land and sea arrivals are outside the form (it lists airports only). It is not a visa; the e-visa is separate.
- **Last verified:** 2026-11-02: form walked end to end on the live site with test data, up to but not including Submit. No CAPTCHA was shown. Who must file and exemptions were not re-checked against an official notice.
- **Always the user's (hand over):**
  - The box 'I have read and agreed to the declaration.' and Submit on step 5 (user replies 'submit').

### Fields, page by page

Tier: 🔐 VAULT = traveller's Vault entry · 🧠 RECALL = memory → email → calendar → derived · ✍️ ASK = ask every time, never saved.

#### Home

Click 'Submit e-Arrival', then 'Sign In as Guest'. The form takes 6-8 seconds to render, and each later step shows a loading skeleton for about 6 seconds. Pickers are read-only boxes that open a dialog: type in its Search box, then click the row. Dates are three dropdowns (Day, Month, Year). Each visit leaves a saved draft.

#### 1 · Profile

Next opens 'Is the information correct?' reading back the passport number and country: check them, then Correct.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Passport Number | `passport_no` | 🔐 VAULT | yes |  |
| Passport Issued Country | `passport_country` | 🔐 VAULT | yes | Picker: type the ISO3 code, pick the 'Singaporean . SGP' style row |
| Date Of Passport Expiry | `passport_expiry` | 🔐 VAULT | yes | Day / Month / Year dropdowns |
| Surname | `surname` | 🔐 VAULT | no |  |
| Given Name | `given_names` | 🔐 VAULT | yes |  |
| Gender | `sex` | 🔐 VAULT | yes | Female / Male / Other |
| Date Of Birth | `dob` | 🔐 VAULT | yes | Day / Month / Year dropdowns |
| Country of birth | `birth_country` | 🔐 VAULT | yes | Picker, country name |
| Occupation | `occupation` | 🔐 VAULT | yes | Fixed list of 17: Student, Teacher, Farmer, Journalist, Government official, Company employee, Military personnel, Retired, Religious personnel, Member of parliament, Crew member, NGO staff, Artist, Self-employed, Worker, Police officer, Other. Map the Vault value to the closest. |
| Email | `email` | 🔐 VAULT | no |  |
| Country/Region Code + Phone Number | `mobile` | 🔐 VAULT | yes | Picker: type the country name (a code search is ambiguous), pick 'Singapore (+ 65)'; then digits |

#### 2 · Travel Information

The contact block (occupation, email, phone) shows again here, empty: fill it a second time.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Date of Arrival in Cambodia | `arrival_date` | 🧠 RECALL | yes | Radio per day, labels like 'November 04, 2026' |
| Date of Departure from Cambodia | `departure_date` | 🧠 RECALL | yes | Day / Month / Year dropdowns |
| Port of Entry | `arrival_airport` | 🧠 RECALL | yes | Techo International Airport (KTI, Phnom Penh's flights since Sept 2025) · Siem Reap Angkor International Airport (SAI) · Sihanoukville International Airport (KOS) |
| Travel purpose | `purpose` | 🧠 RECALL | yes | Holiday/Pleasure/Vocation · Transit · Business/Meeting/Conference/Convention/Exhibition · Education/Training · Employment · Medical Care · Official/Government visit · Religion · Sports event · To take up residence · Visiting Friends/Relatives · Others |
| Type of Air Transport + Flight Number | `arrival_flight` | 🧠 RECALL | yes | Commercial / Private; flight written with a space, e.g. 'SQ 166' |
| Accommodation Type | `stay_type` | 🧠 RECALL | yes | Hotel / Private Residential / Day Trip / Transit / Other |
| Hotel name | `stay_name` | 🧠 RECALL | if Hotel |  |
| Hotel contact number | `stay_phone` | 🧠 RECALL | yes | Required, never invented: from the booking email or the hotel's website. A private home takes the host's phone. |
| Last country/Port of Embarkation | `origin` | 🧠 RECALL | yes | Picker, country name |
| Next country/Port of Disembarkation | `next_destination` | 🧠 RECALL | yes | Picker, country name. Pick it from the list; the 'Same as last' box is unreliable. |

#### 3 · Health Information

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Have you been vaccinated against yellow fever? | `yellow_fever_cert` | ✍️ ASK | yes | Yes / No |
| Do you have fever, cough or rash now? (read the full symptom list from the page) | `health_symptoms` | ✍️ ASK | yes | Yes / No |
| Have you visited the Middle East, Africa or Latin America in the past 14 days? | `recent_travel_14d` | ✍️ ASK | yes | Yes / No |

#### 4 · Customs Declaration

Next needs the box 'I hereby confirm that all the statements in this declaration are true and correct.' Tick it only after the user has given both answers in this chat.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Are you carrying foreign currency over US$10,000? | `customs_cash` | ✍️ ASK | yes | Yes / No checkboxes |
| Do you have goods to declare? | `customs_declarable` | ✍️ ASK | yes | Yes / No checkboxes |

#### 5 · Confirm Declaration (stop here)

Screenshot, send the summary, wait for 'submit'. Then tick 'I have read and agreed to the declaration.' and press Submit. Save the confirmation and any QR code shown.

Sources: https://arrival.gov.kh/

## 4. How to validate the result

The site shows a confirmation after Submit. The confirmation page hasn't been mapped, so describe what it shows
and save a screenshot along with any QR code or reference.

## 5. What to return

What the confirmation shows, the saved path, and what to present: passport, plus the QR code if one was issued.
Mention that each visit to the site leaves a saved draft.

## 6. What requires approval

- The final declaration tick and **Submit**.
- Any health or customs answer the user didn't give in this chat.
