# 🦀 Rust Terminal — A Lightweight Custom Shell in Rust

A fully functional mini **command-line shell** written in **Rust**, featuring support for:  
✅ Command execution  
✅ Directory navigation (`cd`)  
✅ Output redirection (`>`)  
✅ Piping between commands (`|`)  
✅ Built-in `exit` command to gracefully quit  

This project showcases how Rust’s **`std::process`**, **`std::io`**, and **system-level abstractions** can be combined to create a minimal yet powerful terminal emulator.

---

## 🚀 Features

### 🧭 1. Execute System Commands
Run any valid system command directly from the terminal.
```bash
> ls -l
> echo Hello, Rust!
```

### 📂 2. Change Directories
Implements a built-in `cd` command to change the current working directory.
```bash
> cd src
> cd ..
```

### 🔗 3. Command Piping (`|`)
Connect the output of one command to another — just like in real shells.
```bash
> ls | grep main
> cat Cargo.toml | grep edition
```

### 💾 4. Output Redirection (`>`)
Redirect command output to a file.
```bash
> echo Hello > output.txt
> ls > files.txt
```

### 🧱 5. Exit Command
Exit cleanly from the terminal loop.
```bash
> exit
```

---

## 🧰 Tech Stack

- **Language:** Rust 🦀  
- **Core Libraries:**  
  -
[`std::process::Command`](https://doc.rust-lang.org/std/process/struct.Command.html) – to spawn and manage subprocesses  
  - [`std::io`](https://doc.rust-lang.org/std/io/index.html) – for input/output handling  
  - [`std::env`](https://doc.rust-lang.org/std/env/index.html) – for directory management  

---

## ⚙️ Installation & Run

### 1. Clone this repository
```bash
git clone https://github.com/your-username/rust-terminal.git
cd rust-terminal
```

### 2. Build and Run
```bash
cargo run
```

You’ll see your custom prompt:
```bash
> 
```
Start typing commands!

---

## 🧩 Example Session

```
> echo "Rust is awesome!" > message.txt
> cat message.txt
Rust is awesome!
> ls | grep Cargo
Cargo.toml
> cd src
> ls
main.rs
> exit
Goodbye 👋
```

---

## 💡 How It Works

1. The program reads user input in a loop.  
2. It splits the input into **command** and **arguments**.  
3. Special cases (`cd`, `exit`, `|`, `>`) are handled manually.  
4. For all other commands, it uses `Command::new()` to spawn subprocesses.  
5. The terminal waits for processes to finish using `.wait()`.

---

## 🧠 Learning Highlights
- How to **spawn and pipe processes** in Rust.  
- How to **manipulate I/O streams** using `Stdio::piped()` and `Stdio::from()`.  
- How to **implement built-in commands** like `cd` that modify the current process state.  
- Building a **REPL-like loop** with error handling and clean UX.

---

## 🛠️ Future Enhancements

- Add support for input redirection (`<`)  
- Implement command history  
- Handle background processes (`&`)  
- Colorful prompt and formatted output  

---

## 📜 License

This project is licensed under the **MIT License** – feel free to use, modify, and distribute!

---

## ❤️ Acknowledgments

Inspired by the UNIX philosophy:  
> “Write programs that do one thing and do it well.”
