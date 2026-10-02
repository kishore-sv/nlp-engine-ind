import UIKit

final class KeyboardViewController: UIInputViewController {
    private var engine: NlpEngine?
    private let languageControl = UISegmentedControl(items: ["KN", "HI", "TE", "TA", "ML"])
    private let convertButton = UIButton(type: .system)
    private let statusLabel = UILabel()
    private let stack = UIStackView()

    override func viewDidLoad() {
        super.viewDidLoad()
        setupEngine()
        setupUI()
    }

    private func setupEngine() {
        do {
            engine = try NlpEngine(
                config: EngineConfig(
                    language: .kannada,
                    style: .casual,
                    codeMix: .natural,
                    romanization: .casualChat,
                    enabledLanguages: [.kannada, .hindi, .telugu, .tamil, .malayalam]
                )
            )
        } catch {
            statusLabel.text = "Engine init failed"
        }
    }

    private func setupUI() {
        languageControl.selectedSegmentIndex = 0
        convertButton.setTitle("Convert ✨", for: .normal)
        convertButton.addTarget(self, action: #selector(convertTapped), for: .touchUpInside)
        statusLabel.font = .systemFont(ofSize: 12)
        statusLabel.text = "Offline local NLP"
        statusLabel.textAlignment = .center

        stack.axis = .vertical
        stack.spacing = 8
        stack.translatesAutoresizingMaskIntoConstraints = false
        stack.addArrangedSubview(languageControl)
        stack.addArrangedSubview(convertButton)
        stack.addArrangedSubview(statusLabel)

        view.addSubview(stack)
        NSLayoutConstraint.activate([
            stack.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: 8),
            stack.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -8),
            stack.topAnchor.constraint(equalTo: view.topAnchor, constant: 8),
            stack.bottomAnchor.constraint(equalTo: view.bottomAnchor, constant: -8),
        ])
    }

    @objc private func convertTapped() {
        guard let engine else { return }
        let source = currentText().trimmingCharacters(in: .whitespacesAndNewlines)
        guard !source.isEmpty else {
            statusLabel.text = "Nothing to convert"
            return
        }

        let language = selectedLanguage()
        statusLabel.text = "Converting offline..."
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            do {
                try engine.setLanguage(language: language)
                let result = try engine.convertForKeyboard(text: source)
                DispatchQueue.main.async {
                    self?.insertConverted(source: source, converted: result.output)
                    self?.statusLabel.text = String(format: "Confidence %.2f • offline", result.confidence)
                }
            } catch {
                DispatchQueue.main.async {
                    self?.statusLabel.text = "Conversion failed"
                }
            }
        }
    }

    private func selectedLanguage() -> Language {
        switch languageControl.selectedSegmentIndex {
        case 1: return .hindi
        case 2: return .telugu
        case 3: return .tamil
        case 4: return .malayalam
        default: return .kannada
        }
    }

    private func currentText() -> String {
        if let selected = textDocumentProxy.selectedText, !selected.isEmpty {
            return selected
        }
        let before = textDocumentProxy.documentContextBeforeInput ?? ""
        return String(before.split(separator: "\n").last ?? Substring(before))
    }

    private func insertConverted(source: String, converted: String) {
        if let selected = textDocumentProxy.selectedText, !selected.isEmpty {
            textDocumentProxy.insertText(converted)
            return
        }
        for _ in 0..<source.count {
            textDocumentProxy.deleteBackward()
        }
        textDocumentProxy.insertText(converted)
    }
}
