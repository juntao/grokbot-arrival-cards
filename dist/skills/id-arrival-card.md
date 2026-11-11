---
skill: id-arrival-card
description: Walk the All Indonesia arrival card (immigration, health, quarantine and customs) on the official site, field by field, from values resolved by arrival-card-fill. Stops at the Declaration page for the user's 'submit' and the CAPTCHA.
---

# Skill: id-arrival-card

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for anyone arriving in Indonesia within
3 days. This skill covers foreign visitors. Indonesian citizens use a different form on the same site: say so and
ask before going further.

This card is **not a visa**. It doesn't replace the Visa on Arrival / e-VOA, and it doesn't cover the Bali
tourist levy (lovebali.baliprov.go.id). Mention both when they apply; this skill doesn't pay for either.

## 2. Required inputs and access

- Every 🔐 VAULT and 🧠 RECALL field below has a value.
- The user's own answers, from this conversation, to every ✍️ ASK question: symptoms, countries in the last 21
  days, animals and plants, number of bags, goods to declare, phones bought abroad, and the different-nationality
  passport question.
- The same passport as on any visa or e-VOA: the site checks the visa number against immigration records.

## 3. Sequence of work

1. Open `https://allindonesia.imigrasi.go.id/`. Open the **Languages** menu (it reacts after about 4 seconds) and
   pick **English**. Click the **Foreign Visitor** tile and continue as a guest.
2. **Personal Information**, in table order. Skip "Scan MRZ".
   - Dropdowns are read-only boxes: click the box, type in the panel's **Search** box, wait for the row, click it,
     then check the box shows the value.
   - For more travellers, use **Add Traveller** (up to 10) and repeat.
3. **Travel Details.** Pick the arrival date from the four listed. Answer the visa question: Yes with the visa or
   stay-permit number from the Vault's `visa_ID=` line, and confirm the "Data Verification" popup only if its name
   and passport number match. Otherwise No.
4. **Mode of Transportation & Address.**
   - AIR, then wait about 3 seconds for the air fields.
   - Port of Entry: type the airport code and pick the row.
   - Type of Air Transport, then wait for the airline list. Pick the airline by name; the code fills itself. Type
     the flight number digits.
   - Purpose of Travel from the list.
   - Accommodation: for HOTEL, type the hotel name and pick it from the site's own list, then wait until the
     immigration office fills itself. If the hotel isn't listed, use the list's "other" option, or ask.
5. **Declaration.** Answer each question with the user's own answer. If any answer is Yes, fill the follow-up
   details the user gave; if they didn't give them, stop and ask.
6. Stop there and follow step 7 of `arrival-card-fill`.
7. After "submit": tick the certification box, press **Submit**, confirm, and use **Take control** for the image
   CAPTCHA.
8. On the **Submission Summary**, save the Arrival QR Code and card number (**Download Arrival Card**).

If **Next** shows "Incomplete Data!" or "Invalid Input", press OK, find the marked field, and fill it. A field
whose list was still loading is the usual cause: wait and pick it again.

### Card facts: All Indonesia arrival card (immigration, health, quarantine and customs in one)

- **Official URL (only this one):** https://allindonesia.imigrasi.go.id/
- **Filing window:** From 3 days before arrival: the arrival-date list offers only yesterday, today, +1 and +2 days. Can be changed before the trip ('Change Data').
- **Who files:** Every international arrival, foreigners and Indonesian citizens alike, at all international airports, seaports and land borders. One card per person, infants included.
- **Exempt:** No exemption found in official sources.
- **Last verified:** 2026-11-11: labels and validation from the site's own English locale and form code; field flow from a live walk 2026-11-02. Dropdown lists are served by the site and are read from the page. It is not a visa: Visa on Arrival and the Bali tourist levy are separate.
- **Always the user's (hand over):**
  - The image CAPTCHA in the submit dialog (Take control).
  - The certification tick box and Submit on the Declaration page (user replies 'submit').

### Fields, page by page

Tier: 🔐 VAULT = traveller's Vault entry · 🧠 RECALL = memory → email → calendar → derived · ✍️ ASK = ask every time, never saved.

#### Home

Open the Languages menu (it reacts after about 4 seconds) and pick English, then click the 'Foreign Visitor' tile and continue as a guest. Dropdowns are read-only boxes: click, type in the panel's Search box, wait for the row, click it. Lists that depend on an earlier field load a moment later. Next with something missing shows 'Incomplete Data!': press OK and fill the marked field.

#### 1 · Personal Information

Skip 'Scan MRZ' and enter by hand. 'Add Traveller' adds up to 10 people with one group leader.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Passport/Country/Region | `nationality` | 🔐 VAULT | yes | Dropdown, country name in capitals. Use the same passport as for any visa or e-VOA. |
| Full Name | `full_name` | 🔐 VAULT | yes | Letters only, upper case |
| Date of Birth | `dob` | 🔐 VAULT | yes | DD/MM/YYYY, typed |
| Country/Place of Birth | `birth_country` | 🔐 VAULT | yes | Dropdown, same country list |
| Gender | `sex` | 🔐 VAULT | yes | MALE / FEMALE rows |
| Passport Number | `passport_no` | 🔐 VAULT | yes |  |
| Date of Passport Expiry | `passport_expiry` | 🔐 VAULT | yes | DD/MM/YYYY |
| Have you ever used a passport under different nationality/region to enter Indonesia? | `other_passport_nationality` | ✍️ ASK | yes | Yes / No; Yes asks for that passport's country and number |
| Mobile Number | `mobile` | 🔐 VAULT | yes | Code dropdown '+65 - SINGAPORE' style, then digits |
| Email | `email` | 🔐 VAULT | yes |  |

#### 2 · Travel Details

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Arrival Date to Indonesia | `arrival_date` | 🧠 RECALL | yes | List of four dates, e.g. '04 NOVEMBER 2026' |
| Departure Date from Indonesia | `departure_date` | 🧠 RECALL | yes | DD/MM/YYYY |
| Do you already have a Visa or Stay Permit? | `entry_policy` | 🧠 RECALL | yes | Yes / No. No shows a note to apply for Visa on Arrival after the card. |
| Visa or Stay Permit Number | `visa` | 🔐 VAULT | if Visa or Stay Permit = Yes | A 'Data Verification' popup shows the visa record: check the name and passport, then 'Yes, it is' |

#### 3 · Mode of Transportation & Address

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Mode of Transport | `arrival_flight` | 🧠 RECALL | yes | AIR / LAND / SEA. After AIR, wait for the air fields to load. |
| Port of Entry | `arrival_airport` | 🧠 RECALL | yes | Type the airport code, pick e.g. 'DPS - I GUSTI NGURAH RAI AIRPORT', 'CGK - SOEKARNO-HATTA AIRPORT' |
| Type of Air Transport | `arrival_flight` | 🧠 RECALL | yes | COMMERCIAL FLIGHT; the airline list loads after it |
| Flight Name + Flight Number | `arrival_flight` | 🧠 RECALL | yes | Airline by name (e.g. 'SINGAPORE AIRLINES'); the code fills itself; then the digits |
| Purpose of Travel | `purpose` | 🧠 RECALL | yes | 1-DAY TRANSIT · BUSINESS/MEETING/CONFERENCE/CONVENTION/EXHIBITION · EDUCATION/TRAINING · EMPLOYMENT · HOLIDAY/SIGHTSEEING/LEISURE · MEDICAL CARE · OFFICIAL/GOVERNMENT VISIT · RELIGION · SPORT EVENT · VISITING FRIENDS/RELATIVES · OTHERS. Should match the visa type. |
| Type of Accommodation | `stay_type` | 🧠 RECALL | yes | RESIDENTIAL / HOTEL / OTHERS |
| Name of Hotel | `stay_name` | 🧠 RECALL | if HOTEL | Searched on the site's own hotel list: type the name, pick the row ('ALILA SEMINYAK, KAB. BADUNG'). The immigration office then fills itself: wait for it. Not on the list: use the list's 'other' option and type the name, or ask. |
| Address in Indonesia / Province / City | `stay_address` | 🧠 RECALL | if RESIDENTIAL |  |

#### 4 · Declaration (stop here)

Health, quarantine and customs in one page. Screenshot, send the summary, wait for 'submit'. Then tick the certification, press Submit, confirm, and hand over for the image CAPTCHA. Save the Arrival QR Code and card number from the Submission Summary ('Download Arrival Card').

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Do you currently have any of the following symptoms: fever, cough, runny nose, shortness of breath, sore throat, skin lesions/rashes, diarrhea, vomiting, bleeding, or yellowing of the skin/eyes? | `health_symptoms` | ✍️ ASK | yes | Yes / No; Yes asks which |
| Country of origin, transit, and other countries visited within 21 days prior to your arrival in Indonesia | `recent_travel_21d` | ✍️ ASK | yes | Multi-select, at least one |
| Seat Number | `seat_number` | 🧠 RECALL | no |  |
| Are you bringing any animals, fish, plants, and/or their processed products? | `customs_restricted` | ✍️ ASK | yes | Yes / No; Yes opens commodity details. The site notes a temporary ban on bringing these in. |
| Number of Baggage Arriving with You | `baggage_count` | ✍️ ASK | yes | Number, up to 2 digits |
| Do you carry goods that must be declared to Customs? | `customs_declarable` | ✍️ ASK | yes | Yes / No. Show the user the categories: restricted items; cash of IDR 100 million or more; alcohol over 1 litre or tobacco over 200 cigarettes; purchases over USD 500; trade goods. |
| Goods to declare: Description, Quantity, Currency, Value | `customs_goods_detail` | ✍️ ASK | if Goods to declare = Yes |  |
| Are you bringing Mobile Phones, Handheld Computers, or Tablet Computers purchased abroad that will be used in Indonesia on local cellular networks? (IMEI Registration) | `imei_devices` | ✍️ ASK | yes | Yes / No |

Sources: https://allindonesia.imigrasi.go.id/ · https://batam.imigrasi.go.id/berita/2025/09/arrival-passengers-at-three-international-airports-and-ports-in-batam-are-required-to-declare-their-arrival-through-the-all-indonesia-application

## 4. How to validate the result

The Submission Summary shows the QR code, a card number and the status "Submitted". If it shows a health alert,
pass it on to the user word for word.

## 5. What to return

Card number, the saved QR path, and what to show: passport plus the QR code. Add the reminders that apply: the
Visa on Arrival link on the summary page, and the Bali levy. To change something before the trip, use **Retrieve
Arrival Card** and "Change Data".

## 6. What requires approval

- The certification tick and the final **Submit**.
- Any Yes or No on the Declaration page that the user didn't give in this chat.
- Confirming a visa "Data Verification" popup whose details don't match.
