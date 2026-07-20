# Client IQ Backend

The robust Rust backend service for the **Client IQ** platform, built using Axum, MongoDB, and Serde.

---

## 🛠️ Tech Stack

*   **Language:** Rust
*   **Web Framework:** Axum (`tokio`, `axum`)
*   **Database:** MongoDB (`mongodb` crate with BSON support)
*   **Serialization:** Serde & Serde JSON

---

## 📂 Core Architecture & Models

### 1. Client Models (`Client`, `ClientRequest`, `ClientResponse`)
*   **`Client`**: Primary database entity featuring an optional MongoDB `ObjectId`, client name, device reference, dynamic metadata (`HashMap<String, serde_json::Value>`), and timestamp.
*   **`ClientRequest`**: Payload structure used for incoming client registration or creation requests.
*   **`ClientResponse`**: Standardized wrapper structure generated via `Client::to_response()`.

### 2. Device Models (`DeviceProfile`, `DeviceStatus`)
*   **`DeviceStatus`**: Enum representing device states (`active`, `offline`, `suspended`) with `snake_case` serialization.
*   **`DeviceProfile`**: Core profile entity tracking unique device identifiers, status, total event counts, last seen timestamps, and configuration maps.

### 3. Event Models (`EventData`, `EventType`)
*   **`EventType`**: Categorizes incoming events (`error_report`, `session_start`, `interaction_log`).
*   **`EventData`**: Represents logged events containing client references, device references, event types, event metadata dictionaries, and occurrence timestamps.

---

## 🚀 API Endpoints

The application exposes the following routes configured in `main.rs`:

| Method | Endpoint | Description |
| :--- | :--- | :--- |
| **GET** | `/ws` | Test database / root status endpoint (`test_db_handler`) |
| **ANY** | `/ws/client/{client_id}` | WebSocket connection handler for active clients |
| **POST** | `/api/v1/create-client` | Register a new client entity |
| **GET** | `/api/v1/clients` | Retrieve all registered clients |
| **GET** | `/api/v1/client/{id}` | Fetch a specific client by ID |
| **POST** | `/api/v1/devices` | Register or update a device profile |
| **GET** | `/api/v1/devices/{device_id}` | Retrieve device profile status and configuration |
| **POST** | `/api/v1/events` | Submit system telemetry and interaction events |

---

## ⚙️ Getting Started & Setup

1. **Clone the repository:**
   ```bash
   git clone [https://github.com/your-username/client-iq-backend.git](https://github.com/your-username/client-iq-backend.git)
   cd client-iq-backend