# Android keyboard integration

Sample IME at `integrations/android-keyboard/`.

## Architecture

```text
NlpKeyboardService (InputMethodService)
        │
        ▼
NlpEngine (UniFFI Kotlin)
        │
        ▼
libnlp_engine_ind_uniffi.so (Rust)
```

## UX

- Language selector (Kannada, Hindi, Telugu, Tamil, Malayalam)
- **Convert ✨** button — explicit conversion (not per-keystroke)
- Space, backspace, enter
- Converts: selected text → composing text → current sentence

## Text replacement

1. If text is selected → replace selection
2. Else delete surrounding composing text and insert result

Conversion runs on `Dispatchers.Default` — UI thread is not blocked.

## Privacy

- No Full Access required
- No network permission
- No logging of user text
- No disk persistence

## Testing

```bash
./scripts/build-android.sh
# Open in Android Studio, run on device/emulator
```

Verify offline: enable airplane mode, convert text, confirm output appears.
