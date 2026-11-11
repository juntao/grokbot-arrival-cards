---
skill: mv-traveller-declaration
description: Walk the Maldives IMUGA Traveller Declaration on the official immigration site, step by step, from values resolved by arrival-card-fill, reading the email code from Gmail. Stops at the last step for the user's 'submit'.
---

# Skill: mv-traveller-declaration

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for a traveller flying into the Maldives
within 96 hours. Don't open the site earlier: early filings are reported as rejected.

## 2. Required inputs and access

- Every 🔐 VAULT and 🧠 RECALL field below has a value, including a **face photo** of each traveller (the Vault
  hint `portrait_photo=`, or one attached in this chat).
- The user's own answers, from this conversation, to the health and customs questions.
- Gmail, to read the 6-digit code, when the email is the connected mailbox.

**The labels in the table below are descriptions, not the site's exact wording.** The steps behind the email code
haven't been seen. Match each field to what the page shows. If the page asks for something that isn't in the
table, stop and ask the user: don't guess.

## 3. Sequence of work

1. Open `https://imuga.immigration.gov.mv/traveller/declaration`. Check the address ends in `.gov.mv`: paid
   look-alike sites exist, and this form is free.
2. **Verify email.** Enter the email and send the code. Read the 6-digit code from Gmail, or ask the user for it.
3. **Purpose.** Pick the card that matches. A tourist is **Holiday**.
4. **Personal.** Enter the passport details by hand if the page allows it. If it insists on a passport scan,
   upload the kept passport photo, then check every value it read against the Vault and correct any difference.
   Upload the face photo and adjust the crop so the whole face shows.
5. **Travel.** Pick the airline and then the flight from the lists. Pick the place of stay from the site's list
   and give the island. If the resort or guesthouse isn't listed, ask.
6. **Health.** Answer with the user's answers. If an Ebola block or a vaccination question appears that the user
   wasn't asked, stop and ask, showing the page's wording.
7. **Customs.** Read the list of declarable items from the page and check it against what the user was asked.
   Enter the number of checked bags.
8. For family or a group, add them at the end as the page offers.
9. Stop before the final submit and follow step 7 of `arrival-card-fill`. Submit after "submit".
10. Find the confirmation email and save the QR code. Keep the **edit token** from it in the trip log: it's
    needed to change the declaration.

### Card facts: Maldives IMUGA Traveller Declaration

- **Official URL (only this one):** https://imuga.immigration.gov.mv/traveller
- **Filing window:** Within 96 hours before the arriving flight. Earlier filings are reported as rejected. The departure declaration is reported abolished since Aug 2024, though the site still mentions it.
- **Who files:** All travellers arriving in the Maldives, one declaration each, infants included.
- **Exempt:** Maldivian citizens, per press reports from Nov 2023.
- **Last verified:** 2026-11-11: step order and field names from the official declaration page's own form state. The on-screen labels of steps 2-6 sit behind the email code and were not seen: the labels below are descriptions, and the bot matches them to what the page shows.
- **Always the user's (hand over):**
  - The 6-digit email code at step 1: read from Gmail when the email is the connected mailbox, else the user types it.
  - The final submit (user replies 'submit').

### Fields, page by page

Tier: 🔐 VAULT = traveller's Vault entry · 🧠 RECALL = memory → email → calendar → derived · ✍️ ASK = ask every time, never saved.

#### 1 · Verify email

Open /traveller/declaration. Enter the email, 'Send verification code', then the 6-digit code. The confirmation is sent to this address.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Email Address | `email` | 🔐 VAULT | yes |  |

#### 2 · Purpose

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Purpose of visit | `purpose` | 🧠 RECALL | yes | Cards: Holiday · Employment · Dependent · Business · Official or Diplomatic · Conference · Sponsorship · Crew |
| Permit number and expiry | `visa` | 🔐 VAULT | if A purpose that needs a permit (not Holiday) |  |

#### 3 · Personal

Skip the passport scan if manual entry is offered. If the scan is forced, upload the kept passport photo and check every value it read against the Vault.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Passport number | `passport_no` | 🔐 VAULT | yes |  |
| Given name | `given_names` | 🔐 VAULT | yes |  |
| Surname | `surname` | 🔐 VAULT | yes |  |
| Date of birth | `dob` | 🔐 VAULT | yes |  |
| Gender | `sex` | 🔐 VAULT | yes |  |
| Passport expiry date | `passport_expiry` | 🔐 VAULT | yes |  |
| Nationality | `nationality` | 🔐 VAULT | yes |  |
| Country of residence | `home_country` | 🔐 VAULT | yes |  |
| Mobile number | `mobile` | 🔐 VAULT | yes |  |
| Photo of the traveller | `portrait_photo` | 🔐 VAULT | yes | Image upload with a crop tool; a clear face photo on a light background (JPEG/PNG, reported limit 6 MB) |

#### 4 · Travel

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Country of departure | `origin` | 🧠 RECALL | yes |  |
| Port of entry | `arrival_airport` | 🧠 RECALL | yes | Select; Velana International Airport (MLE) for Malé |
| Arrival date | `arrival_date` | 🧠 RECALL | yes |  |
| Arrival airline and flight | `arrival_flight` | 🧠 RECALL | yes | Chosen from lists, not typed: airline first, then the flight |
| Booking reference (PNR) | `booking_ref` | 🧠 RECALL | no |  |
| Return ticket, departure date, airline and port | `departure_flight` | 🧠 RECALL | no | Yes with the onward flight details when there is a ticket |
| Departure date | `departure_date` | 🧠 RECALL | no |  |
| Next destination | `next_destination` | 🧠 RECALL | no |  |
| Place of stay (resort, guesthouse, hotel or liveaboard) | `stay_name` | 🧠 RECALL | yes | Picked from the site's list of facilities |
| Address and island in the Maldives | `stay_address` | 🧠 RECALL | yes |  |
| Length of stay | `length_of_stay` | 🧠 RECALL | yes | Days |

#### 5 · Health

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Yellow fever: travelled from or through a risk country, and which | `recent_travel_yf` | ✍️ ASK | yes | Yes / No, then the countries |
| Yellow fever vaccination, date and certificate | `yellow_fever_cert` | ✍️ ASK | if Yellow fever = Yes |  |
| Ebola: countries visited, contact, funeral attendance, symptoms | `ebola_exposure` | ✍️ ASK | if Shown for certain origins |  |

#### 6 · Customs (stop here)

'Travelling with family or a group? You can add them at the end.' Screenshot, send the summary, wait for 'submit', then submit. The confirmation with a QR code and an edit token goes to the email.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Number of checked bags | `baggage_count` | ✍️ ASK | yes |  |
| Anything to declare to customs | `customs_declarable` | ✍️ ASK | yes | Yes / No; read the listed items from the page and show them to the user |
| Declaration details | `customs_goods_detail` | ✍️ ASK | if Anything to declare = Yes |  |

Sources: https://imuga.immigration.gov.mv/traveller · https://www.immigration.gov.mv/traveller-declaration

## 4. How to validate the result

The confirmation email arrives with a QR code. The site's **Verify** page (surname and passport number) finds
the declaration.

## 5. What to return

Confirmation, the saved QR path, and what to show: passport plus the QR code. Say which on-screen fields differed
from the table, so the catalog can be corrected. A departure declaration is reported as no longer required; say
you didn't file one.

## 6. What requires approval

- The final submit.
- Any health or customs answer the user didn't give in this chat.
- Keeping the face photo in `/workspace`.
