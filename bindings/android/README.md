# Android bindings

UniFFI Kotlin bindings + native libraries for Android.

## Build

```bash
./scripts/build-android.sh
```

Produces:

```text
bindings/uniffi/generated/kotlin/     # UniFFI Kotlin API
bindings/android/nlp-engine-ind/src/main/jniLibs/arm64-v8a/libnlp_engine_ind_uniffi.so
```

## Gradle module

Include in your project:

```kotlin
include(":nlp-engine-ind")
project(":nlp-engine-ind").projectDir = file("path/to/bindings/android/nlp-engine-ind")
```

```kotlin
dependencies {
    implementation(project(":nlp-engine-ind"))
}
```

## Package

```kotlin
import uniffi.nlp_engine_ind.*
```

See [docs/android/library.md](../../docs/android/library.md).
