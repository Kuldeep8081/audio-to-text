# 🎙️ Voice-to-Text Desktop App

A blazing fast, cross-platform desktop application that converts speech to text in real-time. Built with **Tauri (Rust)** for native performance and **Deepgram AI** for industry-leading transcription accuracy.

![Tauri](https://img.shields.io/badge/Tauri-v2-orange) ![React](https://img.shields.io/badge/React-TypeScript-blue) ![Deepgram](https://img.shields.io/badge/AI-Deepgram%20Nova--2-purple)

## ✨ Features

* **Real-time Transcription:** Uses Deepgram's `Nova-2` model for instant, accurate speech-to-text.
* **Native Performance:** Extremely lightweight compared to Electron apps (thanks to Rust).
* **"Type for Me" Mode:** Simulates keyboard strokes to insert transcribed text directly into other applications (Word, Notepad, VS Code, etc.).
* **Clipboard Integration:** One-click copy functionality.
* **Visual Feedback:** Live transcript display with clean UI.

---

## 🛠️ Prerequisites

Before running the app, ensure you have the following installed:

1.  **Node.js** (v24.11.1 or newer)
2.  **Rust & Cargo** (Required for the backend)
    * *Windows:* Install [Rustup](https://rustup.rs/) and [C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/).
    * *macOS:* Run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
3.  **Deepgram API Key** (Get a free key from [console.deepgram.com](https://console.deepgram.com/))

---

## 🚀 Installation & Setup

1.  **Clone the repository**
    ```bash
    git clone [https://github.com/your-username/voice-to-text-app.git](https://github.com/your-username/voice-to-text-app.git)
    cd voice-to-text-app
    ```

2.  **Install Frontend Dependencies**
    ```bash
    npm install
    ```

3.  **Configure Environment Variables**
    Create a `.env` file in the root directory:
    ```bash
    touch .env  # or create manually on Windows
    ```
    Add your API Key inside `.env`:
    ```env
    VITE_DEEPGRAM_API_KEY=your_deepgram_api_key_here
    ```

4.  **Install Backend Dependencies**
    Move to the Tauri folder and ensure the keyboard simulation crate is added:
    ```bash
    cd src-tauri
    cargo add enigo
    cd ..
    ```

---

## 🏃‍♂️ Running the App

Start the development server. This will compile the Rust backend and launch the native window.

```bash
npm run tauri dev
