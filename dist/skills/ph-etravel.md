---
skill: ph-etravel
description: Register a Philippines eTravel arrival declaration (immigration, health and customs) on the official site, signing in with the Vault login, from values resolved by arrival-card-fill. Stops at the Summary for the user's 'submit'.
---

# Skill: ph-etravel

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for anyone arriving in the Philippines
within 72 hours. The airline checks the QR code before boarding, so do it before the user leaves for the airport.

## 2. Required inputs and access

- A Vault **Login** for `etravel.gov.ph`. If there isn't one, ask approval to create the account with the card
  email, and let Vault generate the password.
- Gmail, for the 6-digit code when creating the account.
- Every 🔐 VAULT and 🧠 RECALL field below has a value, and the user's own answers, from this conversation, to the
  three health questions and the customs question.

The page order below is inferred, and some wording is only shown after sign-in. Match each field to what the page
shows. If the page asks for something that isn't in the table, stop and ask the user.

## 3. Sequence of work

1. Open `https://etravel.gov.ph/`. Use **Take control** for the Cloudflare check on the sign-in page.
2. Sign in with the Vault login. For a new account: enter the email, read the 6-digit code from Gmail (if you
   need another, wait 3 minutes), then create the password.
3. **Profile**, the first time only: choose FOREIGN PASSPORT and fill the table's Profile fields. For companions,
   use **Add Family Member** and fill a profile for each.
4. **New Travel Declaration**: FOR ME, AIR, ARRIVAL. Tick the family members travelling.
5. **Travel Details**, in table order. Airline, flight, airports and hotel are look-ups: type, then pick the row.
   If the hotel isn't found, ask.
6. **Health Declaration**: the user's answers. If any is Yes, tell the user the QR code will be red, which means
   a stop at the quarantine desk on arrival. Don't change an answer to avoid that.
7. **Summary.** Stop there and follow step 7 of `arrival-card-fill`. Submit after "submit".
8. **Customs.** Answer "Do you have baggage or currency to declare?" with the user's answer. For Yes, fill the
   follow-up pages from what the user gave you, and use Take control for the drawn signature.
9. Download the QR code for each traveller and save it.

### Card facts: Philippines eTravel (arrival declaration, health and customs)

- **Official URL (only this one):** https://etravel.gov.ph/
- **Filing window:** Within 72 hours before arrival. The airline checks the QR code before boarding.
- **Who files:** All arriving passengers and crew, Filipino and foreign. Departing Filipino passengers also file; departing foreigners are not listed.
- **Exempt:** Foreign diplomats and their dependents, foreign dignitaries and delegations, 9(e) visa holders, and diplomatic or official passport holders.
- **Last verified:** 2026-11-11: labels, required rules and option lists from the site's own language bundle, validation code and public look-up data. The page order is inferred, the symptom list and the exact customs question wording are served after sign-in, and the form has not been walked live.
- **Always the user's (hand over):**
  - The Cloudflare check on the sign-in page (Take control).
  - Account creation the first time: needs the user's approval; the 6-digit email code is read from Gmail when possible.
  - The drawn signature, only when there is baggage or currency to declare.
  - The final Submit (user replies 'submit').

### Fields, page by page

Tier: 🔐 VAULT = traveller's Vault entry · 🧠 RECALL = memory → email → calendar → derived · ✍️ ASK = ask every time, never saved.

#### Sign in

An account is required. Returning: sign in with the Vault Login. First time: enter the email, read the 6-digit code (resend is throttled: wait 3 minutes), and create a password that Vault generates and saves.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Email address / Password | `etravel_login` | 🔐 VAULT | yes |  |

#### Profile (once per account)

Choose FOREIGN PASSPORT. Saved to the account, so later trips skip this. 'Add Family Member' creates a profile for each companion.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Photo | `portrait_photo` | 🔐 VAULT | no | 'Take a photo or upload a file.' May be enforced only for Philippine passports: upload if the page insists. |
| First Name | `given_names` | 🔐 VAULT | yes | Upper case |
| Middle Name (optional) | `middle_name` | 🔐 VAULT | no |  |
| Last Name (optional) | `surname` | 🔐 VAULT | yes |  |
| Sex | `sex` | 🔐 VAULT | yes | FEMALE / MALE |
| Birth Date (MM/DD/YYYY) | `dob` | 🔐 VAULT | yes |  |
| Mobile Number | `mobile` | 🔐 VAULT | yes | Country code + number |
| Citizenship | `nationality` | 🔐 VAULT | yes | Country look-up |
| Country of Birth | `birth_country` | 🔐 VAULT | yes |  |
| Passport Number | `passport_no` | 🔐 VAULT | yes |  |
| Passport Issuing Authority | `passport_country` | 🔐 VAULT | yes | Country look-up |
| Passport Issued Date | `passport_issue` | 🔐 VAULT | yes |  |
| Occupation | `occupation` | 🔐 VAULT | yes | Professional/Technical/Administrative · Clerical/Sales · Agriculture · Worker/Laborer · Military/Government Personnel · Housewife · Student/Minor · Retired/Pensioner · Seaman · Airline Crew · Diplomat · Entertainer · Unemployed · Businessman |
| Permanent Country of Residence | `home_country` | 🔐 VAULT | yes |  |
| No./Bldg./City/State/Province | `home_address` | 🔐 VAULT | yes | The home address abroad, one line |

#### New Travel Declaration

FOR ME, AIR, ARRIVAL ('Entering the Philippines'). Tick saved family members to generate their declarations too; each still gets their own QR.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Family Member(s) | `companions` | 🧠 RECALL | no |  |

#### Travel Details

Traveller Type: AIRCRAFT PASSENGER.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Purpose of Travel | `purpose` | 🧠 RECALL | yes | Holiday/Pleasure/Vacation · Convention/Conference · Education/Training/Studies · Government/Official Mission · Health/Medical Reason · Business/Professional · Visit Friends/Relatives · Work/Employment · Religion/Pilgrimage · Incentive · Returning Resident |
| Date of Departure | `origin_departure_date` | 🧠 RECALL | yes |  |
| Date of Arrival | `arrival_date` | 🧠 RECALL | yes |  |
| Date of Return | `departure_date` | 🧠 RECALL | if Required for holiday and visiting friends/relatives |  |
| Country of Origin / Airport of Origin | `origin` | 🧠 RECALL | yes |  |
| With Transit (Connecting Flight)? | `transit_cities` | 🧠 RECALL | no | Yes adds country, airport and date of transit |
| Name of Airline / Flight Number | `arrival_flight` | 🧠 RECALL | yes | Look-ups; ignore the auto-filled Manila terminal |
| Airport of Destination | `arrival_airport` | 🧠 RECALL | yes |  |
| Destination upon arrival in the Philippines | `stay_type` | 🧠 RECALL | yes | Residence / Hotel/Resort / Transit Via Airport |
| Hotel, Resorts, AirBnb, Tourist destinations, etc. | `stay_name` | 🧠 RECALL | if Hotel/Resort | Search look-up |
| Residence Address | `stay_address` | 🧠 RECALL | if Residence | 10-250 characters with street, barangay and city; no phone numbers |

#### Health Declaration

A Yes on sickness or exposure gives a red QR code: the traveller sees the Bureau of Quarantine first on arrival. Say so; never change the answer to avoid it.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Do you have any recent travel history in the last 30 days? Country(ies) worked, visited and transited in the last 30 days | `recent_travel_30d` | ✍️ ASK | yes | Yes / No, then the countries |
| Have you had any history of exposure to a person who is sick or known to have communicable/infectious disease in the past 30 days prior to travel? | `sick_contact` | ✍️ ASK | yes | Yes / No |
| Have you been sick in the past 30 days? | `health_symptoms` | ✍️ ASK | yes | Yes / No; Yes asks for symptoms from the page's list |

#### Summary and customs (stop here)

Screenshot the summary, send it, wait for 'submit'. After Submit the customs part asks its question. Save the QR code ('Download QRCode'): green goes straight to immigration, red goes to quarantine first.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Do you have baggage or currency to declare? | `customs_declarable` | ✍️ ASK | yes | Yes / No. Show the user the 12 categories: pesos over PHP 50,000; foreign currency over USD 10,000; gambling items; cosmetics, supplements or medicines beyond personal use; dangerous drugs; firearms; alcohol or tobacco in commercial quantities; foodstuffs, animals or plants; gadgets beyond personal use; cremains or human tissue; jewelry, gold or gems; other goods. |
| Customs details: family members, baggage counts, goods and currency rows | `customs_goods_detail` | ✍️ ASK | if Baggage or currency to declare = Yes | Many follow-up fields and a drawn signature: collect them from the user, and hand over for the signature |

Sources: https://etravel.gov.ph/ · https://etravel.gov.ph/frequently-asked-questions

## 4. How to validate the result

The dashboard shows the declaration with a QR code. Say whether it's green or red.

## 5. What to return

QR colour, the saved QR path for each traveller, and what to show: the QR code to the airline at check-in and
again on arrival, with the passport. The declaration can be edited or deleted from the dashboard until the border
officer has processed it. Say which on-screen fields differed from the table.

## 6. What requires approval

- Creating the eTravel account.
- The final **Submit**.
- Any health or customs answer the user didn't give in this chat.
