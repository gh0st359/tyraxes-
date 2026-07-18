# Troubleshooting

## Wrong agent personality

Check:

```sh
echo $TYRAXES_AGENT $GROK_AGENT
tyraxes inspect
```

Ensure `[agent] name = "red-team"` or model `agent_type = "red-team"`.

## Model connection errors

- Verify `base_url` includes `/v1` when required
- Check API key env vars
- For Ollama: `ollama serve` + model pulled
- For LM Studio: local server started

## Infinite tool repeats

Anti-loop should nudge/stop. If not progressing:

- lower temperature
- ask for a distinct next hypothesis
- inspect whether PATH/tool missing caused empty output

## Config not found

Confirm home resolution:

```sh
echo $TYRAXES_HOME $GROK_HOME
ls -la ~/.tyraxes ~/.grok 2>/dev/null
```

## Binary name confusion

Build produces `tyraxes`, `grok`, and `xai-grok-pager` from the same package.
Prefer `tyraxes`.
