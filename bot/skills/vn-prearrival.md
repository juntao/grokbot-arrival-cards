---
skill: vn-prearrival
description: Fill Vietnam's Pre-arrival Information (digital arrival card) on the official immigration portal, from values resolved by arrival-card-fill, reading the email code from Gmail. Stops at Review & Submit for the user's 'submit'.
---

# Skill: vn-prearrival

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for a foreigner flying into Vietnam
within 3 days. Vietnamese passport holders and transit passengers don't file it.

It is not a visa. The form asks for the traveller's existing visa or exemption and checks it. If the traveller
needs an e-visa and has none, say so now: this skill doesn't apply for one.

## 2. Required inputs and access

- Every 🔐 VAULT and 🧠 RECALL field below has a value. The card has no health or customs questions.
- The traveller's **entry basis** for Vietnam: visa-exempt by nationality, an e-visa (with its 9-digit number and
  dates), or another visa or card. Take it from the Vault's `visa_VN=` line, or ask.
- The user is available for the image CAPTCHA at the start.
- Gmail, to read the 6-digit code at submit, when the email is the connected mailbox.
- **Everything ready before you start**: the page reloads after 60 seconds idle and sessions expire.

## 3. Sequence of work

1. Open `https://prearrival.immigration.gov.vn/`. Check the address: fake sites that ask for card details exist,
   and this form is free. Set the language to English.
2. Choose **Create & Submit Pre-arrival Information**. Use **Take control** for the CAPTCHA.
3. **Skip** the passport scan and select the nationality.
4. **Passenger Information**, in table order.
   - Arrival date: one of the three buttons. If the date isn't offered, the window isn't open: stop and say when
     to try again. The buttons follow the computer's date, so near midnight check them against Vietnam time.
   - Visa: tick the "read and understood" box, pick the visa type that matches the entry basis, then the number
     and dates if it asks. If the page says the visa or passport number is invalid, stop and ask.
   - For companions, use **Add Passenger +**. Each needs their own passport details, email and relationship.
     Everyone shares the same flight and accommodation.
5. **Trip Information.**
   - Flight: pick it from the search list. Use "Other" only if it's truly missing.
   - Check the Border Gate that fills in is the airport on the ticket.
   - Accommodation: province, then ward, then the hotel from the list. If the hotel isn't listed, use "Other"
     and type its address.
6. **Review & Submit.** Stop there and follow step 7 of `arrival-card-fill`.
7. After "submit": tick the confirmation, press **Submit**, read the 6-digit code from Gmail (or ask the user;
   a new code can be requested after 60 seconds), and enter it.
8. Save the QR code and the PDF. Note the **Pre-Arrival Information Number** in the trip log.

<!-- FIELDS -->

## 4. How to validate the result

The page shows "Your submission is successful!" with a QR code and a Pre-Arrival Information Number, and an email
copy arrives. A "duplicate" error means a declaration already exists for this traveller and date: look it up
instead of filing again.

## 5. What to return

The number, the saved QR and PDF paths, and what to show: passport plus the QR code. To change it, use **Lookup &
Update Declaration** with the number and the lead passport number. The arrival date, passport number, date of
birth and nationality can't be changed: cancel and file again.

## 6. What requires approval

- The confirmation tick and **Submit**.
- Choosing a visa type when the entry basis isn't clear.
