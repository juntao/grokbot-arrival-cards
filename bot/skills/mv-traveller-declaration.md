---
skill: mv-traveller-declaration
description: Walk the Maldives IMUGA Traveller Declaration on the official immigration site, step by step, from values resolved by arrival-card-fill, reading the email code from Gmail. Stops at the last step for the user's 'submit'.
---

# Skill: mv-traveller-declaration

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for a traveller flying into the Maldives
within 96 hours. Don't open the site earlier: early filings are reported as rejected.

## 2. Required inputs and access

- Every 🔐 VAULT and 🧠 RECALL field below has a value, including a **face photo** of each traveller (the Vault
  hint `portrait_photo=`, or one attached in this chat).
- The user's own answers, from this conversation, to the health and customs questions.
- Gmail, to read the 6-digit code, when the email is the connected mailbox.

**The labels in the table below are descriptions, not the site's exact wording.** The steps behind the email code
haven't been seen. Match each field to what the page shows. If the page asks for something that isn't in the
table, stop and ask the user: don't guess.

## 3. Sequence of work

1. Open `https://imuga.immigration.gov.mv/traveller/declaration`. Check the address ends in `.gov.mv`: paid
   look-alike sites exist, and this form is free.
2. **Verify email.** Enter the email and send the code. Read the 6-digit code from Gmail, or ask the user for it.
3. **Purpose.** Pick the card that matches. A tourist is **Holiday**.
4. **Personal.** Enter the passport details by hand if the page allows it. If it insists on a passport scan,
   upload the kept passport photo, then check every value it read against the Vault and correct any difference.
   Upload the face photo and adjust the crop so the whole face shows.
5. **Travel.** Pick the airline and then the flight from the lists. Pick the place of stay from the site's list
   and give the island. If the resort or guesthouse isn't listed, ask.
6. **Health.** Answer with the user's answers. If an Ebola block or a vaccination question appears that the user
   wasn't asked, stop and ask, showing the page's wording.
7. **Customs.** Read the list of declarable items from the page and check it against what the user was asked.
   Enter the number of checked bags.
8. For family or a group, add them at the end as the page offers.
9. Stop before the final submit and follow step 7 of `arrival-card-fill`. Submit after "submit".
10. Find the confirmation email and save the QR code. Keep the **edit token** from it in the trip log: it's
    needed to change the declaration.

<!-- FIELDS -->

## 4. How to validate the result

The confirmation email arrives with a QR code. The site's **Verify** page (surname and passport number) finds
the declaration.

## 5. What to return

Confirmation, the saved QR path, and what to show: passport plus the QR code. Say which on-screen fields differed
from the table, so the catalog can be corrected. A departure declaration is reported as no longer required; say
you didn't file one.

## 6. What requires approval

- The final submit.
- Any health or customs answer the user didn't give in this chat.
- Keeping the face photo in `/workspace`.
