# Threads example Project

Welcome to the Threads project, a Rust application designed to manage and handle numerous concurrent threads in an efficient manner. This project is particularly useful for developers looking to experiment with concurrency in Rust.

## 🚨 Warning

This application uses a large number of threads which may lead to high resource consumption. It is also designed to panic and crash under certain circumstances as a means to handle errors and unexpected behavior. Please use this software responsibly and consider the limits of your system.

## 📝 Configuration

To customize the behavior of Threads, you can adjust the width and height constants in the main configuration file. These determine the number of threads the application will attempt to handle simultaneously.

```rust
// Configure the dimensions to control the number of threads
const WIDTH: usize = 60;   // Default number of threads along the width
const HEIGHT: usize = 60;  // Default number of threads along the height
```

### 📊 Adjusting Thread Count

The default settings (WIDTH = 60, HEIGHT = 60) result in 3600 threads. To adjust the number of threads, change these constants in your configuration. Keep in mind that increasing these numbers will increase the total number of threads, which may affect performance and stability depending on your system's capabilities.

## 🛠 Installation

To install and run Threads, follow these steps:

1. Clone this repository:
   ```bash
   https://github.com/Jl115/threads-rust.git
   ```
2. Navigate to the project directory:
   ```bash
   cd threads
   ```
3. Build the project using Cargo (the Rust package manager):
   ```bash
   cargo build
      ```
4. Run the application:
   ```bash
   cargo run 
   ```

## 🧪 Testing

To ensure that Threads performs as expected, you can run the included tests with the following command:

```bash
cargo test
```

## 🤝 Contributing

Contributions to the Threads project are welcome. Please consider the following steps if you are interested in contributing:

1. Fork the repository.
2. Create a new branch for your feature (`git checkout -b feature/YourFeatureName`).
3. Make your changes and commit them (`git commit -am 'Add some feature'`).
4. Push to the branch (`git push origin feature/YourFeatureName`).
5. Open a pull request.

## 📜 License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE) file for details.
