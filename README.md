# Logger

This Rust program records keyboard and mouse input events and saves them to a JSONL file with timestamps. It supports logging key presses, character input, mouse clicks, mouse movements, and mouse wheel events.

---

## Features

- Records global keyboard and mouse events
- Converts key presses to characters considering Shift and CapsLock
- Stores events in a JSONL file with timestamps
- Buffers events in memory to reduce disk I/O
- Automatic program termination after a configurable duration

---

## dependencies

**My Rust Developing environment**

- 1.89

**[dependencies]**

- chrono = { version = "0.4", features = ["clock", "serde"] }
- rdev = "0.5"
- serde = { version = "1.0", features = ["derive"] }
- serde_json = "1.0"

**Notification:**

- the chrono version of chrono package must the newer than 0.3

---

## Usage

1. Build the program using Rust:

```powershell
    cargo build --release
    cargo run

    #Type any thing or click the button you want to click, your incident will get logged
```

2. Check Logfile and filter the data you want

```powershell
    # If You only wanna see what the user printed with the keyboard
    type log_****.jsonl | Findstr "CharInput"
```

---

## Input Event Types

This program records keyboard and mouse input events. Each event is stored in a JSONL file with a timestamp and an event type. The following table shows the event types and their corresponding keywords:

| Type           | Enum Value                  | Description                                         |
|----------------|---------------------------- |--------------------------------------------------- |
| Key Press      | `KeyPress(String)`          | Captures a key press, e.g., `"KeyA"`              |
| Key Release    | `KeyRelease(String)`        | Captures a key release                             |
| Character Input| `CharInput(String)`         | Converts key presses to characters considering Shift and CapsLock, e.g., `"a"` or `"A"` |
| Mouse Press    | `ButtonPress(String)`       | Captures a mouse button press, e.g., `"Left"` or `"Right"` |
| Mouse Release  | `ButtonRelease(String)`     | Captures a mouse button release                     |
| Mouse Move     | `MouseMove {x, y}`          | Captures mouse position on the screen, e.g., `{x: 500.0, y: 300.0}` |
| Mouse Wheel    | `Wheel {delta_x, delta_y}`  | Captures mouse wheel movement, e.g., `{delta_x: 0, delta_y: -120}` |

---

## Note

- You can customize the params of the program by ways you like, 
- such as: 
- 1.changing the place to log the data
- 2.changing the buffer size(which is in the """struct InputLogger""")
- 3.changing the time to automatically terminate the program(which is in the """main function""").

---

## The Most Important thing you need to Know:
**WARNING**

This is a malicious program, which may be detected by your Anti-Virus, please use it properly. You Can only Test it on your !!!OWN!!! laptop
