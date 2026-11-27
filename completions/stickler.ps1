# PowerShell completion for stickler
Register-ArgumentCompleter -Native -CommandName stickler -ScriptBlock {
  param($wordToComplete, $commandAst, $cursorPosition)
  $subcommands = @('check', 'render', 'review', 'census', 'rejudged', 'split', 'contested', 'pattern', 'files', 'borderline', 'outcomes', 'verdicts', 'shapes', 'choices', 'categories', 'grounds', 'judgements', 'standings', 'states', 'flags', 'models')
  $elements = $commandAst.CommandElements
  if ($elements.Count -eq 1 -or ($elements.Count -eq 2 -and $wordToComplete.Length -gt 0)) {
    $subcommands | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
      [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
    }
    return
  }
}
