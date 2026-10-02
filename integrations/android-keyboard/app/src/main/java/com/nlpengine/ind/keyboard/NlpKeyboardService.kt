package com.nlpengine.ind.keyboard

import android.inputmethodservice.InputMethodService
import android.view.LayoutInflater
import android.view.View
import android.view.inputmethod.InputConnection
import android.widget.ArrayAdapter
import android.widget.Button
import android.widget.Spinner
import android.widget.TextView
import android.widget.Toast
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import uniffi.nlp_engine_ind.CodeMixMode
import uniffi.nlp_engine_ind.ConversionContext
import uniffi.nlp_engine_ind.ConversionRequest
import uniffi.nlp_engine_ind.EngineConfig
import uniffi.nlp_engine_ind.Language
import uniffi.nlp_engine_ind.NlpEngine
import uniffi.nlp_engine_ind.RomanizationStyle
import uniffi.nlp_engine_ind.Style

class NlpKeyboardService : InputMethodService() {

    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.Main.immediate)
    private lateinit var engine: NlpEngine
    private lateinit var statusText: TextView
    private lateinit var languageSpinner: Spinner

    private val languages = listOf(
        "Kannada" to Language.KANNADA,
        "Hindi" to Language.HINDI,
        "Telugu" to Language.TELUGU,
        "Tamil" to Language.TAMIL,
        "Malayalam" to Language.MALAYALAM,
    )

    override fun onCreate() {
        super.onCreate()
        engine = NlpEngine(
            EngineConfig(
                language = Language.KANNADA,
                style = Style.CASUAL,
                codeMix = CodeMixMode.NATURAL,
                romanization = RomanizationStyle.CASUAL_CHAT,
                enabledLanguages = languages.map { it.second },
            ),
        )
    }

    override fun onCreateInputView(): View {
        val view = LayoutInflater.from(this).inflate(R.layout.keyboard_view, null)
        statusText = view.findViewById(R.id.statusText)
        languageSpinner = view.findViewById(R.id.languageSpinner)
        languageSpinner.adapter = ArrayAdapter(
            this,
            android.R.layout.simple_spinner_dropdown_item,
            languages.map { it.first },
        )

        view.findViewById<Button>(R.id.convertButton).setOnClickListener { convertCurrentText() }
        view.findViewById<Button>(R.id.spaceButton).setOnClickListener {
            currentInputConnection?.commitText(" ", 1)
        }
        view.findViewById<Button>(R.id.backspaceButton).setOnClickListener {
            currentInputConnection?.deleteSurroundingText(1, 0)
        }
        view.findViewById<Button>(R.id.enterButton).setOnClickListener {
            currentInputConnection?.commitText("\n", 1)
        }
        return view
    }

    private fun convertCurrentText() {
        val ic: InputConnection = currentInputConnection ?: return
        val selected = ic.getSelectedText(0)?.toString()
        val before = ic.getTextBeforeCursor(1000, 0)?.toString().orEmpty()
        val source = when {
            !selected.isNullOrBlank() -> selected
            before.isNotBlank() -> before.takeLastWhile { it != '\n' }
            else -> return
        }.trim()
        if (source.isEmpty()) {
            Toast.makeText(this, "Nothing to convert", Toast.LENGTH_SHORT).show()
            return
        }

        val language = languages[languageSpinner.selectedItemPosition].second
        statusText.text = "Converting offline..."

        scope.launch {
            try {
                val result = withContext(Dispatchers.Default) {
                    engine.setLanguage(language)
                    engine.convert(
                        ConversionRequest(
                            text = source,
                            language = language,
                            style = Style.CASUAL,
                            codeMix = CodeMixMode.NATURAL,
                            romanization = RomanizationStyle.CASUAL_CHAT,
                            context = ConversionContext(
                                selectedText = selected,
                                currentSentence = source,
                                previousText = null,
                            ),
                        ),
                    )
                }
                replaceText(ic, source, result.output)
                statusText.text = "Confidence ${"%.2f".format(result.confidence)} • offline"
            } catch (e: Exception) {
                statusText.text = "Conversion failed"
                Toast.makeText(
                    this@NlpKeyboardService,
                    e.message ?: "Conversion failed",
                    Toast.LENGTH_SHORT,
                ).show()
            }
        }
    }

    private fun replaceText(ic: InputConnection, original: String, converted: String) {
        if (ic.getSelectedText(0)?.isNotEmpty() == true) {
            ic.commitText(converted, 1)
            return
        }
        ic.deleteSurroundingText(original.length, 0)
        ic.commitText(converted, 1)
    }
}
