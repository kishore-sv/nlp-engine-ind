#ifndef NLP_ENGINE_IND_H
#define NLP_ENGINE_IND_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

typedef struct NlpEngineHandle NlpEngineHandle;

typedef struct {
    char *output;
    float confidence;
    uint64_t processing_time_us;
    int32_t error_code;
} NlpConversionResult;

NlpEngineHandle *nlp_engine_ind_create(const char *language_code);
void nlp_engine_ind_destroy(NlpEngineHandle *engine);
NlpConversionResult nlp_engine_ind_convert(NlpEngineHandle *engine, const char *input);
NlpConversionResult nlp_engine_ind_convert_once(const char *input, const char *language_code);
void nlp_engine_ind_free_result(NlpConversionResult result);

/* Legacy compatibility */
char *nlp_engine_ind_convert_legacy(const char *input, const char *language_code);
void nlp_engine_ind_free_string(char *s);

const char *nlp_engine_ind_version(void);

#ifdef __cplusplus
}
#endif

#endif /* NLP_ENGINE_IND_H */
