# Backend Notes

This backend is built using Rust and several powerful libraries to handle routing, serialization, and asynchronous programming. Below are the main tools and libraries used in this project:

## Tools and Libraries

1. **Axum**

   - **Purpose**: HTTP server framework for building robust and scalable web applications.
   - **Usage**: Used for routing and handling HTTP requests.

2. **Tower**

   - **Purpose**: Provides reusable components for building HTTP servers and clients.
   - **Usage**: Middleware and utilities for composing asynchronous services.

3. **Serde**

   - **Purpose**: Serialization and deserialization framework.
   - **Usage**: Used to convert data structures to and from JSON and other formats.

4. **Tokio**

   - **Purpose**: Asynchronous runtime for the Rust programming language.
   - **Usage**: Handles asynchronous operations, such as I/O and networking.

5. **dotenv**

   - **Purpose**: Loads environment variables from a `.env` file.
   - **Usage**: Used to manage configuration settings.

6. **tower-http**

   - **Purpose**: Provides HTTP-related middleware and utilities.
   - **Usage**: Used for CORS, logging, and other HTTP-related tasks.

7. **sqlx**

   - **Purpose**: Async, pure Rust SQL crate featuring compile-time checked queries.
   - **Usage**: Database connection and query execution.

8. **tower-cookies**
   - **Purpose**: Middleware for managing cookies.
   - **Usage**: Manages cookies for HTTP requests and responses.

## Project Structure
