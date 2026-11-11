---
skill: tw-arrival-card
description: Walk the Taiwan Arrival Card (TWAC) on the official NIA site, field by field, from values resolved by arrival-card-fill, reading the email verification code from Gmail. Stops at Review for the user's 'submit'.
---

# Skill: tw-arrival-card

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for an arrival in Taiwan within 7 days.

## 2. Required inputs and access

- Every 🔐 VAULT and 🧠 RECALL field below has a value, including **`departure_flight`**. The site requires the
  flight leaving Taiwan, so never invent it. If there's no onward ticket, it must have been asked on the form.
- Gmail, to read the verification code, when the card email is the connected mailbox.

## 3. Sequence of work

1. Open `https://twac.immigration.gov.tw/`. Click **Submit Arrival Card**, then **START** on the notice page.
2. **Email Verification.** Type the card email, then press **SEND VERIFICATION CODE**. Search Gmail for
   `from:niasys@immigration.gov.tw subject:"TAIWAN ARRIVAL CARD" newer_than:1h`. The code is 6 letters, mixed
   case, and is valid for 10 minutes. Type it and press **VERIFY**. If the email isn't the connected mailbox, ask the
   user for the code. If a visible Turnstile challenge appears, use Take control.
3. **Traveler Information.**
   - Dates are DD/MM/YYYY.
   - For country pickers, type the ISO3 code and pick the `ISO3,NAME` row.
   - For the phone code picker, pick the `65 SGP` style row.
   - Map occupation to the closest entry on the fixed list, and note it as "chosen for you".
   - Visa type is Visa-Exempt for visa-exempt nationalities. Otherwise use the visa from the Vault's `visa_TW=` line.
4. Press **NEXT**. If a dialog names empty fields, fill them, press **CONFIRMATION**, and retry.
5. **Itinerary.**
   - Arrival: AIR, then the flight code from the picker (`SQ : SINGAPORE AIRLINES` style) and the digits.
   - Departure: AIR, then the same pattern for the exit flight.
   - Purpose: from the option list. "Visit Relative" requires the relative's name and mobile.
   - Accommodation: Hotel Name with only letters, digits, spaces and `. , -`, in upper case.
6. Press **NEXT** to reach **Review**. Stop there and follow step 7 of `arrival-card-fill`.
7. After "submit": tick the declaration, press **SUBMIT**, then capture the confirmation and the confirmation email.

<!-- FIELDS -->

## 4. How to validate the result

The confirmation step shows success, and the confirmation email arrives from the NIA. A later submission for the
same entry overrides the earlier one, so a correction means submitting again.

## 5. What to return

Confirmation status, travellers covered, the forwarded confirmation email, and that the passport is shown at
immigration. e-Gate enrolment is a separate counter.

## 6. What requires approval

- The final **SUBMIT**.
- Typing a verification code the user read out. Do it only for this card.
