# Android library

The Android library module wraps the Rust UniFFI crate.

## Module location

```text
bindings/android/nlp-engine-ind/
├── build.gradle.kts
├── src/main/jniLibs/          # Generated .so files
└── src/main/AndroidManifest.xml
```

## Gradle integration

```kotlin
dependencies {
    implementation(project(":nlp-engine-ind"))
}
```

The module includes:

- UniFFI-generated Kotlin (`bindings/uniffi/generated/kotlin`)
- Native libraries (`jniLibs/arm64-v8a/`)
- JNA for FFI (`net.java.dev.jna:jna`)

## Kotlin API

```kotlin
import uniffi.nlp_engine_ind.*

val engine = NlpEngine(
    EngineConfig(
        language = Language.KANNADA,
        style = Style.CASUAL,
        codeMix = CodeMixMode.NATURAL,
        romanization = RomanizationStyle.CASUAL_CHAT,
        enabledLanguages = listOf(Language.KANNADA),
    )
)

val result = engine.convertText("I'll call you later")
println(result.output)
```

## Keyboard helper

```kotlin
val result = engine.convertForKeyboard("I'll call you later")
// result.output, result.confidence, result.processingTimeUs
```

## Building AAR

From Android Studio: Build → Make Module 'nlp-engine-ind'.

Or via Gradle:

```bash
cd integrations/android-keyboard
./gradlew :nlp-engine-ind:assembleRelease
```

AAR output: `bindings/android/nlp-engine-ind/build/outputs/aar/`

## No duplicate NLP logic

All conversion happens in Rust. Kotlin is a thin wrapper only.
