# Actix-Web HTMX Tailwind Vite Micro-SaaS Boilerplate

A micro-SaaS boilerplate built with modern, powerful tools for rapid development and seamless scalability. This project leverages:  
- **Bun**: A fast, all-in-one JavaScript runtime that combines package management, bundling, and server-side rendering with unparalleled speed and simplicity.  
- **Vite**: A lightning-fast development server and build tool for a streamlined frontend workflow, enabling instant hot module replacement and optimized production builds.  
- **Tailwind CSS**: A utility-first CSS framework that empowers you to design responsive, modern user interfaces with ease and efficiency.  
- **HTMX**: A hypermedia extension library that enhances user interactions without the overhead of heavy JavaScript frameworks, enabling server-driven UI updates.  
- **Actix Web**: A powerful and performant Rust-based web framework that delivers high-speed, reliable backends for modern web applications.  

This boilerplate offers a robust foundation for creating and scaling micro-SaaS applications, ensuring a cohesive development experience from front to back.  

**Key Benefits**  
- **Efficiency**: Develop faster with Bun and Vite, which simplify common tasks and speed up builds.  
- **Simplified Interactions**: HTMX allows for dynamic user interactions without relying on heavy JavaScript libraries.  
- **Performance**: Actix Web ensures backends are both fast and reliable, making it ideal for scalable applications.  

## Screenshots

![Register](https://raw.github.com/johndavedecano/actix-web-htmx-tailwind/main/register.png)
![Login](https://raw.github.com/johndavedecano/actix-web-htmx-tailwind/main/login.png)
![Forgot](https://raw.github.com/johndavedecano/actix-web-htmx-tailwind/main/forgot.png)
![Reset](https://raw.github.com/johndavedecano/actix-web-htmx-tailwind/main/reset.png)
![Dashboard](https://raw.github.com/johndavedecano/actix-web-htmx-tailwind/main/dashboard.png)

## Prerequisites

To work with this project, ensure you have the following tools installed on your system:

- [Bun](https://bun.sh): Version 1.1.7 or later.
- [Rust](https://www.rust-lang.org/tools/install): Ensure Rust is installed using `rustup`.
- Cargo utility for managing Rust packages.

## Installation

### JavaScript Dependencies

To install the JavaScript dependencies using Bun, execute the following command in the terminal:

```bash
bun install
```

### Rust Dependencies

Install the necessary Cargo packages as follows:

```bash
cargo install
cargo install cargo-watch --locked
cargo install cargo-run-script
```

The above commands will set up the Rust environment and install essential tools for development, like `cargo-watch` for monitoring changes and `cargo-run-script` for running custom scripts.

## Running the Project

### Using Bun

To start the project using Bun's development server, run:

```bash
bun run dev
```

This command will start the Bun development server, enabling you to work on your JavaScript code with features like live reloading.

### Using Rust

Start the Rust application with:

```bash
cargo run-script start
```

This command runs the Rust server, allowing you to work on the backend functionality. Using `cargo-watch`, you can configure automatic rebuilding and re-running on changes for efficient development.

## Additional Notes

This project leverages the power of both Bun for JavaScript/TypeScript runtime capabilities and Rust for efficient, reliable server-side logic. Combining these technologies can significantly improve both performance and developer experience.

For further details on how to extend or modify this setup, please refer to the official documentation for [Bun](https://bun.sh/docs) and [Rust](https://doc.rust-lang.org/book/).

---

This README provides clarity on project setup and usage, offering instructions on how users can install dependencies, run the project, and understand the technology stack utilized. Adjust the content to suit any additional specifics or configurations unique to your project.

## Project Structure
```bash
src/
├── main.rs         # Entry point of the application
├── routes/         # Directory for route handlers
│   ├── mod.rs      # Module file for routes
│   ├── user.rs     # User-related routes
│   └── auth.rs     # Authentication-related routes
├── services/       # Business logic
│   ├── mod.rs      # Module file for services
│   └── user_service.rs
├── models/         # Data structures and types
│   ├── mod.rs      # Module file for models
│   └── user.rs
├── utils/          # Utility functions
│   ├── mod.rs      # Module file for utils
│   └── validator.rs
└── lib.rs          # (Optional) Shared functionality
```