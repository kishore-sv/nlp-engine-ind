param(
  [string]$Language = "kn",
  [Parameter(ValueFromRemainingArguments = $true)]
  [string[]]$Text
)

$Root = Resolve-Path (Join-Path $PSScriptRoot "../..")
Set-Location $Root
$joined = ($Text -join " ")
cargo run -p nlp-engine-ind-cli --release -- --language $Language $joined
