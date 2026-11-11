---
skill: my-arrival-card
description: Walk the Malaysia Digital Arrival Card (MDAC) on the official immigration site, field by field, from values resolved by arrival-card-fill. Stops on the filled form for the user's 'submit' and the slider puzzle.
---

# Skill: my-arrival-card

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for a foreigner arriving in Malaysia
within 3 days.

First check whether the traveller needs it. Singapore citizens, Malaysian permanent residents and a few pass
holders are reported exempt (see the card facts below). If the traveller is one of them, say so and confirm before
filing anyway.

## 2. Required inputs and access

- Every 🔐 VAULT and 🧠 RECALL field below has a value. The card has no health or customs questions.
- A plain email address. The site rejects addresses with a `+` alias.
- The user is available for the slider puzzle at the end.

## 3. Sequence of work

1. Open `https://imigresen-online.imi.gov.my/mdac/main` and click **REGISTER**. The site is slow at times: wait
   and retry once before reporting a problem.
2. Fill the single page **in table order**. The order matters:
   - Nationality reloads the phone-code list, so pick it before the phone code.
   - Date of Arrival resets Date of Departure, so pick arrival first.
   - State loads the City list, and City sets the postcode options.
3. **Type every value.** Paste is blocked on almost every box. Dates are read-only: pick them from the calendar.
4. Name: letters and spaces only. Remove hyphens, apostrophes and accents, and list the change as "chosen for
   you".
5. Address: the hotel name and street. The site strips `.` `#` `&` and `:` as you type, so check what the box
   shows afterwards.
6. If the accommodation block isn't shown for this nationality, skip it.
7. The form has no review page. Stop on the filled form and follow step 7 of `arrival-card-fill`.
8. After "submit": use **Take control** for the slider puzzle, then press **Submit**.
9. Find the email from `mdac@imi.gov.my` in Gmail and read the PIN at the bottom. Open **Check Registration**,
   enter the passport number, nationality and PIN, and download the PDF.

For several travellers, repeat the whole form for each one: the web form takes one traveller per submission.

<!-- FIELDS -->

## 4. How to validate the result

The site confirms the registration, and the PIN email arrives. If the email doesn't arrive within a few minutes,
check spam, then say so: without the PIN the PDF can't be downloaded, but the registration is still tied to the
passport.

## 5. What to return

Confirmation, the saved PDF path (or that the PIN email is still missing), and what to show: the passport. An
advance MDAC is needed to use the autogates. A mistake can't be edited: submit a new card.

## 6. What requires approval

- The final **Submit**.
- Filing for a traveller who appears to be exempt.
