---
skill: ph-etravel
description: Register a Philippines eTravel arrival declaration (immigration, health and customs) on the official site, signing in with the Vault login, from values resolved by arrival-card-fill. Stops at the Summary for the user's 'submit'.
---

# Skill: ph-etravel

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for anyone arriving in the Philippines
within 72 hours. The airline checks the QR code before boarding, so do it before the user leaves for the airport.

## 2. Required inputs and access

- A Vault **Login** for `etravel.gov.ph`. If there isn't one, ask approval to create the account with the card
  email, and let Vault generate the password.
- Gmail, for the 6-digit code when creating the account.
- Every 🔐 VAULT and 🧠 RECALL field below has a value, and the user's own answers, from this conversation, to the
  three health questions and the customs question.

The page order below is inferred, and some wording is only shown after sign-in. Match each field to what the page
shows. If the page asks for something that isn't in the table, stop and ask the user.

## 3. Sequence of work

1. Open `https://etravel.gov.ph/`. Use **Take control** for the Cloudflare check on the sign-in page.
2. Sign in with the Vault login. For a new account: enter the email, read the 6-digit code from Gmail (if you
   need another, wait 3 minutes), then create the password.
3. **Profile**, the first time only: choose FOREIGN PASSPORT and fill the table's Profile fields. For companions,
   use **Add Family Member** and fill a profile for each.
4. **New Travel Declaration**: FOR ME, AIR, ARRIVAL. Tick the family members travelling.
5. **Travel Details**, in table order. Airline, flight, airports and hotel are look-ups: type, then pick the row.
   If the hotel isn't found, ask.
6. **Health Declaration**: the user's answers. If any is Yes, tell the user the QR code will be red, which means
   a stop at the quarantine desk on arrival. Don't change an answer to avoid that.
7. **Summary.** Stop there and follow step 7 of `arrival-card-fill`. Submit after "submit".
8. **Customs.** Answer "Do you have baggage or currency to declare?" with the user's answer. For Yes, fill the
   follow-up pages from what the user gave you, and use Take control for the drawn signature.
9. Download the QR code for each traveller and save it.

<!-- FIELDS -->

## 4. How to validate the result

The dashboard shows the declaration with a QR code. Say whether it's green or red.

## 5. What to return

QR colour, the saved QR path for each traveller, and what to show: the QR code to the airline at check-in and
again on arrival, with the passport. The declaration can be edited or deleted from the dashboard until the border
officer has processed it. Say which on-screen fields differed from the table.

## 6. What requires approval

- Creating the eTravel account.
- The final **Submit**.
- Any health or customs answer the user didn't give in this chat.
