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

<!-- FIELDS -->

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
