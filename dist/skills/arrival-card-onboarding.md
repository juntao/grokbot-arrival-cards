---
skill: arrival-card-onboarding
description: First-run setup for the Arrival Card Bot. When no "Travel identity" Vault entry exists, or the user asks to add a traveller, collect each traveller's passport photo and identity fields once and save them to the Vault, so no future arrival card has to ask for them.
---

# Skill: arrival-card-onboarding

## 1. When to use it

- **Automatically, at the start of a conversation**, before answering anything else, when the Vault has **no**
  Contact info entry whose nickname starts with `Travel identity ·`. This is the first time the user opens the bot,
  including right after the template is installed.
- When the user asks to add, update or check a traveller: "add my wife", "I renewed my passport", "what do you have
  on file?".
- From `arrival-card-fill`, when a traveller on a trip has no Vault entry yet.

If the user declines ("later", "not now"), say they can type "set up my travel profile" any time, then answer
their actual request. Don't offer again in the same conversation. When a card is filled later, its gap form still
collects the missing Vault fields.

## 2. Required inputs and access

- Vault: read Contact info entries, and create or update them.
- Optional: Gmail and Google Calendar connected. Check, and offer to connect them at the end.
- From the user: a **photo of the passport data page** (the page with the photo and the two machine-readable lines
  at the bottom), plus answers to the questions the photo can't answer.

## 3. Sequence of work

1. **Check the Vault.** List Contact info entries nicknamed `Travel identity · …`. For each one, note which of the
   fields below are missing or expire soon (passport expiry within 6 months).
2. **Open with the offer.** For a first run, send exactly one message:

   > 👋 I fill out arrival cards for **14 countries**: China, Singapore, Japan, Taiwan, Korea, Thailand,
   > Malaysia, Indonesia, Cambodia, the Philippines, India, the Maldives, New Zealand and Vietnam. I find your flights and hotels in
   > your email and calendar, and ask you only for what I can't find.
   >
   > Let's set up your **travel identity** once (about 2 minutes), so no card ever has to ask for it again. It's
   > stored in your **Vault**, not in chat memory.
   >
   > **Easiest:** attach a photo of your passport's data page. I'll read your name, passport number, dates and
   > nationality from it, and ask only for the few things a passport doesn't show (phone, email, home city,
   > occupation).
   >
   > Or reply **"type it"** to enter the details by hand, or **"later"** to skip for now.
   > Travelling with family? I can set them up too, one passport at a time.

3. **If a passport photo is attached:**
   - Read the visible fields and the two MRZ lines at the bottom. Cross-check name, number, nationality, date of
     birth, sex and expiry between the visual zone and the MRZ. If they disagree, or the photo is blurry or cut
     off, ask for a clearer photo of just that part.
   - Fill these fields from the photo: `surname`, `given_names`, `full_name` (given names, then surname),
     `passport_no`, `passport_country`, `nationality`, `sex`, `dob`, `passport_expiry`, plus `birth_country` and
     `birth_city` when the passport prints a place of birth.
   - Ask once: *"Keep a copy of this photo in your bot's private files for China's arrival card (it requires an
     upload)? Yes / No."* Yes saves it to `/workspace/arrival-cards/identity/<GIVEN>_<SURNAME>_passport.jpg` and
     records the path as `passport_photo=`. No means it's discarded after reading, and China will ask for it again.
4. **Build one form** of the remaining Vault fields from the table below. Skip any field the photo, an existing
   Vault entry, or memory already gave. Put the found values in an "I read this from your passport" block, passport
   shown as `•••••789`, so the user can correct them in the same reply. Number the questions, and give options and
   formats, e.g. *"Occupation (as you'd like it on cards, e.g. SOFTWARE ENGINEER)"*.
   - Ask `visa` only per country: *"Do you hold a visa or residence permit for any country you travel to often (for example China, Japan, Taiwan, Korea, Indonesia, Vietnam, New Zealand)? Reply
     'none' for any where you travel visa-free."*
   - Don't ask `vjw_login` or `etravel_login` here. Each is created the first time that country's card is filed.
   - Offer the optional extras once, and skip them if the user doesn't want to: a face photo (`portrait_photo`,
     needed for the Maldives), passport issue date (Philippines), and an emergency contact (India).
5. **Save to the Vault**, using the format in `arrival-card-fill` §2:
   - Nickname `Travel identity · GIVEN SURNAME`.
   - Name, address, phone and email in the native slots.
   - Every other field as a `key=value` line in Hints, starting with `use=arrival cards; travel identity of GIVEN
     SURNAME`.
   - Update an existing entry; never delete lines.
6. **Confirm** in one short message, showing the passport as `•••••789`:
   *"Saved to your Vault: Mei Ling TAN: passport •••••67X (SGP, expires 31 May 2031), DOB, birthplace, phone,
   email, home city, occupation. ✅ Passport photo kept for China."*
   Then warn if the passport expires within 6 months, or within 6 months of a trip found in the calendar.
7. **Next traveller.** Ask "Anyone who usually travels with you? Send their passport photo, or say 'done'."
   Remember "usually travels with: …" as a memory preference (names only).
8. **Finish.** If Gmail or Google Calendar isn't connected, offer to connect it. Then, if the calendar shows a trip
   to any of the supported countries in the next 60 days, offer: *"I see you're flying to Tokyo on 3 Dec. Want me to prepare
   that arrival card now?"*

### Vault fields to collect (generated from the catalog)

| Key | Ask as | Vault slot | Needed by |
|---|---|---|---|
| `full_name` | Full name as in passport | Vault Contact › Name | ID, IN, MY, SG, TW |
| `surname` | Surname / family name | Vault hint `surname=` | CN, JP, KH, KR, MV, NZ, PH, TH, VN |
| `given_names` | Given name(s) | Vault hint `given_names=` | CN, JP, KH, KR, MV, NZ, PH, TH, VN |
| `other_names` | Other names used (former / alias), or 'none' | Vault hint `other_names=` | CN |
| `other_citizenship` | Other countries / regions of citizenship, or 'none' | Vault hint `other_citizenship=` | CN |
| `middle_name` | Middle name, or 'none' | Vault hint `middle_name=`. Thailand has a separate box; when the passport prints one given-names line, split it only if the user says so. | PH, TH |
| `chinese_name` | Name in Chinese characters (optional) | Vault hint `chinese_name=` | CN, TW |
| `sex` | Sex as in passport (F / M / X) | Vault hint `sex=` | CN, ID, JP, KH, KR, MV, MY, PH, SG, TH, TW, VN |
| `dob` | Date of birth | Vault hint `dob=` | CN, ID, JP, KH, KR, MV, MY, NZ, PH, SG, TH, TW, VN |
| `nationality` | Nationality (passport country) | Vault hint `nationality=` | CN, ID, IN, JP, KR, MV, MY, NZ, PH, SG, TH, TW, VN |
| `birth_country` | Country of birth | Vault hint `birth_country=` | CN, ID, KH, MY, NZ, PH, SG, TW |
| `birth_city` | City of birth | Vault hint `birth_city=` | CN, TW |
| `passport_no` | Passport number | Vault hint `passport_no=` | CN, ID, IN, JP, KH, KR, MV, MY, NZ, PH, SG, TH, TW, VN |
| `passport_country` | Passport issuing country | Vault hint `passport_country=` | CN, KH, PH, VN |
| `passport_issue` | Passport date of issue | Vault hint `passport_issue=` | PH |
| `passport_expiry` | Passport date of expiry | Vault hint `passport_expiry=` | ID, JP, KH, KR, MV, MY, SG, TW, VN |
| `passport_photo` | Photo of the passport data page | Vault hint `passport_photo=`. Collected at onboarding. Read once to fill the other identity fields; with the user's OK a copy is kept at /workspace/arrival-cards/identity/<GIVEN>_<SURNAME>_passport.jpg for China's upload step, and its path is the hint value. | CN |
| `portrait_photo` | Passport-style photo of the traveller's face | Vault hint `portrait_photo=`. Maldives needs an upload. With the user's OK a copy is kept at /workspace/arrival-cards/identity/<GIVEN>_<SURNAME>_portrait.jpg and its path is the hint value; a clear crop from the passport photo page is not acceptable. | MV, PH |
| `home_address` | Home address (street, city, state or province) | Vault Contact › Address | PH |
| `emergency_contact` | Emergency contact phone number, with country code (optional) | Vault hint `emergency_contact=` | IN |
| `home_city` | City of residence | Vault hint `home_city=` | JP, SG, TH |
| `home_state` | State, province or prefecture of residence | Vault hint `home_state=` | NZ |
| `home_country` | Country of residence | Vault hint `home_country=` | JP, MV, NZ, PH, TH, TW |
| `mobile` | Mobile number with country code | Vault Contact › Phone | CN, ID, IN, KH, MV, MY, NZ, PH, SG, TH, TW, VN |
| `email` | Email address for the card | Vault Contact › Email | CN, ID, IN, KH, KR, MV, MY, NZ, SG, TH, TW, VN |
| `occupation` | Occupation / job title | Vault hint `occupation=` | JP, KH, KR, NZ, PH, TH, TW |
| `visa` | Visa or residence permit for this country (number, or 'none') | Vault hint `visa=`. Stored per destination as visa_CN / visa_JP / visa_TW / visa_KR / visa_TH / visa_ID / visa_NZ / visa_VN; 'none' when visa-exempt. | CN, ID, KR, MV, TH, TW, VN |
| `etravel_login` | Philippines eTravel account (email + password) | Vault Login. A Vault Login for etravel.gov.ph, created on the first Philippines card and filled into the page without the Bot seeing the password. | PH |
| `vjw_login` | Visit Japan Web account (email + password) | Vault Login. A Vault Login for vjw.digital.go.jp, filled into the page without the Bot seeing the password. | JP |

## 4. How to validate the result

- The Vault has one `Travel identity · …` entry per traveller, holding every field in the table above that the user
  answered.
- The MRZ check digits are consistent with the passport number, date of birth and expiry. If not, re-read or ask.
- No passport number appears in chat (other than the masked form), in memory, or in any file except the kept photo.

## 5. What to return

Each traveller saved (masked passport, expiry), anything still missing (it will be asked on the first card's form),
the apps still to connect, and the offer for the next upcoming trip.

## 6. What requires approval

- Keeping the passport photo in `/workspace`, which is asked explicitly.
- Overwriting a Vault value the user didn't just give, e.g. a different passport number already saved. Show the old
  and new values masked and ask.
