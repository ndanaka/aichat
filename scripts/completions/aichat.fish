complete -c aichat -s m -l model -x -a "(aichat --list-models)" -d 'Select a LLM model' -r
complete -c aichat -s r -l role -x -a "(aichat --list-roles)" -d 'Select a role' -r
complete -c aichat -s s -l session -x  -a"(aichat --list-sessions)" -d 'Start or join a session' -r
complete -c aichat -s f -l file -d 'Include files with the message' -r -F
complete -c aichat -s w -l wrap -d 'Control text wrapping (no, auto, <max-width>)'
complete -c aichat -l save-session -d 'Forces the session to be saved'
complete -c aichat -l serve -d 'Serve all LLMs as OpenAI-compatible API'
complete -c aichat -s e -l execute -d 'Execute commands in natural language'
complete -c aichat -s c -l code -d 'Output code only'
complete -c aichat -s H -l no-highlight -d 'Turn off syntax highlighting'
complete -c aichat -s S -l no-stream -d 'Turns off stream mode'
complete -c aichat -l light-theme -d 'Use light theme'
complete -c aichat -l dry-run -d 'Display the message without sending it'
complete -c aichat -l info -d 'Display information'
complete -c aichat -l list-models -d 'List all available models'
complete -c aichat -l list-roles -d 'List all available roles'
complete -c aichat -l list-sessions -d 'List all available sessions'
complete -c aichat -s h -l help -d 'Print help'
complete -c aichat -s V -l version -d 'Print version'
