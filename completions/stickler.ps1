# PowerShell completion for stickler
Register-ArgumentCompleter -Native -CommandName stickler -ScriptBlock {
  param($wordToComplete, $commandAst, $cursorPosition)
  $subcommands = @('check', 'render', 'review', 'census', 'rejudged', 'split', 'contested', 'pattern', 'files', 'borderline', 'outcomes', 'verdicts', 'shapes', 'choices', 'categories', 'grounds', 'judgements', 'standings', 'states', 'flags', 'models')
  $elements = $commandAst.CommandElements
  $argIndex = 0
  for ($i = 0; $i -lt $elements.Count; $i++) {
    $ext = $elements[$i].Extent
    if ($cursorPosition -le $ext.EndOffset) {
      $argIndex = $i
      break
    }
    $argIndex = $i + 1
  }
  if ($argIndex -eq 1) {
    $subcommands | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
      [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
    }
    return
  }
}
