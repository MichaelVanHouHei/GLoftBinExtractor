# 🎮 Gameloft Binary File Extractor 🗂️

Welcome to the **Heroes of Order & Chaos** `.bin` file extractor! 🚀 This tool splits your game `.bin` files into separate files, saving them in a folder named after your input file. Perfect for exploring game data! 🧙‍♂️

## ✨ What It Does
- 📂 Creates a folder based on your `.bin` file name
- 🔍 Checks if the file is a valid `.bin` with the right marker
- ✂️ Splits the file into chunks
- 💾 Saves each chunk as a separate file with its original name

## 🛠️ How to Use
1. **Install Rust** 🦀
   - Get Rust at [rustup.rs](https://rustup.rs/) 🌐
2. **Set Up the Project** 📦
   - Clone or download this project
   - Open a terminal and go to the project folder:
     ```bash
     cd path/to/project
     ```
3. **Run with Cargo** 🚗
   - Use this command to extract your Heroes of Order & Chaos `.bin` file:
     ```bash
     cargo run -- path/to/your/file.bin
     ```
     Example:
     ```bash
     cargo run -- data/hoc.bin
     ```
4. **Check Output** 🎉
   - A folder (e.g., `hoc`) will appear with your extracted files! 📁
   - Look at the terminal to see each file's name and size 📜

## ⚠️ Notes
- Make sure your file ends with `.bin`! ❌
- The file must start with the right Gameloft marker 🏷️
- Output files go into a folder named after your input file (without `.bin`) 🗃️

## 🎈 Have Fun!
Dive into your Heroes of Order & Chaos files and enjoy exploring! 🧙‍♀️✨