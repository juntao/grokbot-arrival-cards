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

<!-- FIELDS -->

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
