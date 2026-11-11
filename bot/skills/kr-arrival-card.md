---
skill: kr-arrival-card
description: Walk the Korea e-Arrival Card on the official site, field by field, from values resolved by arrival-card-fill, reading the email verification code from Gmail. Stops at Check information for the user's 'submit'.
---

# Skill: kr-arrival-card

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for a foreigner arriving in Korea within
3 days (Korea time).

First check whether the traveller needs it at all. A traveller with a **valid K-ETA** who isn't entering on a
visa doesn't file this card: tell them to update their K-ETA record at k-eta.go.kr instead, and stop. Ask if you
don't know. K-ETA is a separate, paid application and this skill doesn't do it.

## 2. Required inputs and access

- Every 🔐 VAULT and 🧠 RECALL field below has a value. The card has no health or customs questions.
- A **contact number in Korea**: the traveller's Korean number, or the hotel's phone from the booking.
- Gmail, to read the verification code, when the card email is the connected mailbox.

## 3. Sequence of work

1. Open `https://www.e-arrivalcard.go.kr/` and set the language to English. Under **Individual**, choose
   **Submit**.
2. **Consent and email.** Tick the three boxes. Type the email in both boxes; the second one blocks paste. Confirm.
3. **Passport information.** The form takes about 15 seconds to open, under a passport-photo popup. Close the
   popup and enter the fields by hand, in table order. For several travellers, use **Add declarant** (up to 9; they
   share the arrival date and flight) and repeat.
4. **Travel itinerary.**
   - Arrival: AIR, the date from the datepicker, then the airline from **Check airline** and the flight number.
     Pad a short number to 3 digits (JQ53 becomes JQ053).
   - Departure: AIR, the date, and the flight number if known. An approximate date is accepted.
   - Purpose and Job: pick from the lists; note a mapped job as "chosen for you".
   - Place of stay: Hotel or Residential. Open the address-search popup, search the hotel's road name or
     building in English, and pick the matching row: the base address can't be typed. Then type the hotel name
     in the detailed address.
   - Contact Information: the number in Korea, digits only.
5. Go to **Check information**. Stop there and follow step 7 of `arrival-card-fill`.
6. After "submit": request the verification code and read it from Gmail (it's valid for 5 minutes; three wrong
   tries force a resend). If the email isn't the connected mailbox, ask the user for the code. Hand over for the
   CAPTCHA popup with Take control, then submit.
7. **Save the PDF with the issue number immediately.** The site's own notice says confirmation emails to Gmail
   and Outlook are sometimes delayed or lost.

<!-- FIELDS -->

## 4. How to validate the result

The completion page shows an issue number for each traveller and offers the PDF. If a duplicate-details warning
appeared, the earlier card was replaced: say so.

## 5. What to return

Issue number(s), the saved PDF path, and what to show: the passport only, because the card is matched to it.
The card lapses 72 hours after submission. To change the purpose, stay or departure later, use **Check and Edit**
with the issue number, passport number and date of birth.

## 6. What requires approval

- The final submit.
- Proceeding past a duplicate-details warning, because it deletes an earlier card.
