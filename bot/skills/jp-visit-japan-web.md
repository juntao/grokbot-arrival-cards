---
skill: jp-visit-japan-web
description: Register a trip in Visit Japan Web (immigration Disembarkation Card + customs declaration, one QR), signing in with the Vault login, from values resolved by arrival-card-fill. Stops before each Register for the user's 'submit'.
---

# Skill: jp-visit-japan-web

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for an arrival in Japan. VJW is optional,
but it saves time at both counters.

## 2. Required inputs and access

- A Vault **Login** for `vjw.digital.go.jp`. If there isn't one, ask approval to create an account with the card
  email. Let Vault generate the password: at least 10 characters with upper case, lower case, a digit and a symbol.
- Gmail, for the account confirmation code.
- Every 🔐 VAULT and 🧠 RECALL field below has a value. All 3 immigration and 8 customs ✍️ ASK questions have
  answers from this conversation.

## 3. Sequence of work

1. Open `https://www.vjw.digital.go.jp/` and set the language to English. Sign in with the Vault login.
   - If 2-factor authentication is on, use Take control for the authenticator code.
   - For a new account, the "I am human" check is the user's (Take control). Read the email confirmation code
     from Gmail (`from:vjw newer_than:1h`, or the subject "Visit Japan Web").
2. **Your details.** Skip this if it's already registered and matches the Vault, but fix any mismatch.
   - Answer No to "Japanese passport" and No to "re-entry permit".
   - Use **manual entry**, because there's no camera on the cloud computer.
   - Type names in UPPER CASE, and the passport number twice.
   - Leave "Contact information in Japan" blank unless the user always stays at the same address.
   - Register each companion here as a family member, up to 10.
3. **Register planned entry/return.**
   - Trip name: the trip id.
   - Arrival date: mm/dd/yyyy.
   - Airline from the dropdown, and the flight number as digits only. For a codeshare, use the **operating**
     carrier. If the airline isn't listed, choose "Other" and type the full code.
   - Postal code as digits, then press the auto-fill button for the prefecture and city. Then the address in UPPER
     CASE romaji, the hotel name, and the hotel phone as digits.
   - Tick the travelling family members.
4. **Disembarkation Card for Foreigner.** Fill home country and city, purpose, last flight and length of stay
   (years / months / days). Answer the 3 Yes/No questions with the user's own answers. Stop before **Register**
   and follow step 7 of `arrival-card-fill`. Register after "submit".
5. **Customs declaration.**
   - Occupation: the closest dropdown option.
   - Date and flight.
   - Point of embarkation, from the type-ahead list.
   - Family counts by age band.
   - The 8 questions, answered from the user's answers. If any of ①–⑥ is Yes, fill the goods detail the user gave.
   - Stop before **Register**, then register after "submit". If ⑦ is Yes, tell the user they must also file a
     means-of-payment declaration at the counter.
6. Open the QR screen ("Display QR code"). Screenshot the combined immigration and customs QR for each traveller
   and save it.

<!-- FIELDS -->

## 4. How to validate the result

- Both the Disembarkation Card and the customs declaration show **Registered** (or "Completed") for each traveller.
- A QR code is shown. If anything is edited later, the old QR becomes invalid: take a new screenshot.

## 5. What to return

Registration status for each traveller, QR screenshots saved (paths), and a reminder: show the QR at the
immigration kiosk and again at customs. Paper forms remain a fallback. JESTA, the pre-travel authorisation, is
planned for FY2028 and isn't needed yet.

## 6. What requires approval

- Creating a VJW account.
- Each **Register** (immigration, then customs).
- Any Yes answer to a customs question, which the user must give explicitly.
