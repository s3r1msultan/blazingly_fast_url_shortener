# 🚀 rxst.me - A Blazingly Fast URL Shortener

Welcome to **rxst.me**, a high-performance, scalable, and secure URL shortener built using **Rust, Actix-Web, and SQLx**. This project was developed as part of the **BLAZINGLY FAST Hackathon** to provide an ultra-fast and reliable solution for shortening and managing URLs.

## ✨ Features

✅ **Blazing Fast Performance** – Built with **Rust** and **Actix-Web**, leveraging async operations for speed.  
✅ **Short URL Generation** – Generates short, **Base62-encoded** or **custom alias** URLs.  
✅ **Custom Alias Handling** – Users can provide their own alias; if taken, **alternative suggestions** are provided.  
✅ **Automatic Expiration (TTL)** – URLs **expire after 6 months** and are **automatically deleted**.  
✅ **Leet (1337) Transformations** – If an alias is taken, the system suggests leet-style variations.  
✅ **Redirection System** – Redirects users to the original URL via **`/{short_url}`**.  
✅ **URL Info Retrieval** – Retrieve URL details (original URL, expiration date) via **`/u/{short_url}`**.  
✅ **Validation** – Ensures **valid URLs** and prevents **invalid alias names**.  
✅ **Security Headers** – Enforced **CORS, XSS, and Content Security Policies**.  
✅ **Zero Downtime Cleanup** – A background task **automatically deletes expired URLs**.

---

## ⚡ Tech Stack

- **Rust** – Safe, fast, and memory-efficient.
- **Actix-Web** – High-performance web framework.
- **SQLx + SQLite** – Async database interaction.
- **Tokio** – Async runtime for Rust.
- **Regex** – For URL validation.
- **dotenv** – Manage environment variables.

---

## 📂 Project Structure

```
rxst.me/
│── src/
│   │── main.rs        # Main server entry point
│   │── config.rs      # Initializes logging & environment variables
│   │── database.rs    # Handles database interactions & TTL cleanup
│   │── handlers.rs    # HTTP request handlers (shorten, redirect, info)
│   │── hash.rs        # Hashing & encoding for short URLs
│   │── models.rs      # Database models & request structures
│   │── routes.rs      # Actix-Web route configuration
│   │── validation.rs  # URL validation logic
│── migrations/        # SQL migration files
│── static/index.html  # Frontend UI for shortening URLs
│── .env               # Environment configuration
│── Cargo.toml         # Rust dependencies
```

---

## 🚀 Quick Start

### 📌 1️⃣ **Clone the Repository**
```sh
git clone https://github.com/s3r1msultan/blazingly_fast_url_shortener.git
cd blazingly_fast_url_shortener
```

### 📌 2️⃣ **Setup the Environment**
Create a **`.env`** file with:
```sh
DATABASE_URL=sqlite://url_shortener.db
```

### 📌 3️⃣ **Run Database Migrations**
```sh
cargo install sqlx-cli
cargo sqlx migrate run
```

### 📌 4️⃣ **Start the Server**
```sh
cargo run
```

### 📌 5️⃣ **Open in Browser**
Visit **[`http://0.0.0.0:8080`](http://0.0.0.0:8080)** and start shortening URLs.

---

## 🔗 API Endpoints

### 🔹 **Shorten a URL**
**`POST /shorten`**
```json
{
    "original_url": "https://example.com",
    "custom_alias": "myalias"
}
```
**Response:**
```json
{
    "short_url": "myalias"
}
```

### 🔹 **Redirect to Original URL**
**`GET /{short_url}`**
- Redirects users to the original website.

### 🔹 **Get URL Info**
**`GET /u/{short_url}`**
**Response:**
```json
{
    "short_url": "myalias",
    "expiration_date": "2025-08-03 13:00:00"
}
```

---

## 🎯 How It Works

### 🔥 **1️⃣ Hash-Based Short URL Generation**
- Uses **FNV-1a Hashing + Base62 Encoding** to generate **deterministic short URLs**.

### 🛠 **2️⃣ Alternative Alias Suggestions**
- If an alias is **already taken**, the system **suggests alternative aliases** using **leet-style replacements**:
    - `"google" → "9oogle", "90ogle", "900gle"`

### 🕒 **3️⃣ Automatic Expiration & Cleanup**
- URLs **expire in 6 months**.
- A **background task deletes expired URLs every hour**.

### 🔎 **4️⃣ Fetch URL Details**
- Users can **retrieve info about a short URL** via **`/u/{short_url}`**.

---

## 🛠 Contributing

Want to improve **rxst.me**? Contributions are welcome!  
To contribute:
1. **Fork the repo**.
2. **Create a feature branch** (`git checkout -b feature-name`).
3. **Commit your changes** (`git commit -m "Added feature XYZ"`).
4. **Push to GitHub** (`git push origin feature-name`).
5. **Submit a Pull Request (PR)**.

---

## 📜 License
This project is licensed under the **MIT License**.

---

## 👥 Team & Credits
Built by **s3r1msultan** for the **BLAZINGLY FAST Hackathon**.

🚀 **Let's make URL shortening faster than ever!**
