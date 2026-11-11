---
skill: vn-prearrival
description: Fill Vietnam's Pre-arrival Information (digital arrival card) on the official immigration portal, from values resolved by arrival-card-fill, reading the email code from Gmail. Stops at Review & Submit for the user's 'submit'.
---

# Skill: vn-prearrival

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for a foreigner flying into Vietnam
within 3 days. Vietnamese passport holders and transit passengers don't file it.

It is not a visa. The form asks for the traveller's existing visa or exemption and checks it. If the traveller
needs an e-visa and has none, say so now: this skill doesn't apply for one.

## 2. Required inputs and access

- Every 🔐 VAULT and 🧠 RECALL field below has a value. The card has no health or customs questions.
- The traveller's **entry basis** for Vietnam: visa-exempt by nationality, an e-visa (with its 9-digit number and
  dates), or another visa or card. Take it from the Vault's `visa_VN=` line, or ask.
- The user is available for the image CAPTCHA at the start.
- Gmail, to read the 6-digit code at submit, when the email is the connected mailbox.
- **Everything ready before you start**: the page reloads after 60 seconds idle and sessions expire.

## 3. Sequence of work

1. Open `https://prearrival.immigration.gov.vn/`. Check the address: fake sites that ask for card details exist,
   and this form is free. Set the language to English.
2. Choose **Create & Submit Pre-arrival Information**. Use **Take control** for the CAPTCHA.
3. **Skip** the passport scan and select the nationality.
4. **Passenger Information**, in table order.
   - Arrival date: one of the three buttons. If the date isn't offered, the window isn't open: stop and say when
     to try again. The buttons follow the computer's date, so near midnight check them against Vietnam time.
   - Visa: tick the "read and understood" box, pick the visa type that matches the entry basis, then the number
     and dates if it asks. If the page says the visa or passport number is invalid, stop and ask.
   - For companions, use **Add Passenger +**. Each needs their own passport details, email and relationship.
     Everyone shares the same flight and accommodation.
5. **Trip Information.**
   - Flight: pick it from the search list. Use "Other" only if it's truly missing.
   - Check the Border Gate that fills in is the airport on the ticket.
   - Accommodation: province, then ward, then the hotel from the list. If the hotel isn't listed, use "Other"
     and type its address.
6. **Review & Submit.** Stop there and follow step 7 of `arrival-card-fill`.
7. After "submit": tick the confirmation, press **Submit**, read the 6-digit code from Gmail (or ask the user;
   a new code can be requested after 60 seconds), and enter it.
8. Save the QR code and the PDF. Note the **Pre-Arrival Information Number** in the trip log.

### Card facts: Vietnam Pre-arrival Information (digital arrival card)

- **Official URL (only this one):** https://prearrival.immigration.gov.vn/
- **Filing window:** Within 3 days before arrival: the form offers only today, tomorrow and the day after as the arrival date (Vietnam time). Can also be filed on arrival.
- **Who files:** Foreign nationals, and overseas Vietnamese entering on a visa, arriving at Tan Son Nhat, Noi Bai, Da Nang, Cam Ranh and Phu Quoc airports. Each traveller is a separate entry, children included.
- **Exempt:** Vietnamese passport holders and transit passengers. Official sources call it a pilot and differ on whether it is mandatory: treat it as required in practice. It is not a visa; the e-visa (evisa.gov.vn) is separate.
- **Last verified:** 2026-11-11: labels and validation from the portal's own code and English text (build 1.1.0). Dropdown lists (purpose, visa types, airports, hotels, provinces) are served after the CAPTCHA and are read from the page. Not walked live. No health or customs questions.
- **Always the user's (hand over):**
  - The image CAPTCHA that gates the form (Take control).
  - The 6-digit email code at Submit: read from Gmail when the email is the connected mailbox, else the user types it.
  - The confirmation tick and Submit (user replies 'submit').

### Fields, page by page

Tier: 🔐 VAULT = traveller's Vault entry · 🧠 RECALL = memory → email → calendar → derived · ✍️ ASK = ask every time, never saved.

#### Home and pre-steps

Choose 'Create & Submit Pre-arrival Information', pass the CAPTCHA, skip 'Scan Your Passport', and select the nationality. The page reloads after 60 seconds idle and sessions expire: have everything ready and keep moving. Latin letters only; dates are DD/MM/YYYY.

#### 1 · Passenger Information

Passenger 1 is the lead. 'Add Passenger +' adds companions, who share the trip details and each need an email and a relationship to the lead.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Expected Arrival Date (DD/MM/YYYY GMT+7) | `arrival_date` | 🧠 RECALL | yes | Three buttons: today, +1, +2. Cannot be changed later. |
| Passport Type | `passport_country` | 🔐 VAULT | yes | Dropdown; 'P' (ordinary passport) is the default |
| Passport Number | `passport_no` | 🔐 VAULT | yes | Letters and digits; checked by the server |
| Date of Expiry | `passport_expiry` | 🔐 VAULT | yes |  |
| Gender | `sex` | 🔐 VAULT | yes | Male / Female / Other |
| Surname | `surname` | 🔐 VAULT | no |  |
| Given Name | `given_names` | 🔐 VAULT | yes | No accents |
| Date of Birth | `dob` | 🔐 VAULT | yes |  |
| Nationality | `nationality` | 🔐 VAULT | yes | Prefilled from the pre-step |
| Country Code + Phone Number | `mobile` | 🔐 VAULT | yes |  |
| Email Address | `email` | 🔐 VAULT | yes |  |
| Relationship with Lead Passenger | `companions` | 🧠 RECALL | if Passengers 2 and later | Spouse · Child · Parent · Sibling · Friend · Colleague · Other … |
| Visa Type / Purpose | `entry_policy` | 🧠 RECALL | yes | Read the list from the page: e-visa, stamped visa, visa exemption (by nationality), Phu Quoc exemption, 5-year exemption certificate, APEC card, temporary residence card … Tick 'I have read and understood this information.' first. If unsure which applies, ASK. |
| Visa Number, Date of Issue, Date of Expiry, Issued Place | `visa` | 🔐 VAULT | if Any type except a visa exemption | An e-visa number is exactly 9 digits (the 'Số / No.' line on the e-visa). Checked by the server. |

#### 2 · Trip Information

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Departure country before Arrival in Vietnam | `origin` | 🧠 RECALL | yes | Country; with connections, the first country of departure |
| Purpose of Travel | `purpose` | 🧠 RECALL | yes | Read the list from the page (Tourism, Business, Education, Employment, Transit …) |
| Mode of Travel + Flight Number | `arrival_flight` | 🧠 RECALL | yes | Air; pick the flight from the search list. Use 'Other' and type it (e.g. VN1234) only if the flight is truly missing: mismatched flight numbers are the main cause of failed submissions. |
| Border Gate | `arrival_airport` | 🧠 RECALL | yes | Fills itself from the flight; check it is the right airport |
| Type of Accommodation in Vietnam | `stay_type` | 🧠 RECALL | yes | Hotel / Residential / Others |
| Province / City of Hotel, Ward / Commune of Hotel | `stay_address` | 🧠 RECALL | yes | Province dropdown, then ward |
| Accommodation Address | `stay_name` | 🧠 RECALL | yes | Hotel: pick from the site's hotel list, or 'Other' and type the address. Residential: typed. |
| Expected date of departure from Vietnam (DD/MM/YYYY GMT+7) | `departure_date` | 🧠 RECALL | no | On or before the visa's expiry |

#### 3 · Review & Submit (stop here)

Screenshot, send the summary, wait for 'submit'. Then tick 'I confirm that the information is correct.', press Submit, and enter the 6-digit email code. Save the QR code and 'Download PDF Pre-Arrival Information'. If the page times out during processing, the declaration is still received and the result is emailed.

Sources: https://prearrival.immigration.gov.vn/ · https://en.mps.gov.vn/article/immigration-department-launches-pre-arrival-information-system-1778670404

## 4. How to validate the result

The page shows "Your submission is successful!" with a QR code and a Pre-Arrival Information Number, and an email
copy arrives. A "duplicate" error means a declaration already exists for this traveller and date: look it up
instead of filing again.

## 5. What to return

The number, the saved QR and PDF paths, and what to show: passport plus the QR code. To change it, use **Lookup &
Update Declaration** with the number and the lead passport number. The arrival date, passport number, date of
birth and nationality can't be changed: cancel and file again.

## 6. What requires approval

- The confirmation tick and **Submit**.
- Choosing a visa type when the entry basis isn't clear.
