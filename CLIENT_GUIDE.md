# **Client Behavior Guide: Payment-Aware Professional Events (Optional)**

**NIP Reference:** 
**Status:** Draft / Informational

---

## **Purpose**

Some clients may wish to **reduce spam or prioritize trusted events** by considering optional micropayments attached to professional events (job postings, applications, etc.).

This guide defines **recommended practices** for clients handling the `payment_proof` field introduced in NIP-25001.

---

## **Payment Proof Structure**

All payment proofs are optional and contained in the `content` JSON under the key `payment_proof`:

```json
"payment_proof": {
  "amount_sats": 1000,
  "invoice": "lnbc1...",
  "txid": "<optional-on-chain-txid>"
}
```

* `amount_sats` — numeric amount paid in satoshis.
* `invoice` — Lightning invoice or fiat-to-sats reference.
* `txid` — optional on-chain transaction ID.

Clients **MUST NOT reject** events solely because `payment_proof` is missing or invalid.

---

## **Recommended Client Behaviors**

### **1. Display & Prioritize**

Clients MAY:

* Highlight or badge events with a valid `payment_proof`.
* Sort or filter job listings by `amount_sats` or recency of payment.
* Allow users to **optionally hide unpaid events**.

### **2. Validation**

Clients MAY:

* Check that the `amount_sats` is positive.
* Optionally verify the Lightning invoice or transaction to ensure it was paid.
* Display the payer or issuer pubkey if available.

**Note:** Verification is optional; clients can still display events even if payment cannot be independently validated.

### **3. Anti-Spam Heuristics**

Clients MAY:

* Use payment amount as a scoring factor for event trustworthiness.
* Prefer events with higher `amount_sats` when displaying lists.
* Combine payment scoring with other heuristics (reputation of issuer, endorsements, etc.).

### **4. Interaction with Relays**

* Relays **do not need to validate or filter** events based on payment.
* Clients MAY filter or ignore unpaid events locally.
* All events remain standard Nostr events; backwards compatibility is preserved.

---

## **Examples**

### Highlighting Paid Jobs

```text
[💰 Paid Job] Senior Rust Engineer – 1000 sats
[ ] Backend Developer – unpaid
```

### Prioritized Feed Algorithm (Pseudo-Code)

```python
events.sort(key=lambda e: e.get('payment_proof', {}).get('amount_sats', 0), reverse=True)
```

### Optional User Preferences

* `show_unpaid_jobs`: True / False
* `min_payment_sats`: 500

---

## **Rationale**

* Preventing spam is a **local, client-side choice**, not a network-wide rule.
* Payment handling is **opt-in**, ensuring **backwards compatibility**.
* Users or organizations can **signal commitment or trustworthiness** via optional payments.
* Supports both **Lightning-native sats payments** and **fiat-to-sats gateways**.

---

## **Security Considerations**

* Clients must **never require** payment to view events.
* Displayed amounts should not be considered a cryptographic guarantee — they only indicate the issuer’s intent.
* Avoid storing sensitive payment credentials in client storage.
* Clients MAY combine payment with other verifiable proofs for trust scoring.

---

This guide provides a **flexible, fully optional approach** for clients that want to support paid prioritization or spam mitigation while keeping Nostr fully decentralized.

