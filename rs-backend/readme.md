# About
This project is a comprehensive backend solution written in Rust, featuring extensive API endpoint unit testing.

# Setup Instructions

## Prerequisites
- **Cargo**: Ensure Cargo is installed. You can find installation instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).
- **PostgreSQL**: A running instance of PostgreSQL is required.

## Installation Steps
1. Install the SQLx CLI:
   ```bash
   cargo install sqlx-cli
   ```
2. Create the database:
   ```bash
   cargo sqlx database create
   ```
3. Run database migrations:
   ```bash
   cargo sqlx migrate run
   ```
4. Start the application:
   ```bash
   cargo run
   ```

# Testing
1. Run the tests one at a time.
   ```bash
   cargo nextest run -j 1
   ```