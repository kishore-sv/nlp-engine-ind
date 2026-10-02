# Code mixing

Code mixing is intentional — the engine preserves commonly used English words when they appear naturally in Indian chat.

## Modes

| Mode | Behavior |
|------|----------|
| `Natural` | Preserve English for work/chat vocabulary (`work`, `call`, `project`, …) |
| `Minimal` | Prefer local words where lexicon has entries |
| `Aggressive` | Maximize local-language content |

## Protected vocabulary

Words like `work`, `call`, `meeting`, `project`, `file`, `office`, `free`, `busy`, `update`, `check`, `send` are defined in `data/common/code_mix.json` and preserved in Natural mode.

## Protected tokens

URLs, @mentions, #hashtags, email addresses, and brand names bypass translation entirely.

## Example

Input: `I'll finish this work and call you later`

Kannada Natural output: `Ee work mugsi call madtini` (English `work` and `call` preserved)

## Configuration

Set via `ConversionRequest.code_mixing` or `EngineConfig.code_mixing`.
