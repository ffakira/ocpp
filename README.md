# Draft Specification: Offline Cashless Payment Protocol (OCPP)

## 1. Introduction

This document defines a protocol for conducting offline, cashless transactions within a closed or semi-closed loop (inner loop) system. The protocol ensures transactional integrity, replay protection, and offline security guarantees, leveraging established cryptographic primitives from existing RFCs.

## 2. Terminology

- **Payer:** The entity initiating the transaction.
- **Merchant:** The entity accepting the transaction.
- **Transaction Token:**  A signed data structure representing the transaction.
- **Offline Terminal:** The device validating and processing the transaction locally.
- **Settlement Server:** The centralized backend system reconcilling transactions.

## 3. Cryptographic Primitives

This protocol integrates cryptographic standards defined in:

- **RFC 8017:** PKCS #1 RSA Cryptography Specification v2.2
- RFC 5869: HMAC-based Extract-and-Expand Key Derivation Function (HKDF)
- RFC 7515: JSON Web Signature (JWS)
- ED25519: High-speed, high-security public key signature system (FC 8032)
- X25519: Diffie-Hellman key exchange over Curve25519 (RFC 7748) for optional ephemeral key exchange

## 4. Transaction Flow (Inner Loop)

### 4.1 Transaction Token Generation (by Payer)

- Payer device generates a transaction token containing:
    - `transaction_id`
    - `payer_id`
    - `merchant_id`
    - `amount`
    - `timestamp`
    - `nonce` (to prevent replay attacks)
- Token is signed using either:
    - JWS with RSA PKCS#1 v1.5 (as per RFC 8017),
    - Ed25519 signatures (as per RFC 8032), or
    - EC25519 (X25519-based ephemeral key derived signatures for inner-loop transactions)

### 4.2 Transaction Validation (by Offline Terminal)

- Verify token signature using the corresponding algorithm:
    - JWS signature verification (RFC 7515),
    - Ed25519 signature verficiation (RFC 8032), or
    - EC25519 ephemeral key signature validation (established via X25519 session exchange)
- Derive a session key using **X25519 Diffie-Hellman** exchange between payer’s ephemeral public key and termina’s private key.
- Apply **HKDF (RFC 5869)** to the shared secret for clean key derivation.
- Validate HMAC signature over the payload using derived session key.
- Check token freshness via `timestamp` and uniqueness of `nonce`.
- Deduct amount from local balance (if sufficient).
- Record transaction log locally with
    - `transaction_id`
    - `payer_ephemeral_pub`
    - `HMAC`
    - `amount`
    - `merchant_id`
    - `timestamp`
    - `nonce`
    - `result` (accepted / denied)

### 4.3 Settlement (Outer loop)

- When back online, terminal syncs transaction logs to Settlement Server.
- Server verifies logs against transaction tokens.
- Recomputes shared secrets using `payer_ephemeral_pub` and terminal’s private key.
- Derives session keys via HKDF.
- Verifies HMACs against stored payloads.
- Reconciles balances accordingly.

### 4.4 Whitelist-Based Pre-Authorization (Optional)

- Settlement server can store whitelisted transactions prior to offline operations:
    - `transaction_id`
    - `payer_id`
    - `payer_ephemeral_pub`
    - `allowed_amount`
    - `expiration_timestamp`
    - `nonce`
- Offline terminals can cross-check transaction details against this whitelist if needed

## 5. Data Structures

Transaction Token (JWS, ED25519, or EC2559-signed structure

```json
{
		"transaction_id": "uuid",
		"payer_id": "string",
		"merchant_id": "string",
		"amount": "number",
		"timestamp": "ISO8601",
		"nonce": "hexstring"
}
```

Signed via either:

- JWS with RSA PKCS$1 v1.5 (RFC 8017),
- ED25519 (RFC 8032), or
- EC25519 ephemeral key derived signatures (X25519)

**Transaction Log Entry (Offline Terminal)**

```json
{
		"transaction_id": "uuid",
		"payer_ephemeral_pub": "base64",
		"HMAC": "hexstring",
		"amount": "number",
		"merchant_id": "string",
		"timestamp": "ISO8601",
		"nonce": "hexstring",
		"result": "accepted | denied"
}
```

**Whitelist entry (Settlemnet server)**

```json
{
		"transaction_id": "uuid",
		"payer_id": "string",
		"payer_ephemeral_pub": "base64",
		"allowed_amount": "number",
		"expiration_timestamp": "ISO8601",
		"nonce": "hexstring"
}
```

## 6. Security considerations

- Replay protection: nonce and timestamp checked locally.
- Tamper resistance: private keys stored in secure element or HSM.
- Offline integrity: transaction logs signed and hash chained (optional).
- key derivation: HKDF (RFC 5869) for deriving session keys
- Optional lightweight signatures: Ed25519 and EC25519 (X25519-derived) provide faster, lightweight signing suitable for resource-constrained devices.
- Ephemeral key agreement: optional X25519 Diffle-Hellman for mutual offline authentication or pre-transaction negotiation.
- Forward secrecy: no storage of derived session keys; keys derived at runtime via ephemeral exchanges.

## 7. Future extesions

- Support for multi-currency
- Biometric authentication integration
- Offline mutual authentication via ephemeral keys (EC25519/X25519)
- Post-quantum signature scheme evaluation

## References

- RFC 8017: PKCS #1 RSA Crytography Specifications v2.2
- RFC 5869: HMAC-based Extract-and-Expand Key Derivation Function (HKDF)
- RFC 7515: JSON Web Signature (JWS)
- RFC 8032: Edwards-Curve Digital Signature Algorithm (EdDSA) for ED25519
- RFC 7748: Elliptic Curves for Security (X25519)