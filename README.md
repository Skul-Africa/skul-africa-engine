Skul Africa Timetabling & Resource Optimization Engine
🚀 The Future of School Management Starts Here

Every school in Africa faces the same nightmare: building a functioning timetable. It's messy, stressful, slow, and usually done manually with trial, error, tears, and Excel sheets. Teachers complain, classes clash, rooms get double-booked, and the entire academic flow becomes chaotic.

The problem is not because schools are disorganized — it’s because there has never been a true intelligent engine designed for African school structures. Until now.

This project introduces the Skul Africa Timetabling & Resource Optimization Engine — a standalone, high‑performance scheduling and optimization engine that solves one of the hardest problems in school administration.

This engine runs independently of any backend (including Skul Africa’s NestJS app). It is a plug‑and‑play microservice that any app can integrate with via REST or gRPC.

💢 The Frustration (The Story)

Every term, schools spend weeks trying to build a timetable. The workflow usually goes like this:

The school writes subjects on a board

They try to place teachers in time blocks

Conflicts appear immediately

Rooms clash

Teachers exceed teaching limits

Students end up with impossible schedules

Sports, clubs, and labs get forgotten

A single teacher becoming absent breaks the entire schedule

And when they finally get something “usable”…

Exams come → everything breaks again

New class added → everything breaks again

Teacher resigns → everything breaks again

A parent complains → everything breaks again

Schools deserve better.

🎯 The Solution

A Smart Timetabling & Resource Optimization Engine that:

Understands teachers, classes, rooms, subjects, extracurriculars

Automatically generates optimal timetables

Handles clashes, fairness, and constraints

Rebuilds the schedule instantly when something changes

Provides clear explanations for decisions

Works as a separate engine that any system can call

This engine is designed to be:

Fast

Robust

Scalable

Difficult for competitors to replicate

A game‑changer for African education

🧠 Our Goal

To build the most advanced, simple‑to‑use, AI‑powered timetabling system on the continent.

By the end of this project, we will have:

A production-grade optimization engine

A pluggable microservice deployable anywhere

A clear API that any backend can integrate with

A system capable of solving real school constraints

A foundation for future AI‑driven school management

🏗 Technology Stack
👨‍💻 Engine Language: Go (Golang)

Chosen for:

Ease of learning

Fast development

Great concurrency

Stability for microservices

Clean integration with gRPC and REST

🧠 Optimization Brain: Google OR-Tools (C++)

Handles:

Complex constraints

Optimization

Timetabling logic

Room allocation

Class balancing

🧩 Communication Layer

gRPC (protobuf) — primary engine API

REST — optional layer for easy integration

🗃 Storage

PostgreSQL (optional for schools that want history)

Redis (for caching solves & sessions)

🚀 Deployment

Docker

Kubernetes (optional)

📡 Observability

Prometheus

Grafana



# Skul Africa Rust Timetabling & Resource Optimization Engine

## Project Structure (Rust Base)

This is the base folder structure for the engine using Rust. It is designed for modularity, fast development, and integration with OR-Tools for optimization.

```
skul-engine-rust/
│
├── Cargo.toml               # Rust project manifest
├── src/
│   ├── main.rs              # Entry point for the engine
│   ├── lib.rs               # Core library for the engine logic
│   ├── api/
│   │   ├── mod.rs           # API module
│   │   ├── server.rs        # gRPC / REST server setup
│   │   └── handlers.rs      # Request handlers
│   ├── solver/
│   │   ├── mod.rs           # Solver module
│   │   ├── ortools.rs       # OR-Tools solver integration
│   │   └── scheduler.rs     # Timetable & optimization logic
│   ├── models/
│   │   ├── mod.rs
│   │   ├── teacher.rs
│   │   ├── class_group.rs
│   │   ├── room.rs
│   │   ├── constraint.rs
│   │   └── schedule.rs
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── validation.rs    # Input validation helpers
│   │   └── mapper.rs        # Transform data to solver format
│   └── config.rs            # Configuration & environment variables
│
├── proto/
│   ├── timetabling.proto     # Protobuf definitions for gRPC
│   └── mod.rs
│
├── examples/
│   └── sample_request.json   # Example request payloads
│
├── Dockerfile               # Docker setup
└── README.md                # Documentation
```

## Features Planned

* Standalone microservice engine
* gRPC & REST API
* Timetable generation with constraints
* Teacher availability, room allocation, class scheduling
* Partial rescheduling (dynamic updates)
* Integration with OR-Tools for optimization
* Future ML integration for predictions

🔌 Integration Example (NestJS → Engine)
POST http://engine.skul.africa/solve
{
  "teachers": [...],
  "rooms": [...],
  "classes": [...],
  "constraints": [...]
}

Response:

{
  "schedule": { ... },
  "warnings": []
}
🧱 Core Concepts
Teachers

Subjects they teach

Availability windows

Max hours per week

Rooms

Capacity

Special tags (lab, ICT, hall)

Class Groups

Required subjects

Student count

Constraints

Hard constraints (must be obeyed)

Soft constraints (preferred)

Objective Functions

Examples:

Minimize teacher gaps

Minimize class conflicts

Balance workload

Maximize room usage

🚀 Vision

We want Skul Africa to become the platform of choice for schools, and this engine is the foundation of a system that will:

Transform school operations

Save thousands of hours yearly

Eliminate timetable disasters

Predict scheduling problems before they happen

Become a core engine other platforms rely on

This is the beginning of a new era of African school technology.
