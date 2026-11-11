---
skill: nz-traveller-declaration
description: Walk the New Zealand Traveller Declaration (NZTD) web form, section by section, from values and declaration answers resolved by arrival-card-fill. Stops before Submit for the user's 'submit'.
---

# Skill: nz-traveller-declaration

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for anyone travelling into New Zealand,
**no earlier than 24 hours before their trip to New Zealand starts**. The form refuses trip details before that.
With a stopover where the traveller collects their bags, count from the flight that leaves the stopover.

It is separate from the NZeTA and from visas. A traveller who needs an NZeTA and has none should be told now:
this skill doesn't apply for it.

## 2. Required inputs and access

- Every 🔐 VAULT and 🧠 RECALL field below has a value.
- **Every ✍️ ASK question answered by the user in this conversation**: about 20 Yes/No questions on
  immigration history, biosecurity and restricted items, plus the list of countries in the last 30 days. Ask them
  on the gap form with the exact wording in the table. Remind the user that packaged snacks, tea and coffee count
  as food, and that used hiking boots and sports gear count as outdoor gear.
- **Everything ready before you start**: the session ends after 10 minutes idle and 30 minutes in total.

This is a legal declaration with an instant fine of at least NZ$400 for a false answer. Never answer a question
for the user, and never assume No.

## 3. Sequence of work

1. Open `https://app.travellerdeclaration.govt.nz/` and choose **New declaration**. Tick the privacy box.
2. **Contact details**, then **Passport details**, in table order. Use the passport the traveller will enter on.
3. **Trip details.** Search the flight number. If it isn't found, use "Manually add flight details". If the form
   says it's too early, stop and tell the user when it opens.
4. A reference number is emailed now. Note it in the trip log.
5. Complete each section from the hub, in table order: **Purpose of travel**, **Immigration status**, **Travel
   history**, **Biosecurity**, **Restricted items**.
   - Immigration status: with an NZeTA and no visa, the option is "travelling visa waiver". With a visa, the
     matching visa option. If unsure, ask.
   - For each Yes, the page opens follow-up choices. Use what the user told you. If they didn't cover a
     follow-up, stop and ask with the page's options.
6. On **Submit**, answer "Do you know the contents of your baggage?" with the user's answer. Stop there and
   follow step 7 of `arrival-card-fill`: the summary must list every Yes/No answer.
7. After "submit": tick the declaration and press **Submit**. If a "Confirm you are not a robot" check appears,
   use Take control.

For another traveller, use "Start a declaration for someone else" and repeat: each person needs their own, and the
form asks whether you have their consent or guardianship.

### Card facts: New Zealand Traveller Declaration (NZTD)

- **Official URL (only this one):** https://www.travellerdeclaration.govt.nz/
- **Filing window:** From 24 hours before you start your trip to New Zealand, until passport control. The form refuses trip details earlier than that. With a stopover where you collect bags, count from the flight leaving the stopover.
- **Who files:** Everyone travelling into New Zealand, New Zealand passport holders included. One declaration per traveller, babies included.
- **Exempt:** Airside transit passengers. It is separate from the NZeTA and visas: holders of those still file it.
- **Last verified:** 2026-11-11: every question's wording and required status from the official web app's own question and content data (app version 9.5). Not walked live.
- **Always the user's (hand over):**
  - The 'Confirm you are not a robot' check, if the invisible one falls back to it.
  - The declaration tick and Submit (user replies 'submit'). This is a legal document: a false declaration carries an instant fine of at least NZ$400.

### Fields, page by page

Tier: 🔐 VAULT = traveller's Vault entry · 🧠 RECALL = memory → email → calendar → derived · ✍️ ASK = ask every time, never saved.

#### Start, privacy and contact details

Open app.travellerdeclaration.govt.nz and choose 'New declaration'. Tick the privacy box. Answers must be in English. The session ends after 10 minutes idle and 30 minutes in total: have every answer ready. A 7-character reference number is emailed on starting; 'Edit declaration' uses it.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Your email address | `email` | 🔐 VAULT | yes |  |
| Your contact phone number | `mobile` | 🔐 VAULT | yes | Country code + phone number |

#### Passport details

Use the passport you will enter New Zealand on. Passport expiry and gender are not asked.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Passport number | `passport_no` | 🔐 VAULT | yes | Letters and digits, no spaces |
| Nationality | `nationality` | 🔐 VAULT | yes | Type-ahead |
| Surname or family name | `surname` | 🔐 VAULT | yes |  |
| Given or first name(s) | `given_names` | 🔐 VAULT | no |  |
| Date of birth | `dob` | 🔐 VAULT | yes | Day / month / year |
| Country or territory of birth | `birth_country` | 🔐 VAULT | yes | Type-ahead |

#### Trip details

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Arriving in New Zealand by | `arrival_airport` | 🧠 RECALL | yes | Air / Sea |
| Date of arrival into New Zealand | `arrival_date` | 🧠 RECALL | yes |  |
| Flight number | `arrival_flight` | 🧠 RECALL | yes | Search, e.g. NZ123: the flight number, not the booking reference. 'Manually add flight details' if not found. |
| Departing from | `origin` | 🧠 RECALL | yes | Port type-ahead by code, name or city: where the flight to New Zealand leaves from |

#### Purpose of travel

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Do you live in New Zealand? / Do you intend on staying in New Zealand permanently? | `resident_status` | 🧠 RECALL | yes | Yes / No each; No to both for a visitor |
| How long do you intend to stay in New Zealand? | `length_of_stay` | 🧠 RECALL | yes | Years / months / days |
| What is your main reason for coming to New Zealand? | `purpose` | 🧠 RECALL | yes | Visiting friends/relatives · Business · Holiday/vacation · Conference/convention · Education · Other |
| In which country or territory did you last live for 12 months or more? | `home_country` | 🔐 VAULT | yes |  |
| State, province or prefecture | `home_state` | 🔐 VAULT | yes |  |
| Full contact or residential address in New Zealand | `stay_address` | 🧠 RECALL | yes | Address lookup, or manual: unit, number, street, suburb, town/city, postcode. Use the first address of the stay. |
| Your occupation or job | `occupation` | 🔐 VAULT | yes | Free text, 3-50 characters |

#### Immigration status

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Which of the following applies to you? | `entry_policy` | 🧠 RECALL | yes | NZ citizen on a foreign passport · NZ residence class visa · Australian passport or permanent resident visa · NZ temporary entry class visa (work, student or visitor visa) · Travelling visa waiver (choose this with an NZeTA and no visa) |
| Departure flight number or vessel name | `departure_flight` | 🧠 RECALL | no |  |
| Estimated departure date from New Zealand | `departure_date` | 🧠 RECALL | no |  |
| Are you coming to New Zealand for medical treatment or consultation, or to give birth? | `medical_purpose` | ✍️ ASK | yes | Yes / No |
| Have you ever been deported, removed, excluded or refused entry from any country? | `deported_any_country` | ✍️ ASK | yes | Yes / No; Yes asks which |
| Have you ever been sentenced to 12 months or more in prison? | `prison_12_months` | ✍️ ASK | yes | Yes / No; based on the original sentence, not time served |

#### Travel history

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| List the countries or territories you have been to outside of New Zealand in the last 30 days (including in transit). | `recent_travel_30d` | ✍️ ASK | yes | Multi type-ahead; the departure country is already shown |

#### Biosecurity

Each Yes opens follow-up choices (food types, how equipment was used, whether it is clean and dry). Ask the user those follow-ups with the page's own options.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Any food: cooked, uncooked, fresh, preserved, packaged or dried? | `bio_food` | ✍️ ASK | yes | Yes / No. Packaged snacks, chocolate, tea and coffee count as food. |
| Animals or animal products: including meat, dairy products, fish, honey, bee products, eggs, feathers, shells, raw wool, skins, bones or insects? | `bio_animal_products` | ✍️ ASK | yes | Yes / No |
| Plants or plant products: fruit, flowers, seeds, bulbs, wood, bark, leaves, nuts, vegetables, parts of plants, fungi, cane, bamboo or straw, including for religious offerings or medicinal use? | `bio_plant_products` | ✍️ ASK | yes | Yes / No |
| Biological products: Animal medicines, biological cultures, organisms, soil or water? | `bio_biological` | ✍️ ASK | yes | Yes / No |
| Equipment: Equipment/clothing used with animals, plants or water, including for gardening, beekeeping, fishing, water sport or diving activities? | `bio_equipment` | ✍️ ASK | yes | Yes / No |
| Outdoor gear: Items that have been used for outdoor activities, including any footwear, tents, camping, hunting, hiking, golf or sports equipment? | `bio_outdoor_gear` | ✍️ ASK | yes | Yes / No |
| In the past 30 days (while outside of New Zealand) have you visited any wilderness areas, had contact with animals (except domestic cats and dogs) or visited properties that farm or process animals or plants? | `bio_farm_contact` | ✍️ ASK | yes | Yes / No |

#### Restricted items

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Medicine: over 3 months' supply, or medicine not prescribed to you? | `customs_medicine` | ✍️ ASK | yes | Yes / No |
| Restricted or prohibited goods: For example, weapons, indecent publications, endangered plants or wildlife, illegal or controlled drugs? | `prohibited_items` | ✍️ ASK | yes | Yes / No |
| Alcohol: more than 3 bottles of spirits (not exceeding 1.125 litres each) and 4.5 litres of wine or beer? | `customs_alcohol` | ✍️ ASK | yes | Yes / No |
| Tobacco: more than 50 cigarettes or 50 grams of tobacco products? | `customs_tobacco` | ✍️ ASK | yes | Yes / No |
| Goods obtained overseas and/or purchased duty-free in New Zealand: with a total value of more than NZ$700 (including gifts)? | `customs_over_allowance` | ✍️ ASK | yes | Yes / No |
| Goods carried for business or commercial use? | `customs_commercial` | ✍️ ASK | yes | Yes / No |
| Goods carried on behalf of another person? | `customs_for_others` | ✍️ ASK | yes | Yes / No |
| Cash: NZ$10,000 or more (or foreign equivalent), including travellers cheques, bank drafts, money orders, etc.? | `customs_cash` | ✍️ ASK | yes | Yes / No |

#### Submit (stop here)

Screenshot, send the summary, wait for 'submit'. Then tick 'I declare that the information I am submitting is true, correct and complete.' and Submit. The page shows 'Submission successful' and the reference number. Nothing to print, no QR code: the declaration is linked to the passport. Any later change means submitting again.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Do you know the contents of your baggage? | `baggage_known` | ✍️ ASK | yes | Yes / No |

Sources: https://www.travellerdeclaration.govt.nz/completing-your-declaration/ · https://app.travellerdeclaration.govt.nz/

## 4. How to validate the result

The page shows "Submission successful" with the reference number, and a second email arrives.

## 5. What to return

Reference number and what to show: the passport only. There's nothing to print and no QR code. Tell the user
that anything declared Yes must be shown to an officer on arrival, and that any change before arrival, such as a
new flight or something bought on the way, means editing and submitting again ("Edit declaration" with the email
and reference number).

## 6. What requires approval

- The declaration tick and **Submit**.
- Every Yes/No answer: all must come from the user in this chat.
- Declaring consent to file for another traveller.
