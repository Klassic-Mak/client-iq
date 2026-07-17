# Device Intelligence SDK

> An open, extensible device intelligence SDK built on top of the FingerprintJS open-source engine, designed to provide real-time device identification, telemetry, and security intelligence.

---

# Overview

Device Intelligence SDK is a browser-based device identification library that extends the open-source FingerprintJS engine with additional capabilities for:

-   Real-time device telemetry
-   Device intelligence processing
-   Security and fraud detection
-   Live dashboard integration
-   Risk scoring
-   Custom analytics
-   Event streaming

The project uses the FingerprintJS fingerprinting engine as its foundation while introducing a new intelligence layer that transforms raw fingerprints into actionable device information.

---

# Project Goals

The SDK is designed to become a complete device intelligence platform rather than just a fingerprint generator.

Current goals include:

-   Generate stable visitor IDs
-   Stream visitor data to a central dashboard
-   Build a complete device profile
-   Detect suspicious behavior
-   Support real-time monitoring
-   Support browser and mobile devices
-   Provide a developer-friendly SDK

---

# High-Level Architecture

```text
Browser
    │
    ▼
FingerprintJS Engine
    │
    ▼
Device Intelligence Layer
    │
    ▼
Streaming Layer
    │
    ▼
Backend API
    │
    ▼
Realtime Dashboard
```

---

# Repository Structure

```text
src/
│
├── index.ts
├── load.ts
├── agent.ts
├── source.ts
├── hash.ts
├── components/
│
├── intelligence/
│
├── api/
│
├── realtime/
│
└── types.ts
```

---

# Core Engine (FingerprintJS)

The following modules remain the foundation of the SDK:

## `index.ts`

Public SDK entry point.

Responsible for exporting the public API.

Example:

```ts
const fp = await load()
```

---

## `load.ts`

Creates and initializes the fingerprinting engine.

Responsibilities:

-   initialize sources
-   configure options
-   create the Agent

---

## `agent.ts`

The heart of the SDK.

Responsibilities:

-   collect components
-   generate Visitor ID
-   execute the intelligence pipeline
-   return the final result

Every fingerprint generated flows through this file.

---

## `components/`

Contains individual fingerprint collectors.

Examples include:

-   Canvas
-   WebGL
-   Audio
-   Fonts
-   Screen Resolution
-   Timezone
-   Platform
-   Languages

Each component contributes entropy used to generate the Visitor ID.

---

# Intelligence Layer

The intelligence layer transforms a fingerprint into useful information.

---

## `intelligence/enrich.ts`

Adds contextual information to the fingerprint.

Examples:

-   Browser language
-   Screen size
-   Current page
-   Timezone
-   Timestamp
-   Platform

Output:

```text
Fingerprint

+

Browser Data

+

Page Data

+

Device Data

=

Device Profile
```

---

## `intelligence/normalize.ts`

Ensures every payload follows a consistent structure regardless of browser differences.

Responsibilities:

-   normalize browser names
-   normalize screen data
-   normalize timestamps
-   standardize output

---

## `intelligence/risk.ts`

Calculates a risk score for every device.

Future signals may include:

-   VPN detection
-   Automation detection
-   Headless browser detection
-   High request frequency
-   Suspicious fingerprints
-   Device reputation

Example:

```
Risk Score: 82

Level: HIGH
```

---

# API Layer

## `api/client.ts`

Responsible for communicating with the backend.

Responsibilities:

-   send device payloads
-   retry failed requests
-   authentication
-   batching (future)
-   compression (future)

The rest of the SDK should not call `fetch()` directly.

---

# Realtime Layer

## `realtime/`

Responsible for real-time communication.

Future responsibilities:

-   WebSocket
-   Server Sent Events (SSE)
-   Live visitor notifications
-   Live dashboard updates

Goal:

```
New Visitor

↓

Dashboard updates instantly

↓

No polling required
```

---

# Data Flow

The complete SDK pipeline is:

```
Browser

↓

Fingerprint Components

↓

Agent

↓

Enrichment

↓

Normalization

↓

Risk Analysis

↓

API Client

↓

Backend

↓

Realtime Dashboard
```

---

# Current Development Phase

The project is currently focused on the SDK core.

Completed:

-   Fingerprint engine integration
-   Project restructuring
-   Intelligence layer architecture

In Progress:

-   Data enrichment
-   API streaming
-   SDK packaging

Planned:

-   Backend service
-   WebSocket gateway
-   Dashboard
-   Rules engine
-   Risk engine
-   Device reputation
-   Mobile SDK
-   Developer Portal

---

# Future Features

## Device Intelligence

-   Device reputation
-   Device history
-   Browser fingerprint timeline
-   Device trust score

---

## Security

-   Fraud detection
-   Automation detection
-   Bot detection
-   Headless browser detection
-   Rate anomaly detection

---

## Dashboard

-   Live visitor feed
-   Visitor history
-   Device profile
-   Activity timeline
-   Session replay integration
-   Blocklist management

---

## SDK Features

-   Browser SDK
-   React SDK
-   Vue SDK
-   Angular SDK
-   Next.js SDK
-   Node SDK
-   Flutter SDK
-   React Native SDK

---

# Mobile Support

The long-term vision includes mobile device intelligence.

Future SDKs:

-   Flutter
-   React Native
-   Native Android
-   Native iOS

Each SDK will generate a unified Device Identity compatible with the backend.

---

# Design Philosophy

The project is built around a layered architecture.

```
Fingerprint Engine

↓

Intelligence

↓

Communication

↓

Visualization
```

Each layer has a single responsibility, making the project easier to maintain, extend, and test.

---

# Vision

The goal is to build an extensible, developer-friendly device intelligence platform capable of identifying devices, providing real-time insights, and powering fraud prevention and security systems while remaining modular and easy to integrate.
