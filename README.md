# 專案名稱：Mifare 小額感應卡儲值系統 (Mifare Top-up System)

## 1. 專案概述
本系統旨在建構一個穩定且高效的感應卡儲值服務。當使用者感應 Mifare 卡片時，系統自動辨識卡片 ID，並透過後端處理交易邏輯，記錄儲值金額，最後更新卡片內的餘額或與資料庫同步。

## 2. 核心架構設計
系統採用前後端分離架構，並包含一個硬體互動層。

*   **前端 (Front-end):** 基於 **Vue.js**，負責提供操作介面。
*   **後端 (Back-end):** 使用 **Rust** 配備 **Axum** 框架，負責處理業務邏輯、API 路由、資料庫操作與多執行緒同步。
*   **硬體介面層 (Hardware/Middleware):** 透過序列埠 (Serial Port/USB) 或專用協議與 RFID 讀取器通訊，將硬體訊號轉換為後端可處理的數據。

## 3. 技術棧 (Tech Stack)
### 後端 (Backend)
- **語言:** Rust
- **網頁框架:** Axum (高效能、類型安全的 Web 框架)
- **非同步運行時:** Tokio
- **資料庫驅動:** SQLx (支持 PostgreSQL 或 SQLite)
- **配置管理:** Config-rs 或 Env_vars

### 前端 (Frontend)
- **框架:** Vue.js (建議 Vue 3 + Vite)
- **狀態管理:** Pinia
- **樣式:** Tailwind CSS (快速構建 UI)
- **通訊:** Axios 或 Fetch API

### 資料庫 (Database)
- **選擇:** PostgreSQL (生產環境) 或 SQLite (開發/輕量需求)

## 4. 功能模組與規格

### 4.1 硬體互動功能 (Hardware Integration)
- **卡片辨識:** 當感應卡片時，獲取獨特的 UID (Unique Identifier)。
- **自動感應觸發:** 後端監聽或接收來自硬體層的訊息，自動觸發查詢流程。
- **狀態反饋:** 根據後端處理結果，回傳給前端（如：「認證成功」、「儲值成功」）。

### 4.2 核心交易邏輯 (Transaction Logic)
- **儲值流程:** 
    1. 讀取卡片 ID。
    2. 驗證卡片有效性。
    3. 使用者選擇/輸入金額。
    4. 執行資料庫原子事務 (Atomic Transaction)：增加用戶帳戶餘額、記錄交易記錄。
    5. 寫入卡片（若需要直接寫入卡片數據）。
- **餘額查詢:** 通過卡片 ID 查詢當前帳戶餘值。

### 4.3 管理後台 (Admin Dashboard)
- **交易紀錄:** 列出所有歷史交易（時間、卡片 ID、金額、操作員）。
- **卡片管理:** 啟用/停用特定卡片。
- **統計報表:** 每日/每週儲值總額統計。

## 5. 資料庫設計 (Database Schema)

### 表：Cards (卡片資訊)
| 欄位 | 型別 | 說明 |
| :--- | :--- | :--- |
| `id` | UUID/Serial | 自動遞增 ID |
| `card_uid` | String | 物理卡片的唯一識別碼 |
| `balance` | Decimal | 當前餘額 |
| `status` | Enum | 啟用、停用、丟失 |
| `created_at` | Timestamp | 建立時間 |

### 表：Transactions (交易紀錄)
| 欄位 | 型別 | 說明 |
| :--- | :--- | :--- |
| `id` | UUID | 交易唯一識別碼 |
| `card_uid` | String | 對應的卡片 ID |
| `amount` | Decimal | 此次儲值金額 |
| `timestamp` | Timestamp | 交易發生的時間 |

## 6. API 接口設計 (API Endpoints)

| 方法 | 路徑 | 功能描述 | 說明 |
| :--- | :--- | :--- | :--- |
| `GET` | `/api/card/info` | 查詢卡片資訊 | 傳送 `card_uid`，回傳餘額與狀態 |
| `POST` | `/api/topup` | 執行儲值動作 | 傳送 `card_uid` 與 `amount`，執行原子事務 |
| `GET` | `/api/transactions` | 獲取交易歷史 | 管理員使用，可篩選時間範圍 |
| `GET` | `/api/status` | 系統健康檢查 | 檢查後端與資料庫連線狀態 |

## 7. 系統工作流 (Workflow)
1. **使用者動作:** 用戶將 Mifare 卡片放在讀取器上。
2. **硬體層:** 讀取器捕捉到 UID，發送給後端（或透過 Webhook 通知前端）。
3. **前端顯示:** Vue.js 偵測到新卡片，顯示「請選擇儲值金額」。
4. **使用者選擇:** 使用者點擊按鈕（如：$100）。
5. **後端處理:** Axum 接收請求，在資料庫中執行轉帳，並返回成功訊息。
6. **完成提示:** 前端顯示「儲值成功，剩餘 $[Amount]」。

## 8. 未來擴充建議
- **通知系統:** 儲值成功後發送簡訊或推播給用戶。
- **多點位管理:** 支持多台讀取器同時連線到同一個 Axum 後端。
- **線上儲值:** 整合線上支付網關，允許用戶透過手機 App 遠程儲值。