---
skill: th-arrival-card
description: Walk the Thailand Digital Arrival Card (TDAC) on the official site, field by field, from values resolved by arrival-card-fill, after the user passes the Cloudflare check. Stops at Preview for the user's 'submit'.
---

# Skill: th-arrival-card

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for a non-Thai traveller arriving in
Thailand within 3 days.

## 2. Required inputs and access

- Every 🔐 VAULT and 🧠 RECALL field below has a value, and the user has listed the **countries they stayed in
  during the two weeks before arrival** in this conversation.
- **Everything must be ready before you start.** The site can't save progress and sessions time out, so don't open
  it with questions still open.
- The user is available for the Cloudflare check on the home page.

## 3. Sequence of work

1. Open `https://tdac.immigration.go.th/`. The home page shows a Cloudflare "verify you are human" check, and the
   **Arrival Card** button stays disabled until it passes. Use **Take control** and ask the user to complete the
   check, then say "continue". Never try to get around it.
2. Click **Arrival Card**. Skip "Retrieve with Passport" and enter by hand.
3. **Personal Information**, in table order. Autocomplete boxes need 3 typed characters, then a click on the row.
   The date of birth is three selects: year first.
4. **Trip & Accommodation Information.**
   - Country where you boarded: the country of the flight that lands in Thailand, not where the trip began.
   - Purpose, mode of transport and accommodation type: read the options from the page and pick the closest. If
     none fits, choose "OTHERS (PLEASE SPECIFY)" and type it.
   - Departure: fill all of the departure boxes or none. With no onward ticket, leave them all empty.
   - Accommodation: province first. District and sub-district are needed unless the type is HOTEL. The address is
     the hotel name and street, in English characters.
5. **Health Declaration.** Select the countries the user listed. If the yellow-fever question or the symptom list
   appears and the user hasn't answered it, stop and ask, showing the page's exact options.
6. For more travellers, choose **Add Other Travelers** (up to 10) and use "Same as Previous Traveler" for the trip
   and stay. Otherwise choose **Preview**.
7. On **Preview**, type the email. Stop there and follow step 7 of `arrival-card-fill`.
8. After "submit": tick the terms, press **Submit**, and download the PDF with the QR code.

If the session times out or the site returns an error on submit (it documents HTTP 429 and "0 Unknown Error"),
refresh and start again from step 1, which needs the user for the check again. Tell them why.

<!-- FIELDS -->

## 4. How to validate the result

The page shows a "TH Digital Arrival Card No." and the PDF downloads. The same PDF arrives by email, and the
email says whether the traveller can use the automated gates.

## 5. What to return

Card number(s), the saved PDF path, the gate status from the email, and what to show: passport plus the QR code.
A wrong name, nationality or date of birth can't be edited: submit a new card, and the latest one counts. Other
fields can be changed with **Update Arrival Card** on the home page.

## 6. What requires approval

- The final **Submit**.
- Any answer on the health declaration that the user didn't give in this chat.
