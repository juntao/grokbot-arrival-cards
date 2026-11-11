---
skill: kh-arrival-card
description: Walk the Cambodia e-Arrival Card on the official site, field by field, from values resolved by arrival-card-fill. Stops at Confirm Declaration for the user's 'submit'.
---

# Skill: kh-arrival-card

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for a foreigner flying into Cambodia
within 7 days. The form lists airports only: for a land or sea arrival, say it isn't covered and stop.

This card is not a visa. If the traveller needs a visa, the e-visa is a separate application this skill
doesn't do.

## 2. Required inputs and access

- Every 🔐 VAULT and 🧠 RECALL field below has a value, including the **hotel's phone number**, which the form
  requires. Take it from the booking email or the hotel's website; never invent it.
- The user's own answers, from this conversation, to the three health questions and the two customs questions.

## 3. Sequence of work

1. Open `https://arrival.gov.kh/`. Click **Submit e-Arrival**, then **Sign In as Guest**. Wait 6 to 8 seconds
   for the form.
2. **Profile**, in table order.
   - Pickers: click the box, type in the dialog's Search box, click the row.
   - Dates: three dropdowns each.
   - Occupation: the closest of the 17 options; note it as "chosen for you".
3. Press **Next**. A dialog reads back the passport number and country. Check both against the Vault, then
   **Correct**.
4. **Travel Information.** Wait for the page to load. The occupation, email and phone boxes are empty again:
   fill them a second time. Then the trip and the stay, in table order. Pick "Next country" from its list, not with
   the "Same as last" box.
5. **Health Information.** Answer the three questions with the user's answers. Read each question's full wording
   from the page first; if it differs from what the user was asked, ask again.
6. **Customs Declaration.** Tick Yes or No for both questions, then the "true and correct" box, then **Next**.
7. **Confirm Declaration.** Stop there and follow step 7 of `arrival-card-fill`.
8. After "submit": tick "I have read and agreed to the declaration." and press **Submit**. Save what the
   confirmation page shows.

If a step shows "This field is required." under a box, fill that box and press Next again.

<!-- FIELDS -->

## 4. How to validate the result

The site shows a confirmation after Submit. The confirmation page hasn't been mapped, so describe what it shows
and save a screenshot along with any QR code or reference.

## 5. What to return

What the confirmation shows, the saved path, and what to present: passport, plus the QR code if one was issued.
Mention that each visit to the site leaves a saved draft.

## 6. What requires approval

- The final declaration tick and **Submit**.
- Any health or customs answer the user didn't give in this chat.
