---
skill: in-arrival-card
description: Fill the India e-Arrival Card on the official Bureau of Immigration page, from values resolved by arrival-card-fill. Stops on the filled form for the user's 'submit' and the CAPTCHA.
---

# Skill: in-arrival-card

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for a foreign national arriving in India
within 72 hours. Indian citizens don't file it.

The card is not a visa, and it doesn't cover India's separate health self-declaration (Air Suvidha) or customs.
Tell the user to check whether those apply to them: this skill doesn't file them.

## 2. Required inputs and access

- Every 🔐 VAULT and 🧠 RECALL field below has a value. The form is short: it doesn't ask for the flight, date of
  birth or visa number.
- The user's answer, from this conversation, to **countries visited in the last 6 days**.
- An address in India with a street or hotel name, plus its state and district.

## 3. Sequence of work

1. Open `https://indianvisaonline.gov.in/earrival/`. Check the address: paid look-alike sites exist, and this
   card is free.
2. Fill the single page in table order.
   - Purpose: read the list from the page and pick the closest. If none fits, choose OTHER and type it.
   - Date of Arrival: pick it from the date picker. If the date can't be selected, the window isn't open: stop and
     say when to try again.
   - Countries visited: select the ones the user listed, or tick "No Country Visited".
   - State, then District.
3. For companions, use **Add Member** (up to five). They share the arrival date, address and contact details.
4. The form has no review page. Stop on the filled form and follow step 7 of `arrival-card-fill`.
5. After "submit": tick the declaration, use **Take control** for the image CAPTCHA, and submit.
6. A message shows the File Number for each traveller and a PDF downloads for each. Save them.

For a repeat visitor, "Quick Fill From Existing Data" at the top can reuse an earlier card. It needs its own
CAPTCHA, so only use it if the user prefers.

<!-- FIELDS -->

## 4. How to validate the result

A File Number is shown and `Arrival-Card-<file number>.pdf` downloads. It can be downloaded again from "Download DE
Card" with the file number.

## 5. What to return

File number(s), the saved PDF path, and what to show: the passport, with the PDF as backup. A mistake can't be
edited: file again. Repeat the reminder about the health self-declaration and customs.

## 6. What requires approval

- The declaration tick and the final submit.
