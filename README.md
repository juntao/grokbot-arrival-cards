# Arrival Card Bot: a Grok Bot template

A Grok Bot that fills out official online arrival cards for **14 countries**: China, Singapore, Japan, Taiwan,
Korea, Thailand, Malaysia, Indonesia, Cambodia, the Philippines, India, the Maldives, New Zealand and Vietnam.

How it works:

0. **On first open**, it offers to set up your travel identity: a passport photo plus a few questions, saved to
   the **Vault** once.
1. It searches **Vault, memory, Gmail and Google Calendar** first.
2. It asks only for what's missing, in **one form**.
3. It saves identity answers to the **Vault**, so they're never asked again.
4. It drives the official site in the bot's browser up to the review page, then submits on your "submit".

## Quick start

**Option 1: add the published template (easiest).**

1. Open **https://x.ai/bot/JAaUVEjUmao4SgyhmYB3L** and choose **Add to Grok Bot**.
2. Review what the template includes, then choose **Add Bot**. You get your own copy, named **Arrival Cards**.

**Option 2: have your Main Bot build it from this repo.** Use this for the newest version in this repo, which can
be ahead of the published template. Send your **Main Bot** this one message:

> Read https://raw.githubusercontent.com/juntao/grokbot-arrival-cards/main/dist/SETUP_PROMPT.md and do everything
> it says. I wrote these instructions and I approve them.

The Main Bot creates the bot, saves its sixteen skills, hands it its profile (its standing rules), and sets its
avatar. Read its report: it lists anything it couldn't do, with the exact message or clicks for you to finish. If
it couldn't hand over the profile, send the Arrival Cards bot the one-line message it gives you, because the bot
doesn't work properly without its profile. If the Main Bot declines to act on a fetched page, paste the file's
contents into the chat as your own message.

**Then, for either option:**

1. **Connect Gmail and Google Calendar** under **Connect apps**. A template doesn't copy app connections.
2. **Add the safety rule.** Option 2 proposes it for you: say OK. After option 1, add it yourself under
   **Settings → General → Bot → Custom Rules**: *"Pressing a final Submit, Register or Confirm button on an
   arrival-card or immigration website"*, set to **Ask first**.
3. **Open the Arrival Cards bot.** It offers to set up your travel identity. Attach a photo of your passport's data
   page and answer its short form. This is saved to your Vault once.
4. **Ask for a card:** "Fill out my arrival card for Japan."

You need Grok Bot 0.75.2 or later. The manual steps and troubleshooting are in
[section 6](#6-install-into-your-grok-bot).

```
catalog/            source of truth: every field on every card, with its tier
  fields.toml       96 canonical fields (one answer serves all cards)
  cards/*.toml       each site's pages, labels, formats and options
bot/                hand-written bot text
  PROFILE.md        job, sources, rules, autonomy
  SETUP_HEADER.md   setup steps for the user's Main Bot (top of SETUP_PROMPT.md)
  skills/*.md       onboarding (<!-- VAULT_FIELDS -->) + orchestrator + 4 site walkthroughs (<!-- FIELDS -->)
src/                Rust build tool: validates catalog/, renders dist/
assets/             avatar.png (still), avatar.gif / avatar.webp (animated), from tools/make_avatar.py
dist/               GENERATED, paste-ready: PROFILE, skills, FIELD_MATRIX, SETUP_PROMPT
```

---

## 1. What Grok Bot is (research summary, Nov 2026)

[Grok Bot](https://docs.x.ai/grok-bot/overview) is SpaceXAI's (formerly xAI) agent product. The facts below shape
this template.

| Capability | What the docs say | How this template uses it |
|---|---|---|
| **Cloud computer** | Each account has one cloud computer: browser, command line, `/workspace`. Each Bot gets its own screen. You can watch, or **Take control**. | Fills the fourteen official sites. Saves proof to `/workspace/arrival-cards/<trip>/`. |
| **Memory** | Each Bot has its own memory: preferences, rules, and a dated work log. Name and time zone are shared across Bots. Logins and cards don't belong there. | Trip facts go in the trip log, lasting preferences as preferences. Declarations are never stored. |
| **Vault** (≥ 0.75.2) | Three entry types: **Logins**, **Cards**, **Contact info** (name, address, phone, email), each with a nickname and **Hints**. A Bot *can* create and update contact info, nicknames and hints. Passwords and card numbers are filled without the Bot seeing them. | One Contact info entry per traveller, `Travel identity · GIVEN SURNAME`. Passport data goes in `key=value` hint lines (see §4). The Visit Japan Web account is a Vault Login. |
| **Secure Form** | For values only you can give. Values go straight into the page and are never shown to the Bot. A "Securely save to Bot's vault" box appears for logins, cards and contact info. | Used automatically by Grok Bot for site passwords. |
| **Connected apps** | Gmail (search, read, draft, send), Google Calendar (read, free/busy, events), Drive and others. | Gmail finds e-tickets, bookings and verification codes. Calendar finds trip dates and purpose. |
| **Skills** | Reusable instructions, saved by asking the Bot. Recommended six parts: *when to use · inputs & access · sequence · validation · what to return · what needs approval*. No file format is documented. | Every skill in `bot/skills/` follows those six headings. |
| **Routines** | Run a skill on a schedule, or on an email, Slack, webhook or similar trigger. Runs happen in the cloud. | Optional: "file the SG card on the morning its window opens". |
| **Auto Review** | Custom rules in plain words, set to *Ask first* or *Allow automatically*. | One *Ask first* rule for any final Submit or Register on an immigration site. |
| **Templates** | "A recipe, not a clone." A template carries **instructions, relevant memories, first-party skills, plugins and routines**. It does **not** carry secrets, personal memories, private data, **custom code or scripts**, or MCP servers. | So the bot runs on skill text alone. The Rust tool is build-time only. |

Sources: [How Grok Bot works](https://docs.x.ai/grok-bot/how-grok-bot-works) ·
[Bots](https://docs.x.ai/grok-bot/bots) · [Vault](https://docs.x.ai/grok-bot/vault) ·
[Computer & apps](https://docs.x.ai/grok-bot/computer-and-apps) ·
[Skills & routines](https://docs.x.ai/grok-bot/skills-routines-and-automations) ·
[Templates for Grok Bot](https://x.ai/bot/guides/templates-for-grok-bot).

## 2. The fourteen cards

| | Card | Official URL | Window | Handed to you |
|---|---|---|---|---|
| CN | NIA online arrival card | s.nia.gov.cn/ArrivalCardFillingPC | any time before arrival | passport photo (unless attached), drawn signature, Submit |
| SG | SG Arrival Card + health declaration | eservices.ica.gov.sg/sgarrivalcard | 3 days before, arrival day included | Submit |
| JP | Visit Japan Web | vjw.digital.go.jp | any time; ≥ 6 h before departure | new-account CAPTCHA, 2FA, each Register |
| TW | Taiwan Arrival Card | twac.immigration.gov.tw | **7** days before, arrival day included | email code (unless it's in your Gmail), Submit |
| KR | Korea e-Arrival Card | e-arrivalcard.go.kr | 3 days before (Korea time); lapses after 72 h | CAPTCHA, email code (unless it's in your Gmail), Submit |
| TH | Thailand Digital Arrival Card | tdac.immigration.go.th | 3 days before, arrival day included | Cloudflare check at the start, Submit |
| MY | Malaysia Digital Arrival Card | imigresen-online.imi.gov.my/mdac | 3 days before, arrival day included | slider puzzle, Submit |
| ID | All Indonesia arrival card | allindonesia.imigrasi.go.id | 3 days before | image CAPTCHA, certification tick, Submit |
| KH | Cambodia e-Arrival Card | arrival.gov.kh | 7 days before; air arrivals only | declaration tick, Submit |
| PH | Philippines eTravel | etravel.gov.ph | 72 hours before | Cloudflare check at sign-in, account approval, Submit |
| IN | India e-Arrival Card | indianvisaonline.gov.in/earrival | 72 hours before | image CAPTCHA, Submit |
| MV | Maldives IMUGA Traveller Declaration | imuga.immigration.gov.mv | 96 hours before the flight | email code (unless it's in your Gmail), Submit |
| NZ | New Zealand Traveller Declaration | travellerdeclaration.govt.nz | 24 hours before the trip starts | about 20 declaration answers, Submit |
| VN | Vietnam Pre-arrival Information | prearrival.immigration.gov.vn | 3 days before, arrival day included | CAPTCHA at the start, email code (unless it's in your Gmail), Submit |

The field lists come from live-site walks of SG, CN and TW (Oct–Nov 2026), and KH (Nov 2026). For KR, TH, MY, ID, PH, IN, MV, NZ and VN they come from
the sites' own form code, label bundles and manuals. They were re-checked on 2026-11-11
against the sites' own JS and i18n bundles, plus the Digital Agency VJW manual and Japan Customs form C5360 for JP.
Each card file records its own `verified` date and sources.

## 3. How a request flows

```
first open, no "Travel identity" in Vault
  └─ arrival-card-onboarding: passport photo → read VIZ + MRZ → ask the rest → save to Vault (+ keep photo for CN?)

"Fill my Japan arrival card"
  │
  ├─ 1. identify trip ── memory trip log → Calendar (next 60 days) → Gmail (e-tickets, bookings)
  ├─ 2. resolve fields ─ per traveller × per card, each canonical key through its tier:
  │       🔐 VAULT   Vault entry → passport photo in chat
  │       🧠 RECALL  trip log → Gmail → Calendar → derived → memory preference
  │       ✍️ ASK     nothing: always asked (health, travel history, customs, criminal history)
  ├─ 3. ONE gap form ── "Already known (with sources)" + numbered questions for the gaps only
  ├─ 4. save back ──── 🔐 → Vault entry · 🧠 → trip log / preference · ✍️ → nowhere
  ├─ 5. window check ─ not open yet → offer a routine for the opening day
  ├─ 6. drive site ─── site skill, page by page; hand-overs via Take control
  ├─ 7. review ─────── screenshot + summary → waits for "submit"
  └─ 8. proof ──────── /workspace/arrival-cards/<trip>/, forward confirmation email, log ref in memory
```

[`dist/FIELD_MATRIX.md`](dist/FIELD_MATRIX.md) is the complete answer list. It shows every field, its tier, where
it's found, and which of the 4 cards need it.

## 4. Design decisions you should know

- **Passport data in Vault hints.** Vault has no passport or identity entry type today. Using a Contact info
  entry's hints is the only place a Bot can both **write** to and read from, which is what "answer once, never
  asked again" requires. The trade-off: unlike card numbers, hint values are visible to the Bot, as they must be to
  type them into the form. The bot never echoes the full number in chat or memory. If Grok Bot adds an identity
  entry type, the skill tells the bot to switch to it.
- **Passport photo.** Onboarding reads it once, cross-checking the printed fields against the two
  machine-readable lines at the bottom of the page. It keeps a copy in `/workspace/arrival-cards/identity/` only if
  you say yes, because China's card requires an upload. Note that `/workspace` is shared by all your Bots.
- **Declarations are never cached.** Health, the SG 6-day yellow-fever list, China's 2-year travel history, JP
  immigration and customs Yes/No questions, China's lost-passport question, Thailand's two-week travel list, and Indonesia's health, quarantine and customs
  questions are all asked fresh. These are legal
  declarations, and a stale "No" is a false statement. For China's travel history, trips found in email may be
  offered as a starting list.
- **The bot stops before Submit.** It fills everything, then submits only after you reply "submit". The setup also
  proposes an Auto Review *Ask first* rule, so the stop is enforced by the platform and not only by the prompt.
- **One answer, all cards.** Cards point to canonical keys, so a passport number given for Singapore fills Japan,
  every other card too.

## 5. Build

You need Rust 1.85+ (2024 edition).

```sh
cargo run -- check          # validate catalog/ (unknown keys, tier rules, unused fields, https URLs, …)
cargo run -- render         # regenerate dist/ from catalog/ + bot/
cargo run -- card JP        # print one card's generated field reference
cargo test                  # catalog + renderer tests (includes "the real catalog is sound")
```

To change a field, edit `catalog/`. To change behaviour, edit `bot/`. Then run `render`. Never edit `dist/` by
hand.

## 6. Install into your Grok Bot

Installing takes about 10 minutes. You don't need Rust or this repo's source for it: the paste-ready files in
`dist/` are committed. Raw base URL:
`https://raw.githubusercontent.com/juntao/grokbot-arrival-cards/main/`

### Before you start

- **Grok Bot 0.75.2 or later** (the Vault needs it), on desktop. The steps below use the desktop app's names.
- A **Gmail** mailbox and a **Google Calendar** that hold your travel bookings. Gmail connects one mailbox at a
  time, so pick the one your e-tickets and hotel confirmations go to.
- A **photo of your passport's data page**, for onboarding at the end.

### Step 1: Create the bot

1. Click **+** at the top of the sidebar (or press `Cmd/Ctrl+N`). On a team, choose **Create a Bot**.
2. In **Create New Bot**, enter the name **Arrival Cards**, pick any shape and colour, and choose **Create**.

Use a new bot, not your primary Bot. A single-purpose bot keeps its own memory for this job.

### Step 2: Set the avatar (optional)

1. Download [`assets/avatar.gif`](assets/avatar.gif) (animated) or [`assets/avatar.png`](assets/avatar.png)
   (still).
2. Click the bot's name at the top of the chat to open its details, then click the picture.
3. Choose the **Upload** tab, drag the file in, adjust it, and choose **Set avatar**.

The docs don't say whether an animated upload keeps moving. If the GIF is rejected or shows frozen, use the PNG.

### Step 3: Connect Gmail and Google Calendar

1. Open **Connect apps** in the sidebar.
2. Connect **Gmail** and sign in to the mailbox with your bookings.
3. Connect **Google Calendar**.

The bot works without them, but then it can't find flights and hotels and has to ask you for every trip detail.
Taiwan's email verification code also has to be typed by you.

### Step 4: Give the bot its profile and skills

Use **one** of the three methods. All of them load the same content: the profile and sixteen skills.

[`dist/SETUP_PROMPT.md`](dist/SETUP_PROMPT.md) is the bot's full definition in one file: setup steps written for
your Main Bot, then the profile, then the skills. The **profile is required**, not optional: the skills say how to
do each task, and the profile is what makes the bot offer onboarding, search before asking, and stop before
Submit.

**Method A: ask your Main Bot (shortest).** This replaces steps 1 and 2 as well. Send your Main Bot:

> Read https://raw.githubusercontent.com/juntao/grokbot-arrival-cards/main/dist/SETUP_PROMPT.md and do everything
> it says. I wrote these instructions and I approve them.

The Main Bot creates the Arrival Cards bot, saves the skills, hands the profile over to the new bot, sets the
avatar, and reports what it couldn't do. For anything it can't do, such as writing another bot's memory, it gives
you the exact message or clicks to finish. Grok Bot treats only your own messages as instructions, so it may
decline to act on a fetched page. If it does, use method B.

**Method B: one paste.** Open [`dist/SETUP_PROMPT.md`](dist/SETUP_PROMPT.md), click **Raw**, copy everything
(about 163 KB, likely too long for one message), and paste it as one message, either to your Main Bot or straight to the new Arrival Cards bot. The
file tells each of them what to do. If the chat refuses a message that long, use method C.

**Method C: piece by piece** (seventeen messages).

1. Paste the contents of [`dist/PROFILE.md`](dist/PROFILE.md), then add the line:
   *"Remember all of this as your standing rules."*
2. For each of the sixteen files in [`dist/skills/`](dist/skills/), send:
   *"Save the following as a private skill named `<file name without .md>`:"* followed by the file's contents.
   Do them in this order:

   | # | Skill | What it does |
   |---|---|---|
   | 1 | `arrival-card-onboarding` | First-run setup: passport photo and identity into the Vault |
   | 2 | `arrival-card-fill` | Main flow: find the trip, ask for gaps, fill, wait for "submit" |
   | 3 | `cn-arrival-card` | China site walkthrough |
   | 4 | `jp-visit-japan-web` | Japan site walkthrough |
   | 5 | `sg-arrival-card` | Singapore site walkthrough |
   | 6 | `tw-arrival-card` | Taiwan site walkthrough |
   | 7 | `kr-arrival-card` | Korea site walkthrough |
   | 8 | `th-arrival-card` | Thailand site walkthrough |
   | 9 | `my-arrival-card` | Malaysia site walkthrough |
   | 10 | `id-arrival-card` | Indonesia site walkthrough |
   | 11 | `kh-arrival-card` | Cambodia site walkthrough |
   | 12 | `ph-etravel` | Philippines site walkthrough |
   | 13 | `in-arrival-card` | India site walkthrough |
   | 14 | `mv-traveller-declaration` | Maldives site walkthrough |
   | 15 | `nz-traveller-declaration` | New Zealand site walkthrough |
   | 16 | `vn-prearrival` | Vietnam site walkthrough |

3. Send: *"Propose an Auto Review rule, Ask first: pressing a final Submit, Register or Confirm button on an
   arrival-card or immigration website. Then run the arrival-card-onboarding skill."*

### Step 5: Check what was saved

The bot should reply with a checklist. Confirm it yourself:

1. Ask: *"What are your standing rules?"* It should mention searching Vault, memory, email and calendar before
   asking, one form, and stopping before Submit.
2. Ask: *"List your skills."* All sixteen names from the table above should appear. If one is missing, open
   **Connect apps**, select the connected count next to the title, and enable it for this bot under **Private
   skills**. If it isn't there either, repeat method C for that skill.

### Step 6: Approve the safety rule

The setup proposes one Auto Review rule. Reply OK to create it, or add it yourself: **Settings → General →
Bot → Custom Rules**, write *"Pressing a final Submit, Register or Confirm button on an arrival-card or
immigration website"*, and set it to **Ask first**.

This makes the platform, not only the prompt, stop the bot before it files a legal declaration.

### Step 7: Onboarding, your travel identity

The setup ends by starting onboarding, and every new conversation starts with the same offer until the Vault has a
`Travel identity` entry.

1. Attach the photo of your passport's data page. The bot reads your name, passport number, dates and
   nationality.
2. Answer its one form for what a passport doesn't show: phone, email, home city, occupation, and any visas for
   the countries that need one.
3. Say Yes or No to keeping a copy of the photo. China's card requires a passport photo upload, so No means you'll
   be asked for it on every China trip.
4. Repeat for anyone who usually travels with you, or say "done".
5. Check the result in **Settings → Vault**: one Contact info entry named `Travel identity · GIVEN SURNAME`, with
   your details as `key=value` lines under **Hints**.

### Step 8: First real card

1. With a real trip coming up, say: *"Fill out my arrival card for Singapore"* (or any of the other seven countries).
2. The bot shows an **Already known** block with the source of each value, then a numbered form for the gaps and
   the declaration questions. Correct anything wrong and answer the form.
3. Watch it fill the site: open the computer preview in the right-hand panel. If it asks you to **Take control**
   (CAPTCHA, signature, 2-factor code), do only that step and tell it to continue.
4. On the review page it sends you a summary and waits. Reply **submit** to file, or tell it what to change.

If the card's window isn't open yet (3 days before arrival for most, 7 for Taiwan), the bot keeps your answers
and offers a routine that files on the opening day.

### If something goes wrong

| Problem | Fix |
|---|---|
| Bot doesn't offer onboarding in a new chat | Say *"set up my travel profile"*. Then ask it to repeat its standing rules; if the "First thing, every conversation" rule is missing, paste `dist/PROFILE.md` again. |
| "I can't find that skill" | Enable it under **Connect apps → (connected count) → Private skills**. |
| It asks for something that's in your email | Check Gmail is connected to the right mailbox, then say *"search my email again for the e-ticket"*. |
| It asks for your passport number again | Open **Settings → Vault** and check the `Travel identity` entry's Hints has a `passport_no=` line. |
| A site's labels or steps changed | Tell the bot what changed so it can finish. Then fix `catalog/cards/<cc>.toml`, run `cargo run -- render`, and re-save that skill. |

### Updating later

Pull this repo, then re-send the changed skill files from `dist/skills/` with *"Replace your skill `<name>` with
the following:"*. Your Vault entries and memory are untouched.

## 7. Publish it as a template, and install someone else's

**Publish** (from the working bot):

1. Click the bot's name, then **Share**, then **Create a copy**. On a phone: **Share as Template**.
2. **View Details**. Check that the template includes the 5 skills and the standing rules, and **no** personal
   memories (no trip logs, names or passport details). Vault entries are never included.
3. Choose **Team** or public, then **Publish**, then **Copy link**. Later edits: **Share → Update template**.

**Install** (as a recipient). The published template is https://x.ai/bot/JAaUVEjUmao4SgyhmYB3L.

1. Open the link, then **Add to Grok Bot**. Review the context and integrations, then **Add Bot**. Or use
   **Connect apps → Bot templates → Import Bot**.
2. Connect **Gmail** and **Google Calendar** yourself, because plugins and apps are not copied.
3. Re-create the Auto Review *Ask first* rule, because account settings are not copied.
4. Your Vault starts empty, so the bot opens by offering onboarding: your passport photo and a few questions.

## 8. Limits and next steps

- **JP:** the screens behind sign-in were mapped from the official manual and the customs form, not walked live.
  Verify the labels on the first real run and update `catalog/cards/jp.toml`.
- **Visa-free lists change often** (China especially). The CN skill tells the bot to verify the current list and
  ask when unsure. The list is never hard-coded.
- **TH:** the form sits behind a Cloudflare check that only you can pass, so it has not been walked live. Its
  labels come from the site's own label bundle, but several dropdown lists were not verified. The bot reads the
  options from the page.
- **MV:** the steps behind the email code were not seen, so that card's labels are descriptions, not the site's
  wording. Expect corrections after the first real run.
- **PH:** needs an eTravel account, and the page order is inferred from the site's code.
- **IN:** the card doesn't cover India's separate health self-declaration or customs.
- **NZ:** about 20 declaration questions, all asked every trip, and a 30-minute session limit.
- **KH:** the form was walked live to the last page, but who is exempt was not confirmed.
- **VN:** officially still a pilot at five airports, and sources differ on whether it is mandatory. The card asks
  for your visa or exemption but is not a visa.
- **Australia:** not supported. There is no general online arrival card yet: paper is still the default, with a
  digital pilot only on some Qantas flights and into Western Sydney. National rollout is planned by the end of 2027.
- **MY:** Singapore citizens and some pass holders are reported exempt, but that list comes from press and embassy
  notices, not the form. The web form takes one traveller per submission and can't be edited afterwards.
- **ID:** the card is not a visa. Visa on Arrival and the Bali tourist levy are separate, and the bot doesn't pay
  for either.
- **KR:** travellers with a valid K-ETA don't file this card. The bot asks, and doesn't apply for K-ETA.
- **Multiple travellers:** SG, TW, KR, TH and ID batch travellers in one submission, and JP uses account family members. CN
  and MY need one card per person.
- **Adding a country:** add `catalog/cards/<cc>.toml` and `bot/skills/<skill>.md` with the six headings and a
  `<!-- FIELDS -->` marker. Reuse the canonical keys, and add new ones only when needed. Then run `check` and
  `render`.
