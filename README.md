# Rust Password Manager

A password manager built with **Rust**, using **Axum** for the backend, **MongoDB** for storage, and **libsodium** for encryption. The frontend is built with **Svelte**.

---

## Tech Stack

### Backend

* Rust
* Axum
* MongoDB
* libsodium

### Frontend

* Svelte

---

## Overview

This project consists of:

* A Rust backend API built with Axum
* A MongoDB database for storing data
* Encryption handled with libsodium
* A Svelte frontend for the user interface

---

## Setup

### Backend

```bash
cd backend
cargo build
cargo run
```

### Frontend

```bash
cd frontend
npm install
npm run dev
```

---

## Notes

* The backend is responsible for handling API requests and interacting with MongoDB.
* Encryption is implemented using libsodium.
* The frontend communicates with the backend API.

---
