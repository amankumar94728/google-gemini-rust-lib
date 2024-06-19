# google_gemini
# Google Gemini API Rust Wrapper

This repository contains a Rust library that provides a wrapper for the Google Gemini API, packaged as an NPM module for easy integration into Node.js applications. This library simplifies interaction with the Google Gemini API, enabling developers to seamlessly integrate its functionality into their Rust-based projects and Node.js environments.

## Features

- **Comprehensive API Coverage**: Access various endpoints and functionalities of the Google Gemini API.
- **Easy Integration**: Provides a straightforward interface for integrating Google Gemini API with Rust and Node.js.
- **High Performance**: Leverages Rust's performance benefits while maintaining ease of use in Node.js.
- **Unit Tests**: Includes extensive test cases to ensure reliability and correctness of the library.

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (version 18 or higher)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (Rust's package manager)

### Installation

1. **Install the NPM Package**
   ```bash
   npm install google-gemini-rust


2. Use the Library in Your Node.js Project

  const { GeminiClient } = require('google-gemini-rust');

const client = new GeminiClient('your-api-key');
client.getSomething().then(response => {
  console.log(response);
}).catch(error => {
  console.error(error);
});


Development
   To contribute or modify this library, follow these steps: 
    1. Clone the Repository 
         git clone https://github.com/amankumar94728/google-gemini-rust
         cd google-gemini-rust

    2. Install Dependencies
        npm install
    3. Build the Project
        neon build
    4. Run Tests
        npm test


Contributing
Contributions are welcome! Please open an issue or submit a pull request for any bugs, features, or documentation improvements.

Acknowledgments
-Thanks to the Rust and Node.js communities for their fantastic tools and support.
-Special thanks to the contributors who help improve this project.
