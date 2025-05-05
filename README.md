# Simple Health Tracker Smart Contract

## 📘 Table of Contents
- [Project Title](#project-title)
- [Project Description](#project-description)
- [Project Vision](#project-vision)
- [Key Features](#key-features)
- [Contract Details](#contract-details)

---

## 📌 Project Title
**68. Simple Health Tracker**

---

## 📄 Project Description
The **Simple Health Tracker** is a lightweight smart contract that enables users to record, retrieve, and delete basic health information on the blockchain. It allows individuals to maintain private and immutable logs of their health parameters, such as age, height, weight, and personal notes.

---

## 🚀 Project Vision
To empower users with control over their health data through secure and decentralized recordkeeping. This tracker aims to promote personal health awareness while maintaining user privacy and ownership.

---

## ✨ Key Features
- Add or update your health profile on-chain
- Retrieve your health records at any time
- Delete your data for complete control and privacy

---

## 🔧 Contract Details

Contract Address: CCGZ6JWQIHSK5HNXQZZCRQOQ2V7XGPDPNCBPWPFZ3RAPDI7LTRXNHPAA

### `upsert_health(env, user, age, height_cm, weight_kg, notes)`
Allows a user to create or update their health profile. Requires authentication.

### `get_health(env, user)`
Fetches the health data associated with the given address. Publicly viewable.

### `delete_health(env, user)`
Deletes the user’s health data from the contract storage. Requires authentication.

---

✅ Built with [Soroban SDK](https://soroban.stellar.org/docs)
