# flex
why not make it better 
# 🧵 TailorFlex

**TailorFlex** is a lightweight tailoring management web app built for Nigerian and African tailors.  
It helps fashion designers manage their orders, measurements, and due dates — *even when offline*.  

This MVP focuses on the **tailor’s side** only — simple, fast, and ready for local environments.

---

## 🚀 Features

✅ Manage tailoring **orders** (style, fabric, due date, status)  
✅ Add and update **measurements** for each order  
✅ **Offline-first** — works without internet using local storage  
✅ Multi-language support — **English**, **Pidgin**, **Yoruba**, **Hausa**  
✅ Built with **NestJS (backend)** + **Next.js (frontend)**  
✅ Ready for **PWA** support (installable like a mobile app)  
✅ Local database (SQLite) for easy setup  

---

## 🧩 Tech Stack

| Layer | Technology | Purpose |
|-------|-------------|----------|
| **Backend** | [NestJS](https://nestjs.com) | API and business logic |
| **Database** | SQLite + TypeORM | Simple, local data storage |
| **Frontend** | [Next.js 15](https://nextjs.org) | Modern React-based web UI |
| **Styling** | Tailwind CSS | Clean, responsive design |
| **Offline Storage** | IndexedDB / localForage | Offline data caching |
| **Localization** | react-i18next | Multi-language translations |

---

## 🗂️ Project Structure

TailorFlex/
├── tailorflex-backend/
│ ├── src/
│ │ ├── tailor/
│ │ │ ├── tailor.controller.ts
│ │ │ ├── tailor.service.ts
│ │ │ ├── entities/
│ │ │ │ ├── tailor.entity.ts
│ │ │ │ ├── order.entity.ts
│ │ │ │ └── measurement.entity.ts
│ │ │ └── dto/
│ │ │ ├── create-order.dto.ts
│ │ │ └── create-measurement.dto.ts
│ │ ├── app.module.ts
│ │ └── main.ts
│ └── package.json
│
└── tailorflex-frontend/
├── src/
│ ├── app/
│ │ ├── page.tsx
│ │ ├── orders/
│ │ └── measurements/
│ ├── components/
│ ├── lib/
│ └── styles/
└── package.json

yaml
Copy code

---

## ⚙️ Setup Instructions

### 1️⃣ Clone the Repository
```bash
git clone https://github.com/<your-username>/tailorflex.git
cd tailorflex
2️⃣ Backend Setup (NestJS)
bash
Copy code
cd tailorflex-backend
npm install
npm run start:dev
Environment file example (.env):

ini
Copy code
DATABASE_URL=sqlite://./data/tailorflex.db
PORT=5000
3️⃣ Frontend Setup (Next.js)
bash
Copy code
cd ../tailorflex-frontend
npm install
npm run dev
App runs at:
👉 http://localhost:3000

🧵 API Endpoints (MVP)
Method	Endpoint	Description
POST /orders	Create a new tailoring order	
GET /orders	Get all orders	
PATCH /orders/:id	Update order status	
POST /measurements	Add measurement for an order	

🌍 Languages Supported
Language	Code	Example
English	en	“Add Order”
Pidgin	pg	“Add Cloth Work”
Yoruba	yo	“Fi Aṣẹ Kun”
Hausa	ha	“Ƙara Oda”

You can easily add more translations inside frontend/src/lib/i18n.ts.

📦 Offline Support
Uses IndexedDB via localforage to store unsynced data.

When internet returns, frontend syncs with backend through /sync.

Perfect for tailors in areas with unstable internet.

🧠 Roadmap
 Tailor dashboard

 Orders & measurements module

 Language switcher

 Customer portal

 WhatsApp notifications

 Cloud sync (Firebase / Supabase)

 Mobile app (React Native)

🛠️ Developer Notes
Backend auto-migrates schema using TypeORM.

Frontend can be converted to PWA (next-pwa plugin).

Designed for both desktop and mobile browsers.

👩🏾‍🎨 Credits
Author: [Your Name / @codeflex / @skul_africa]
Idea: Solving real issues for Nigerian tailors who lose customer measurements during peak seasons.

🪄 License
MIT © 2025 skul-africa Team



---













ChatGPT can make mistakes. Check important info.
