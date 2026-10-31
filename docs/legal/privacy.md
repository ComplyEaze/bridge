# ComplyEaze Bridge Privacy Policy

**Version:** 2026-10.4

**Effective:** 31 October 2026

**Describes:** ComplyEaze Bridge builds that include or link to this version of the policy, the desktop app built from the same source, and the Bridge website at bridge.complyeaze.com.

> **The short version**
>
> - **ComplyEaze Bridge runs on your computer.** It connects only to a Tally address on the same computer, and hands what it reads to the AI assistant you connect it to.
> - **Bridge sends nothing to ComplyEaze.** It has no feature that uploads files or sends data to us. It has no analytics, usage tracking, crash reporting or automatic update check.
> - **Your AI provider does receive what your assistant reads.** That can include client names, PAN, bank details and amounts. Your assistant sends it to your AI provider (for example Anthropic, for Claude) under your own account and that provider's terms. An optional setting can shorten some names or remove narrations first. This reduces what is shared, but does not make it anonymous, and **it never hides amounts, PAN, GSTINs or many other details** (section 5 lists them).
> - **Bridge keeps files on your computer.** These include a log of every request and **unmasked** copies of any vouchers it prepares or posts, with client names and amounts. Bridge never deletes them by itself, and uninstalling may not remove them. Section 7 explains how to remove or archive them.
> - **You are responsible for your clients' data.** As a professional, you decide what to read, which AI provider to use and what to post. You need your clients' authority to do so.
>
> This summary is for convenience. The full policy below is what applies.

---

## 1. Who we are and what this policy covers

ComplyEaze Bridge ("**Bridge**") is published by **SPMS Comply Eaze Solutions LLP**, a limited liability partnership registered with limited liability under the Limited Liability Partnership Act, 2008, LLP identification number ACI-9231 ("**ComplyEaze**", "**we**", "**us**", "**our**"). Our registered office is at S-137, 1st Floor, Sunsquare Shopping Plaza, Plot No. SPL-1/J, RIICO Chowk, Bhiwadi Ind. Area, Alwar, Bhiwadi, Tijara, Alwar 301019, Rajasthan, India.

This policy explains what happens to information when you use Bridge. It covers:

- the **Bridge extension**: the add-on you install in Claude Desktop, or in another AI app that can use add-on tools;¹
- the **Bridge desktop app**; and
- the **Bridge website** at bridge.complyeaze.com.

It does not cover Tally, your AI assistant or AI provider, GitHub, Cloudflare, your operating system, or other ComplyEaze products and websites (such as complyeaze.com or the Axal workspace). Each of those has its own terms and privacy notice.

This policy is a notice about how information is handled. It is not part of the Bridge Terms of Use.

¹ Technically, the extension is a local server for the Model Context Protocol (MCP), the standard way AI apps connect to tools.

## 2. Words we use

| Term | Meaning in this policy |
| --- | --- |
| **Tally** | TallyPrime (and compatible Tally software). Tally is a product of Tally Solutions Pvt. Ltd., which is not connected with ComplyEaze. |
| **Tally Data** | Anything Bridge reads from Tally, or prepares or posts into Tally: company details, ledgers, vouchers, balances, reports, and the details stored in them. |
| **AI Assistant** | The app on your computer that you connect Bridge to, such as Claude Desktop. |
| **AI Provider** | The company that runs the AI model your AI Assistant uses (for Claude, Anthropic). |
| **Tool request** | A request your AI Assistant makes to Bridge, such as "show the trial balance for April". |
| **Posting** | Using Bridge to create a voucher in Tally. |
| **Local Files** | Files Bridge writes on your own computer (section 7). |
| **Personal Data** | Data about an individual who can be identified by or from it, as defined in the Digital Personal Data Protection Act, 2023. Tally Data often contains Personal Data. Examples are the names, PAN, addresses, phone numbers and bank details of proprietors, partners, employees and other parties. |
| **Clients** | The businesses and people whose books you keep or review in Tally. |
| **You** | The person using Bridge. Where you use it for a firm or company, it also means that firm or company. |

## 3. How information moves when you use Bridge

| Connection | What travels | Who sends it | When |
| --- | --- | --- | --- |
| Between Bridge and Tally, at an address on your computer | Requests to Tally and Tally's replies | Bridge | When your AI Assistant makes a tool request, or you use the desktop app. |
| From Bridge to your AI Assistant, on your computer | The result of each tool request | Bridge | Each time your AI Assistant makes a tool request. |
| From your AI Assistant to your AI Provider, over the internet | Your conversation, including every Bridge result the assistant has read | **Your AI Assistant, not Bridge** | As your AI Assistant works. Your agreement with your AI Provider governs this. |
| From your browser to the Bridge website, Cloudflare and GitHub | Ordinary web requests | Your browser | When you visit the Bridge website (section 8). |

**Bridge connects to Tally only at an address that means "this computer".**² If you or your IT support forward that address somewhere else (for example to Tally inside a virtual machine, as some Mac users do), Bridge's requests go wherever you forwarded them.

In the builds this version describes, Bridge's own code makes no internet connections: its only network connection is to that Tally address. Bridge also includes a third-party library, PDFium, which it loads only to read a bank-statement PDF you name. Bridge has no analytics, usage tracking, advertising, crash-reporting or automatic-update code.

² Bridge accepts only the addresses `localhost`, `127.0.0.1` (or any `127.x.x.x`) and `::1`. It does not use a proxy or follow redirects.

## 4. What ComplyEaze receives, and what it does not

**Through Bridge on your computer, we do not receive** any of the following, so we cannot see, retrieve, correct or delete them:

- your Tally Data;
- your conversations with your AI Assistant;
- your Local Files;
- usage statistics, crash reports or device identifiers.

**As the publisher of Bridge, we receive information only when** you contact us by email, through GitHub or otherwise, for example in a support request, bug report or vulnerability report. We also receive limited information when you visit the Bridge website (section 8).

## 5. What your AI Assistant and AI Provider receive

Bridge's job is to give your AI Assistant the Tally Data it asks for. **Anything your AI Assistant reads through Bridge becomes part of your conversation and is sent to your AI Provider.** Depending on the tool requests, this can include:

- company names and identifiers;
- ledger and party names, groups, and balances;
- vouchers, including dates, amounts, narrations, references, voucher numbers and party GSTINs;
- outstanding bills and ageing;
- trial balance, profit and loss and balance sheet figures;
- when the assistant asks for them, ledger details such as PAN, name on PAN, GSTIN, email, phone, address, PIN code, bank account number and holder name, IFSC and MSME/Udyam numbers;
- the contents of any bank-statement PDF you ask Bridge to read; and
- the location of files Bridge saves on your computer, and of the files your assistant names (such as a bank statement and its password file), both of which usually include your computer user name.

When Bridge reads a bank statement, your AI Assistant gives it an account label, and Bridge returns the last four characters of the statement's account number (for most banks, its last four digits). Your assistant writes the label, so your AI Provider receives it under every setting. The label carries at least the account number's last four digits, and can carry more of it or all of it; a statement that prints its account number in part may need the whole number in the label. Bridge also writes the label into the narration of every voucher it prepares from the statement, so it is sent again whenever your assistant reads those vouchers, unless you choose `drop_narration`.

**Your AI Provider's own terms and privacy policy decide** how long it keeps this information, whether it uses it to train models, and who at the provider can see it. They also decide where it is processed, which may be outside India. These depend on the provider, and on the plan and settings you choose. Your AI Assistant may also keep its own records of conversations and results on your computer. Read your AI Provider's and AI Assistant's terms before you connect Bridge to real client books.

**Optional masking (the "Response redaction" setting).**

Bridge has one optional setting, **Response redaction**, that changes what it gives your AI Assistant. You type one of three values exactly as shown. Any other value stops the extension from starting.

| Value | What it does | What it does **not** do |
| --- | --- | --- |
| `none` (the default) | Nothing is masked or removed. | Everything is shared as read. |
| `mask_parties` | Shortens to their first two and last two characters the names of parties and ledgers, stock items, stock groups, godowns, cost centres and cost categories. It does the same to the name on PAN, and to the bank account holder's name and bank account number in a ledger's details. Anything of four characters or fewer becomes an ellipsis (…). It also removes Tally's error messages, which can repeat names, and it leaves out some lists of similar or near-miss ledger names. | For example, it does **not** mask amounts, dates, company names, narrations, references, voucher numbers, GSTINs, PAN numbers, MSME or Udyam numbers, email addresses, phone numbers, postal addresses, PIN codes, IFSCs, or the last four characters of a bank statement's account number. Account group names are sent as the book has them, including in the Balance Sheet and Profit and Loss lines; only the line names in those two reports' checks against Tally's own reports are shortened. A name or account number written inside a narration or reference is not masked. |
| `drop_narration` | Removes narration fields. It also removes Tally's error messages. | It does not mask any names, and it does not remove amounts or any other text field. |

**Amounts are never masked or removed, and neither are the last four characters of a bank statement's account number that Bridge returns when it reads one. Your AI Provider always receives the account label your AI Assistant gives it.** A shortened name can still identify a person, especially alongside other details. Masking reduces what your AI Provider receives. It does not make the data anonymous, and it does not by itself meet any legal duty you may have. It applies only to what Bridge gives your AI Assistant. The Local Files in section 7 are stored unmasked.

## 6. No uploads to ComplyEaze

Bridge has no feature that uploads files or sends data to ComplyEaze. If we add one, we will update this policy before we publish a build that includes it.

## 7. Files Bridge keeps on your computer

Bridge writes Local Files so that you can see what it did, and so that a posting can be checked or reconciled later. **The files in the table below stay until you delete them. Bridge does not delete or trim them automatically, and removing the extension may leave them in place.**

**Where to find them:**

- **Mac:** in Finder, choose Go, then Go to Folder, and type `~/Library/Application Support/Bridge`.
- **Windows:** in File Explorer, type `%LOCALAPPDATA%\Bridge\agent` in the address bar. On some computers it is `%APPDATA%\Bridge\agent` instead. On a Microsoft Store install of Claude Desktop, Windows may keep these files in the Claude package's own folder under `%LOCALAPPDATA%\Packages`, at `LocalCache\Local\Bridge\agent`, instead. On one computer we measured, Bridge started from inside the Claude package, as the extension is, wrote its files to that package folder, while Bridge started outside the package used `%LOCALAPPDATA%\Bridge\agent`, so the two kept separate files. The location that `build_import_xml` reports for an import file is the file's real location, and it opened from outside the package. Check both folders before you archive or delete. Microsoft's documentation says that uninstalling or resetting a Store app (here Claude Desktop itself) removes that second folder and everything in it.

If someone set Bridge up by hand to use a different folder,³ that folder holds the files below. Small, empty lock files still go in the default folder. On Windows, Bridge also puts lock files in a second folder, `%USERPROFILE%\.complyeaze-bridge`, so that every copy of Bridge you run finds the same ones. They keep two copies of Bridge from sending to Tally at the same time, and Bridge does not delete them.

On a Mac, Bridge restricts its folder to your user account through file permissions. On Windows it uses the folder's normal permissions. Administrators and backup software on your computer may still be able to read these files.

| File or folder | What it holds | Contains amounts or names? |
| --- | --- | --- |
| `agent-egress.jsonl` (the request log) | Two lines for each tool call: an entry for the result Bridge returns, and a second line that records that the response was written out. Each entry records the time, the tool, Tally's internal code for the company, counts, field names, sizes and the masking setting. It also records error codes, the dates of any period that failed to read, and fingerprints of the request and result.⁴ For each tool call, the entry also lists the last 32 requests Bridge sent to Tally for that call (their kind, size, outcome and timing) and counts the rest. | Designed to hold no amounts, names or narrations. Sizes can hint at the length of a name, so treat this file as confidential too |
| `agent-import-ledger.jsonl` (the import journal) | Every voucher batch Bridge prepared or posted, with its status | **Yes**: full voucher details, unmasked |
| `imports/` | The Tally import files and the read-back record for each batch. Also review records, which include your computer user name | **Yes**: unmasked |
| `bank-statements/` | Voucher proposals and each statement row in the dates you chose, or all of them if you chose none (its date, amount and the party as printed) from bank statements you asked Bridge to read, with the balances you gave and the account label, which carries at least the last four digits of the account number | **Yes**: unmasked |
| `agent-import-statement-skips.jsonl` (the skipped-rows file) | Rows of a bank statement that you chose to skip because the other bank account's statement records them, kept when a statement had no row to post. For each row: an ID made from a fingerprint of the row as the statement printed it, its date, amount, direction, Tally's internal code for the other ledger, the bank reference, the cheque or instrument number, and whether the statement shows the same amount going the other way that day. Also a record ID, Tally's internal codes for the company and the bank ledger, the Tally address, the time, and the dates Bridge read the statement in full, when there are any | **Yes**: amounts, bank references and cheque numbers, unmasked. Designed to hold no names or narrations, but a cheque or reference number is copied as the bank printed it |
| `terms-acceptance.jsonl` | The version of the Terms of Use you accepted, the time Bridge first started with the acceptance setting on, and that it was accepted through the setting | No |

If you built and used the desktop app, it keeps more files in folders named `com.complyeaze.bridge`:

- an encrypted copy of Tally records it has read. Its key is kept in your computer's password store (Keychain on a Mac, Credential Manager on Windows) under the name `com.complyeaze.bridge.tally-mirror`;
- your client-label and sort preferences; and
- the Tally address you last used, in the app's own storage.

Reports you export from the desktop app are saved to your Downloads folder or a folder you choose. Older versions may also have left a file named `bridge-exports.sha256`: a list of fingerprints of exported files, with no names or contents. It is no longer used and can be deleted.

Some information is held only in memory while Bridge runs, and is lost when it stops:

- recent read records;
- temporary result pages (kept for at most 10 minutes);
- posting approvals; and
- any bank-statement password.

**Deleting and archiving local data.** Bridge does not delete these records by itself, and it does not know how long you must keep records. Tally remains your book of record, and you should keep the bank's original statements.

- **Bank statements and other working files.** We plan to add a tool that deletes Bridge's copies of bank statements it has read and its other working files, after you confirm in a separate window. It will remove all of them together; it cannot remove one client's or one person's. It will not delete the request log (`agent-egress.jsonl`), which is kept unless you delete the file yourself.
- **A report of what is stored.** Your AI Assistant can ask Bridge for a report of what it has stored, by kind: how many files, how much space, and how old the oldest is, but not file names or contents. The report also says whether any batch is still "not settled", and says so when it could not read something. It covers Bridge's own data folder, not files kept by the Bridge desktop app.
- **Records of what you posted.** Bridge will not delete its import journal or its `imports/` folder. Bridge uses the journal to remember which batches it has already sent to Tally, so it can refuse to send one twice and check a later batch against them. It uses the proofs and review records in the `imports/` folder to tell you what happened to a batch it posted. The journal also records which bank-statement rows Bridge has already sent to Tally. Bridge refuses to include a row again, when building or posting a batch, if an earlier batch sent it or a read-back found it posted. That check has limits: it cannot see a repeat under a new transaction ID, a voucher typed by hand in Tally, or a batch recorded only in another computer's journal. To retire these records:
  1. Quit your AI Assistant, and the desktop app if you used it.
  2. Check in Tally that everything you posted through Bridge is there. Use the report above: if it shows batches that are not settled, check each one in Tally, then archive anyway. A batch that Tally rejected stays not settled, and that is normal.
  3. Move the whole Bridge folder (on a Microsoft Store install of Claude Desktop, both folders; see section 7, where to find them) into an encrypted archive or encrypted disk image. Do not simply delete it.
  4. Bridge then starts fresh and no longer remembers what it sent, including which bank-statement rows it sent. It compares a new batch with what your Tally book shows, but that may not catch a repeat that differs from the original, so check Tally before posting old entries again.
- **Everything else.** If you used the desktop app, delete its `com.complyeaze.bridge` folders and the `com.complyeaze.bridge.tally-mirror` entry in your password store. Delete any reports you exported. Remove the extension in your AI Assistant's settings. Removing it may leave the Bridge folder, its lock files and the password-store entry in place; if the Bridge folder is still there when you reinstall, Bridge picks up its records again. On Windows, also delete the `%USERPROFILE%\.complyeaze-bridge` folder once your AI Assistant and the desktop app are closed. Bridge keeps only lock files there. On a Microsoft Store install of Claude Desktop, check the second Bridge folder named in section 7 too: a copy of the files can remain there after you delete the first.

Bridge cannot remove one person's or one client's data. Deleting or archiving its files changes nothing in Tally, and does not affect anything your AI Assistant or AI Provider has kept. Copies may also remain elsewhere: in Tally, in reports you exported, in Time Machine or File History backups, in folders synced to a cloud service, in your AI conversations, or on the disk until the space is reused.

**Protecting Local Files.** They are only as safe as your computer. Use a device password and keep your operating system updated. Turn on full-disk encryption (FileVault on a Mac, BitLocker on Windows). Do not share these files, or screenshots of them, when asking for help.

³ Using the `BRIDGE_AGENT_DATA_DIR` setting.
⁴ A fingerprint (a SHA-256 hash) is a short code calculated from the content. It lets you check whether two copies match, but the content cannot be read back from it.

## 8. The Bridge website

The Bridge website is a set of static pages hosted on GitHub Pages, behind Cloudflare. It has no account, sign-up or contact form, and no analytics code of our own.

- **Hosting.** GitHub and Cloudflare receive standard request details, such as your IP address, browser type and the page requested, and may keep them under their own policies.
- **Cloudflare security.** Cloudflare may run a security check in your browser and set a security cookie (`cf_clearance`). That cookie applies across complyeaze.com websites and can last up to a year. Cloudflare may also receive reports if a page fails to load. We keep this protection on because it helps block automated attacks on the site.
- **GitHub.** The download and release pages ask GitHub (`api.github.com`) directly from your browser for the list of releases, so GitHub receives your IP address and browser details under its own privacy statement. If GitHub does not answer, the pages use a copy published with the site. Other pages do not contact GitHub.
- **Colour theme.** The site remembers your choice of colour theme in your browser's local storage. It is not sent to us or to anyone else, and you can clear it in your browser.

## 9. Why we use the information we receive

We use information that reaches us (section 4) only to:

- answer your questions, support requests and bug or vulnerability reports;
- keep Bridge, our website and our services secure, and investigate misuse;
- publish, with your separate written agreement, words you give us about ComplyEaze Bridge, with the description of you and of any connection with us that your agreement states, and keep the record of that agreement;
- comply with the law and respond to lawful requests from authorities; and
- establish, exercise or defend legal claims.

We do not sell personal data. We do not use it for advertising, except as the third purpose above allows.

## 10. Sharing

We share information that reaches us (section 4) only:

- with service providers who host or run our website and email, such as GitHub and Cloudflare for the website and our email provider, under their terms of service;
- where required by law, a court order or a lawful request from a government authority;
- to protect the rights, safety or property of our users, the public or ComplyEaze;
- as part of a merger, acquisition or transfer of our business, subject to this policy; or
- with the public, when we publish words you have agreed in writing that we may publish (section 9).

## 11. Roles and your responsibilities as a professional

**Roles.** Data-protection law gives different duties to the person who decides why and how personal data is used (a "Data Fiduciary") and to anyone who handles it on their behalf (a "Data Processor").

ComplyEaze does not collect, receive, store or have access to the Tally Data that Bridge handles on your computer. You decide which Tally companies to connect, what to read, which AI Provider to use and what to post. On that basis, we consider that we are neither a Data Fiduciary nor a Data Processor for that Tally Data. Those responsibilities rest with you and, where relevant, your Clients. Your AI Provider handles what your AI Assistant sends it under your agreement with that provider.

**Your responsibilities.** Because you control the data, you are responsible for:

- having each Client's authority to access their books with Bridge and to share their data with your AI Provider. You must also give any notice, or obtain any consent, that the law or your engagement requires;
- meeting your professional duties of confidentiality where they apply to you, including under the Chartered Accountants Act, 1949 and your professional body's code of ethics;
- choosing an AI Provider, plan and settings that suit your Clients' data, such as data-retention and model-training settings, and deciding whether to use masking;
- keeping your computer, Tally and your AI Assistant account secure;
- keeping backups of your Tally data before you turn posting on, and regularly while it is on; and
- handling Local Files and exports securely, and deleting them when you no longer need them.

## 12. Your rights

For personal data **we hold** (section 4), you may ask us for information about it, or ask us to correct, complete or erase it. You may also raise a grievance with our Grievance Officer (section 16).

Once the relevant parts of the Digital Personal Data Protection Act, 2023 and its Rules are in force, you may also use the other rights they give you, such as nominating someone to act for you. We may need to verify your identity before acting, and we may keep information where the law requires us to, or to establish, exercise or defend legal claims (section 13).

You can withdraw your agreement to publish your words at any time by writing to contact@complyeaze.com. We will then remove them from pages we control and not use them again, but copies already in the website's public source history or in web archives may remain.

For Tally Data on your computer, we hold nothing, so we cannot act on a request about it. You can view, correct or delete it yourself, in Tally and in the Local Files. For data your AI Assistant sent to your AI Provider, contact your AI Provider.

If one of your Clients, or anyone whose details appear in their books, asks about their data, you, not ComplyEaze, are the right person to answer.

## 13. Retention

- **Tally Data and Local Files:** kept on your computer until you delete them (section 7). We do not hold them.
- **Emails, support requests and reports you send us:** kept as long as needed to deal with them, and for any longer period the law requires, and then deleted, normally no more than three years after the matter is closed.
- **Words you agreed we may publish, the description that goes with them, and your written agreement:** kept while the words are published, and for three years after they are removed or you withdraw your agreement, so that we can show we had it.
- **Website request logs:** kept by GitHub and Cloudflare under their own policies.

## 14. Security

In the builds this version describes, Bridge does the following to reduce risk:

- It connects to Tally only at an address on your computer, and refuses any other address, proxy or redirect.
- The desktop app's window is not allowed to load anything from the internet.
- **Posting is off by default in new installations.** In the Claude Desktop extension, from the build after 0.5.1, the posting setting is a new one. If you turned posting on in an earlier version, turn it on again if you still want it; in that build, posting also includes batches of up to 50 vouchers. (In 0.5.1 and earlier, the extension posts one voucher at a time unless `BRIDGE_AGENT_ENABLE_BATCH_POST` was set on by hand.) If you set `BRIDGE_AGENT_ENABLE_WRITES` yourself outside the extension, it carries over and from that build also allows batches. That build retires `BRIDGE_AGENT_ENABLE_BATCH_POST`: while posting is on, if it is set to any value, Bridge refuses every posting your AI Assistant asks for until you remove it and restart Bridge.
- When posting is on, each posting waits for your approval in a separate window on your computer; a batch of up to 50 vouchers is approved once, from its summary.
- Bridge gives your AI Assistant no way to approve a posting. Software that can control your screen could still click the button, so do not let such software run while a posting is waiting.
- An approval is used once, stays valid for at most 15 minutes after you give it, and is kept only in memory.
- After posting, Bridge reads the voucher back from Tally to check what arrived.
- On a Mac, its data folder and files are restricted to your user account through file permissions.
- The desktop app's copy of Tally records is encrypted, with the key kept in your computer's password store.
- Bridge's source code is public, so anyone can inspect what it does.

No software is completely secure.

**We check each release before we publish it: the release check confirms that each package launches, lists its tools and parses a synthetic encrypted bank statement. It does not run against TallyPrime, and nothing we can run covers every Tally edition, set of books or setting.** Run by us in a controlled test against a real TallyPrime, once, on 16 October 2026: posting with the release candidate's package on one synthetic company of TallyPrime 7.1 Silver on Windows 10 (each of the four voucher types, and one batch of three), with the approval window showing in front of Tally and Claude Desktop on one screen. Not yet run by us in a controlled test against a real TallyPrime: posting with the published package (the files tested were the release candidate's package and a rehearsal build of the release recipe, not the tagged file); the approval window on a second screen; posting on TallyPrime Education; posting on TallyPrime Gold with its approval step recorded; the upgrade from 0.5.1. What has been run, on which Tally edition and operating system, is listed in the README and each release's notes. Builds are not signed with a verified publisher identity or notarised: Apple and Microsoft have not certified them as coming from us.

Download builds only from the official release page (https://github.com/ComplyEaze/bridge/releases), or from the Bridge website, which downloads from it. Each build has a matching fingerprint file (a `.sha256` checksum) on the same release; compare it with your download before opening it. That shows the file arrived whole; it does not show who made it.

To report a security issue, write to security@complyeaze.com (section 16). If we fix a security issue, the fix will be in a new release; we are not obliged to make one (Terms of Use, section 4.4).

## 15. Children

Bridge is a professional tool for accountants and businesses. It is not intended for anyone under 18, and we do not knowingly collect personal data from children. If you believe a child has sent us personal data, contact us and we will delete it.

## 16. Contact and Grievance Officer

- **Privacy questions and requests:** contact@complyeaze.com
- **Grievance Officer:** a Designated Partner of SPMS Comply Eaze Solutions LLP acts as our Grievance Officer and the person who answers questions about personal data. Write to contact@complyeaze.com with "Grievance" in the subject, or by post to the address in section 1. We will acknowledge a grievance within 7 days of receiving it and give you our response within one month.
- **Security vulnerabilities:** security@complyeaze.com, or GitHub private vulnerability reporting for the Bridge repository. We aim to acknowledge a report within seven days. Please do not report a vulnerability in a public issue.

If you are not satisfied with our response, you may be able to complain to the Data Protection Board of India once the relevant provisions are in force.

## 17. Changes to this policy

We will update this policy when Bridge changes in a way that affects your information, for example before we publish a build that sends data anywhere new. Each version has a version number and date at the top, and is kept in the public history of the Bridge repository. The version linked from a build describes that build. We will describe important changes in the release notes of the build that introduces them.
