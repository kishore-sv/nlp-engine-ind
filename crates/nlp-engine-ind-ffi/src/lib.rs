//! C ABI for nlp-engine-ind — used by mobile and desktop integrations.

use nlp_engine_ind_core::{ConversionRequest, Engine, EngineError, Language};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_float, c_int, c_ulonglong};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;

/// Opaque engine handle.
pub struct NlpEngineHandle {
    engine: Engine,
}

/// Result returned across the FFI boundary. Must be freed with `nlp_engine_ind_free_result`.
#[repr(C)]
pub struct NlpConversionResult {
    pub output: *mut c_char,
    pub confidence: c_float,
    pub processing_time_us: c_ulonglong,
    pub error_code: c_int,
}

const ERR_OK: c_int = 0;
const ERR_NULL: c_int = 1;
const ERR_UTF8: c_int = 2;
const ERR_UNSUPPORTED_LANGUAGE: c_int = 3;
const ERR_EMPTY_INPUT: c_int = 4;
const ERR_INVALID_INPUT: c_int = 5;
const ERR_GENERATION: c_int = 6;
const ERR_CONFIG: c_int = 7;
const ERR_PANIC: c_int = 99;

fn map_error_code(err: EngineError) -> c_int {
    match err {
        EngineError::UnsupportedLanguage { .. } => ERR_UNSUPPORTED_LANGUAGE,
        EngineError::EmptyInput => ERR_EMPTY_INPUT,
        EngineError::InvalidInput { .. } => ERR_INVALID_INPUT,
        EngineError::GenerationFailure { .. } => ERR_GENERATION,
        EngineError::InvalidConfiguration { .. } => ERR_CONFIG,
        EngineError::ResourceMissing { .. } => ERR_GENERATION,
    }
}

fn parse_language(code: &str) -> Option<Language> {
    Language::from_code(code)
}

fn empty_result(error_code: c_int) -> NlpConversionResult {
    NlpConversionResult {
        output: ptr::null_mut(),
        confidence: 0.0,
        processing_time_us: 0,
        error_code,
    }
}

/// Create a new engine for the given language code (`kn`, `hi`, `te`, `ta`, `ml`).
///
/// # Safety
/// `language_code` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn nlp_engine_ind_create(
    language_code: *const c_char,
) -> *mut NlpEngineHandle {
    catch_unwind(AssertUnwindSafe(|| {
        if language_code.is_null() {
            return ptr::null_mut();
        }
        let lang_str = match CStr::from_ptr(language_code).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut(),
        };
        let language = match parse_language(lang_str) {
            Some(l) => l,
            None => return ptr::null_mut(),
        };
        match Engine::with_language(language) {
            Ok(engine) => Box::into_raw(Box::new(NlpEngineHandle { engine })),
            Err(_) => ptr::null_mut(),
        }
    }))
    .unwrap_or(ptr::null_mut())
}

/// Destroy an engine created by `nlp_engine_ind_create`.
///
/// # Safety
/// `engine` must be a valid pointer returned by `nlp_engine_ind_create`, or null.
#[no_mangle]
pub unsafe extern "C" fn nlp_engine_ind_destroy(engine: *mut NlpEngineHandle) {
    if !engine.is_null() {
        drop(Box::from_raw(engine));
    }
}

/// Convert English text using a previously created engine.
///
/// # Safety
/// `engine` must be valid. `input` must be valid UTF-8. Free result with `nlp_engine_ind_free_result`.
#[no_mangle]
pub unsafe extern "C" fn nlp_engine_ind_convert(
    engine: *mut NlpEngineHandle,
    input: *const c_char,
) -> NlpConversionResult {
    catch_unwind(AssertUnwindSafe(|| {
        if engine.is_null() || input.is_null() {
            return empty_result(ERR_NULL);
        }
        let handle = &*engine;
        let input_str = match CStr::from_ptr(input).to_str() {
            Ok(s) => s,
            Err(_) => return empty_result(ERR_UTF8),
        };
        match handle.engine.convert_text(input_str) {
            Ok(result) => {
                let latency = result.processing_time_us();
                NlpConversionResult {
                    output: CString::new(result.output)
                        .map(|s| s.into_raw())
                        .unwrap_or(ptr::null_mut()),
                    confidence: result.confidence,
                    processing_time_us: latency,
                    error_code: ERR_OK,
                }
            }
            Err(err) => empty_result(map_error_code(err)),
        }
    }))
    .unwrap_or(empty_result(ERR_PANIC))
}

/// One-shot conversion without creating a persistent engine.
///
/// # Safety
/// See `nlp_engine_ind_convert`.
#[no_mangle]
pub unsafe extern "C" fn nlp_engine_ind_convert_once(
    input: *const c_char,
    language_code: *const c_char,
) -> NlpConversionResult {
    catch_unwind(AssertUnwindSafe(|| {
        if input.is_null() || language_code.is_null() {
            return empty_result(ERR_NULL);
        }
        let input_str = match CStr::from_ptr(input).to_str() {
            Ok(s) => s,
            Err(_) => return empty_result(ERR_UTF8),
        };
        let lang_str = match CStr::from_ptr(language_code).to_str() {
            Ok(s) => s,
            Err(_) => return empty_result(ERR_UTF8),
        };
        let language = match parse_language(lang_str) {
            Some(l) => l,
            None => return empty_result(ERR_UNSUPPORTED_LANGUAGE),
        };
        let engine = match Engine::with_language(language) {
            Ok(e) => e,
            Err(err) => return empty_result(map_error_code(err)),
        };
        match engine.convert(ConversionRequest::new(input_str, language)) {
            Ok(result) => {
                let latency = result.processing_time_us();
                NlpConversionResult {
                    output: CString::new(result.output)
                        .map(|s| s.into_raw())
                        .unwrap_or(ptr::null_mut()),
                    confidence: result.confidence,
                    processing_time_us: latency,
                    error_code: ERR_OK,
                }
            }
            Err(err) => empty_result(map_error_code(err)),
        }
    }))
    .unwrap_or(empty_result(ERR_PANIC))
}

/// Free strings inside a conversion result returned by this library.
///
/// # Safety
/// `result.output` must have been returned by this library, or be null.
#[no_mangle]
pub unsafe extern "C" fn nlp_engine_ind_free_result(result: NlpConversionResult) {
    if !result.output.is_null() {
        drop(CString::from_raw(result.output));
    }
}

/// Legacy one-shot API retained for compatibility.
///
/// # Safety
/// See `nlp_engine_ind_convert_once`.
#[no_mangle]
pub unsafe extern "C" fn nlp_engine_ind_convert_legacy(
    input: *const c_char,
    language_code: *const c_char,
) -> *mut c_char {
    let result = nlp_engine_ind_convert_once(input, language_code);
    result.output
}

/// Legacy string free API retained for compatibility.
///
/// # Safety
/// `s` must be a string previously returned by this library, or null.
#[no_mangle]
pub unsafe extern "C" fn nlp_engine_ind_free_string(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}

/// Returns library version as a static string.
///
/// # Safety
/// The returned pointer is static and must not be freed.
#[no_mangle]
pub unsafe extern "C" fn nlp_engine_ind_version() -> *const c_char {
    c"0.1.0".as_ptr()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn create_convert_destroy() {
        let lang = CString::new("kn").unwrap();
        let input = CString::new("I'll call you later").unwrap();
        let engine = unsafe { nlp_engine_ind_create(lang.as_ptr()) };
        assert!(!engine.is_null());
        let result = unsafe { nlp_engine_ind_convert(engine, input.as_ptr()) };
        assert_eq!(result.error_code, ERR_OK);
        assert!(!result.output.is_null());
        unsafe {
            nlp_engine_ind_free_result(result);
            nlp_engine_ind_destroy(engine);
        }
    }

    #[test]
    fn convert_once_works() {
        let lang = CString::new("kn").unwrap();
        let input = CString::new("I'll call you later").unwrap();
        let result = unsafe { nlp_engine_ind_convert_once(input.as_ptr(), lang.as_ptr()) };
        assert_eq!(result.error_code, ERR_OK);
        unsafe {
            nlp_engine_ind_free_result(result);
        }
    }
}
