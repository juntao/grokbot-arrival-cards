---
skill: sg-arrival-card
description: Walk the Singapore SG Arrival Card + health declaration on the official ICA site, field by field, from values resolved by arrival-card-fill. Stops at Review for the user's 'submit'.
---

# Skill: sg-arrival-card

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for an arrival in Singapore inside the
filing window. Never run it directly from a user request without the gap form first.

## 2. Required inputs and access

- Every 🔐 VAULT and 🧠 RECALL field in the table below has a value, and every ✍️ ASK field has an answer the user
  gave in this conversation.
- Browser on the cloud computer. No login is needed.

## 3. Sequence of work

1. Open `https://eservices.ica.gov.sg/sgarrivalcard/` in a new tab. Check the address bar says
   `eservices.ica.gov.sg`.
2. Choose **Fill in Manually** under the foreign-visitor card, not the resident one.
3. Fill **Traveller Details** in table order. Then:
   - Choose radios by their visible text, because their ids change every session.
   - Dates are three boxes: type DD, MM, YYYY.
   - For look-ups (nationality, places, cities, flight code, hotel), type the start of the value and click the
     matching row. A typed value that isn't picked from the list doesn't count.
   - If the page shows a health or outbreak question not in the table, stop. Ask the user that question
     word-for-word and use their answer.
4. For several travellers, use **Add traveller** and repeat step 3 for each one.
5. Press **Next** and fill **Visit Details**. Use the accommodation branch that matches `stay_type`. If the hotel
   isn't in the list, choose OTHERS and type the hotel name.
6. Press **Next** to reach **Review**. Stop there and follow step 7 of `arrival-card-fill`: screenshot, summary,
   wait for "submit".
7. After "submit": tick the declaration, press **Submit**, and capture the confirmation page. ICA emails an e-Pass
   to the card email. Find it in Gmail (`from:ica.gov.sg newer_than:1h`) and forward it if it went to a different
   address.

<!-- FIELDS -->

## 4. How to validate the result

- The confirmation page shows a submission reference, and the e-Pass email arrived.
- The date of arrival on the confirmation matches the flight's **local** arrival date.

## 5. What to return

Reference number, travellers covered, the e-Pass email status, and that the passport alone is shown at the gate.
The card is linked to the passport, so there's no QR to present.

## 6. What requires approval

- The final **Submit**.
- Any answer to a health question that the user didn't give in this chat.
