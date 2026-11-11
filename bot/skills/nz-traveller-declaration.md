---
skill: nz-traveller-declaration
description: Walk the New Zealand Traveller Declaration (NZTD) web form, section by section, from values and declaration answers resolved by arrival-card-fill. Stops before Submit for the user's 'submit'.
---

# Skill: nz-traveller-declaration

## 1. When to use it

Called by `arrival-card-fill` after the answers are resolved and saved, for anyone travelling into New Zealand,
**no earlier than 24 hours before their trip to New Zealand starts**. The form refuses trip details before that.
With a stopover where the traveller collects their bags, count from the flight that leaves the stopover.

It is separate from the NZeTA and from visas. A traveller who needs an NZeTA and has none should be told now:
this skill doesn't apply for it.

## 2. Required inputs and access

- Every 🔐 VAULT and 🧠 RECALL field below has a value.
- **Every ✍️ ASK question answered by the user in this conversation**: about 20 Yes/No questions on
  immigration history, biosecurity and restricted items, plus the list of countries in the last 30 days. Ask them
  on the gap form with the exact wording in the table. Remind the user that packaged snacks, tea and coffee count
  as food, and that used hiking boots and sports gear count as outdoor gear.
- **Everything ready before you start**: the session ends after 10 minutes idle and 30 minutes in total.

This is a legal declaration with an instant fine of at least NZ$400 for a false answer. Never answer a question
for the user, and never assume No.

## 3. Sequence of work

1. Open `https://app.travellerdeclaration.govt.nz/` and choose **New declaration**. Tick the privacy box.
2. **Contact details**, then **Passport details**, in table order. Use the passport the traveller will enter on.
3. **Trip details.** Search the flight number. If it isn't found, use "Manually add flight details". If the form
   says it's too early, stop and tell the user when it opens.
4. A reference number is emailed now. Note it in the trip log.
5. Complete each section from the hub, in table order: **Purpose of travel**, **Immigration status**, **Travel
   history**, **Biosecurity**, **Restricted items**.
   - Immigration status: with an NZeTA and no visa, the option is "travelling visa waiver". With a visa, the
     matching visa option. If unsure, ask.
   - For each Yes, the page opens follow-up choices. Use what the user told you. If they didn't cover a
     follow-up, stop and ask with the page's options.
6. On **Submit**, answer "Do you know the contents of your baggage?" with the user's answer. Stop there and
   follow step 7 of `arrival-card-fill`: the summary must list every Yes/No answer.
7. After "submit": tick the declaration and press **Submit**. If a "Confirm you are not a robot" check appears,
   use Take control.

For another traveller, use "Start a declaration for someone else" and repeat: each person needs their own, and the
form asks whether you have their consent or guardianship.

<!-- FIELDS -->

## 4. How to validate the result

The page shows "Submission successful" with the reference number, and a second email arrives.

## 5. What to return

Reference number and what to show: the passport only. There's nothing to print and no QR code. Tell the user
that anything declared Yes must be shown to an officer on arrival, and that any change before arrival, such as a
new flight or something bought on the way, means editing and submitting again ("Edit declaration" with the email
and reference number).

## 6. What requires approval

- The declaration tick and **Submit**.
- Every Yes/No answer: all must come from the user in this chat.
- Declaring consent to file for another traveller.
