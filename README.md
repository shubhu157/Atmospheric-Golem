# Atmospheric Golems 🗿⛈️

**A Dynamic NFT Standard for Rialo Network**

## 📖 Overview
Atmospheric Golems is an experimental NFT project built on **Rialo**. It utilizes Rialo's native HTTP triggers to fetch real-world weather data directly on-chain, eliminating the need for third-party oracles like Chainlink.

The appearance of each "Golem" NFT evolves in real-time based on the weather conditions of the owner's chosen city.

## 🛠️ Tech Stack
* **Network:** Rialo (Devnet)
* **Language:** Rust (RISC-V Smart Contracts)
* **Data Source:** OpenWeatherMap API (fetched via Rialo native webhooks)

## ⚡ How it Works
1.  **Minting:** User mints a Golem and assigns a `CityID` (e.g., London).
2.  **Trigger:** The contract executes a cron job every 4 hours.
3.  **Fetch:** `http_get()` request sent to weather API.
4.  **Mutate:**
    * If `temp < 0°C` → Update Metadata to `Ice_Skin`
    * If `weather == rain` → Update Metadata to `Moss_Overlay`
    * If `temp > 30°C` → Update Metadata to `Magma_Core`

## 🚀 Roadmap
* [x] Concept & Art Generation
* [ ] Contract Logic (In Progress)
* [ ] Devnet Deployment
* [ ] UI/Frontend

## 🎨 Art Preview
*(Art assets coming soon)*
