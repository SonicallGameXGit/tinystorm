# 🌩️ TinyStorm

**TinyStorm** is a simple yet powerful framework designed primarily for building and testing low-level games. Whether you're prototyping or experimenting, TinyStorm provides the tools you need to bring your ideas to life. 🚀

---

## ✨ Features

- 🛠️ Lightweight and easy to use
- 🎮 Perfect for low-level game testing
- ⚡ Fast and efficient for prototyping

---

## 📦 Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
tinystorm = "0.0.1"
```

---

## 🚀 Getting Started

Here's a quick example to get you started:

```rust
use tinystorm::window::{WindowBuilder};
let mut window = WindowBuilder::default()
    .with_size(800, 600)
    .with_title("My Window")
    .with_vsync(false)
    .with_max_fps(144 * 5)
    .with_msaa(4)
    .build();

while window.is_running() {
    window.poll_events();
    // Render your scene here
    window.swap_buffers();
}
```

---

## 📚 Documentation

Check out the full documentation [here](https://docs.rs/tinystorm).

---

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to improve TinyStorm.

---

## 📜 License

This project is licensed under the [MIT License](LICENSE).

---

Happy coding! 🎉  