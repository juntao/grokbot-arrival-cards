---
skill: cn-arrival-card
description: Walk China's online arrival card for foreigners on the official NIA site, field by field, from values resolved by arrival-card-fill. Uploads the passport photo, then stops for the user's signature and 'submit'.
---

# Skill: cn-arrival-card

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for a foreigner entering mainland China
who isn't in an exempt group.

## 2. Required inputs and access

- Every 🔐 VAULT and 🧠 RECALL field below has a value, and every ✍️ ASK field has an answer from this
  conversation. That includes **countries visited in the past two years**, which the user confirms. Trips found in
  email and calendar may be offered only as a starting list.
- A photo of the passport data page for the step 1 upload. Use the copy at the Vault hint `passport_photo=`,
  kept at onboarding, or one attached in this chat. Delete any resized copy after the upload. Without a photo, ask
  for one, or hand over step 1 to the user.

## 3. Sequence of work

1. Open `https://s.nia.gov.cn/ArrivalCardFillingPC/`. Choose **English** if the page opens in Chinese. Click
   **Entry Declaration**, then **I AGREE**.
2. **Upload ID Document Page.** Upload the JPG/PNG, first resizing it to ≤100 KB if needed. The site reads the
   photo into step 2.
3. **Basic Information.** Check every value the site read against the Vault and overwrite any difference, because
   photo reading makes mistakes with names and dates. Then fill transport mode, flight, City of Entry, Port of Entry
   and Date of Entry (`yyyy-MM-dd` then Enter).
4. **Entry Category.** If the Vault line `visa_CN=` has a visa number, answer Yes with Visa Type and number.
   Otherwise answer No and choose the entry policy:
   - **Visa-Free Policy** for a nationality currently on China's visa-free list, with a stay inside its limit.
     Verify the current list on the NIA or embassy site, because it changes often.
   - **240-Hour Visa-Free Transit** only for a qualifying third-country onward ticket.
   - If unsure, stop and ask.
5. **Personal Information**, then **Travel Information**, in table order.
   - Address in China: use the region picker (province, city, district), then the detail line `<HOTEL NAME>,
     <street address>`.
   - "Confirmed Departure Itinerary": Yes when there's an onward ticket, and fill its five fields.
   - Inviting entity: No for tourism. Otherwise the entity name and contact from the invitation email.
6. **Accompanying Person(s).** Add the companions with their relationship. Each companion still needs their own
   card, so run this skill again for each one.
7. **Declaration.** Stop there and follow step 7 of `arrival-card-fill`. After "submit", use **Take control** for
   the drawn signature, then submit.
8. Download or screenshot the **Entry Declaration Receipt for Foreigner** QR code and save it. Offer to email it
   to the user.

### Card facts: China Online Arrival Card for Foreigners (NIA)

- **Official URL (only this one):** https://s.nia.gov.cn/ArrivalCardFillingPC/
- **Filing window:** Any time before arrival (no official window); also possible at the port by QR, kiosk or paper card.
- **Who files:** All foreigners entering mainland China by air, land or sea, visa-free entries included.
- **Exempt:** Foreign Permanent Resident ID holders; non-Chinese Mainland Travel Permit (HK/Macau) holders; 24 h direct airside transit; visa-free or group-visa tour groups; transport crew; cruise passengers leaving on the same ship; travellers using the E-channel; 'centralized submission' courtesy cases.
- **Last verified:** 2026-11-11: live form bundle (s.nia.gov.cn/ArrivalCardFillingPC/js/app.*.js) + NIA notice; field map from a live walk 2026-10-31. No health questions (customs health declaration abolished 1 Nov 2023).
- **Always the user's (hand over):**
  - Step 1 passport data-page photo (JPG/PNG ≤100 KB): upload the photo the user attached in chat (resize if larger), else Take control.
  - The drawn signature on the declaration step (Take control).
  - Final Submit (user replies 'submit').

### Fields, page by page

Tier: 🔐 VAULT = traveller's Vault entry · 🧠 RECALL = memory → email → calendar → derived · ✍️ ASK = ask every time, never saved.

#### Landing

Click 'Entry Declaration', then 'I AGREE'. The form is Element UI on one URL: fields have no ids, so find each by its label. Selects open a list (li.el-select-dropdown__item); filterable ones take typed text. Cities come from the 'Popular Cities' list or a typed search.

#### 1 · Upload ID Document Page

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Passport data-page photo | `passport_photo` | 🔐 VAULT | yes | JPG/PNG ≤100 KB. The site reads it into step 2: then CHECK every read value against the Vault and correct it. |

#### 2 · Basic Information

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Type of ID Document | `passport_country` | 🔐 VAULT | yes | Ordinary passport |
| Last Name | `surname` | 🔐 VAULT | yes |  |
| First Name | `given_names` | 🔐 VAULT | yes |  |
| Gender | `sex` | 🔐 VAULT | yes | Male / Female |
| Date of Birth | `dob` | 🔐 VAULT | yes | Year / Month / Day selects |
| Country/Region of Citizenship | `nationality` | 🔐 VAULT | yes | Type the country name; pick the row ending '（ISO3）' |
| Passport Number | `passport_no` | 🔐 VAULT | yes |  |
| Entry Transportation Mode | `arrival_flight` | 🧠 RECALL | yes | Flight / Train / Vessel / Automobile / On Foot |
| Arrival Flight/Train/Vessel Number | `arrival_flight` | 🧠 RECALL | yes | e.g. MU5098, no space |
| City of Entry | `arrival_airport` | 🧠 RECALL | yes | City of the arrival airport (PVG/SHA → Shanghai, PEK/PKX → Beijing, CAN → Guangzhou, SZX → Shenzhen, CTU/TFU → Chengdu) |
| Port/Channel of Entry | `arrival_airport` | 🧠 RECALL | yes | Airport option: PVG Pudong, SHA Hongqiao, PEK Capital, PKX Daxing, CAN Baiyun, SZX Bao'an, TFU Tianfu, HGH Xiaoshan |
| Date of Entry | `arrival_date` | 🧠 RECALL | yes | yyyy-MM-dd + Enter |

#### 3 · Selecting Entry Category

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Do you hold a valid visa or other entry permit? | `visa` | 🔐 VAULT | yes | Yes → Visa Type + Visa Number; No → Entry Policy |
| Entry Policy Selection | `entry_policy` | 🧠 RECALL | if No visa | Visa-Free Policy · 24/72/144/240-Hour Visa-Free Transit · 30-Day Hainan · Emergency Temporary Entry Permit. Visa-free list changes often: verify the nationality on the NIA site; if unsure, ASK. |

#### 4 · Personal Information

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Chinese Name | `chinese_name` | 🔐 VAULT | no |  |
| Other Name | `other_names` | 🔐 VAULT | no |  |
| Other Countries/Regions of Citizenship | `other_citizenship` | 🔐 VAULT | no |  |
| Country/Region of Birth | `birth_country` | 🔐 VAULT | yes |  |
| City of Birth | `birth_city` | 🔐 VAULT | yes |  |
| Contact Number (with Phone Area Code) | `mobile` | 🔐 VAULT | yes |  |
| Contact Number in China | `local_phone` | 🧠 RECALL | no |  |
| Email | `email` | 🔐 VAULT | yes |  |
| Has your passport ever been lost or stolen? | `passport_lost` | ✍️ ASK | yes | Yes / No |

#### 5 · Travel Information

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Purpose of Entry | `purpose` | 🧠 RECALL | yes | Conference/Business · Visit · Sightseeing/Leisure · Visiting Friends/Relatives · Work · Study · Permanent Residence · Return to Country of Residence · Others |
| Destination Cities in China | `destination_cities` | 🧠 RECALL | yes |  |
| Cities of Transit in China | `transit_cities` | 🧠 RECALL | no |  |
| Have you been to other countries or regions in the past two years? | `recent_travel_2y` | ✍️ ASK | yes | Yes → list of countries/regions |
| Confirmed Departure Itinerary | `departure_flight` | 🧠 RECALL | yes | Yes → Date, City, Port/Channel, Mode, Flight number of departure; No if no onward ticket |
| Date of Departure | `departure_date` | 🧠 RECALL | if Confirmed Departure Itinerary = Yes |  |
| Address in China | `stay_address` | 🧠 RECALL | yes | Region picker (province / city / district) + detailed address incl. hotel name |
| Do you have any inviting entities or inviters in China? | `inviting_entity` | 🧠 RECALL | yes | No for tourism; Yes → entity (name + contact) or inviter (name + contact) |

#### 6 · Accompanying Person(s)

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Accompanying persons (name, document, relationship) | `companions` | 🧠 RECALL | no | Up to 10; relationship Spouse / Parents / Children / Relatives / Other. Each companion still needs their own card. |

#### 7 · Declaration and signature (stop here)

Screenshot, send the summary, wait for 'submit'. Then hand over for the drawn signature, and submit. Download the 'Entry Declaration Receipt for Foreigner' (QR) and save it.

| Site label | Key | Tier | Req | Format / options |
|---|---|---|---|---|
| Signature | `signature` | ✍️ ASK | yes |  |

Sources: https://us.china-embassy.gov.cn/eng/lsfw/zj/notice/202512/t20251203_11765346.htm · https://s.nia.gov.cn/ArrivalCardFillingPC/

## 4. How to validate the result

The receipt page shows a QR code and the traveller's name. The passport number on the receipt matches the Vault
(compare only the last 3 characters when you report it).

## 5. What to return

Receipt saved (path), travellers covered, and what to present: passport plus the receipt QR at the border.
No health declaration is needed unless the traveller has symptoms. From 3 Sep 2026, people coming from
polio-affected areas who have had contact with polio cases or have symptoms must declare to customs.

## 6. What requires approval

- The final **Submit**.
- The signature, which is always drawn by the user.
- Choosing an entry policy other than plain visa-free, which needs the user's confirmation.
