[![progress-banner](https://backend.codecrafters.io/progress/claude-code/b2ca15df-43ad-47d7-be4c-68d3cbc1e61d)](https://app.codecrafters.io/users/VhRvo?r=2qF)

This is a starting point for Rust solutions to the
["Build Your own Claude Code" Challenge](https://codecrafters.io/challenges/claude-code).

Claude Code is an AI coding assistant that uses Large Language Models (LLMs) to
understand code and perform actions through tool calls. In this challenge,
you'll build your own Claude Code from scratch by implementing an LLM-powered
coding assistant.

Along the way you'll learn about HTTP RESTful APIs, OpenAI-compatible tool
calling, agent loop, and how to integrate multiple tools into an AI assistant.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

# Passing the first stage

The entry point for your `claude-code` implementation is in `src/main.rs`. Study
and uncomment the relevant code, and submit to pass the first stage:

```sh
codecrafters submit
```

# Stage 2 & beyond

Note: This section is for stages 2 and beyond.

1. Ensure you have `cargo (1.96)` installed locally.
2. Run `./your_program.sh` to run your program, which is implemented in
   `src/main.rs`. This command compiles your Rust project, so it might be slow
   the first time you run it. Subsequent runs will be fast.
3. Run `codecrafters submit` to submit your solution to CodeCrafters. Test
   output will be streamed to your terminal.

# Using OpenRouter

Set your OpenRouter API key, then pass a prompt to the program:

```sh
export OPENROUTER_API_KEY="your-openrouter-api-key"
./your_program.sh -p "Explain what this project does"
```

The default model is `deepseek/deepseek-v4-flash`. You can select any model in
the OpenRouter catalog with `OPENROUTER_MODEL`:

```sh
export OPENROUTER_MODEL="provider/model-name"
./your_program.sh -p "Hello from OpenRouter"
```

The OpenRouter API base URL defaults to `https://openrouter.ai/api/v1`. It can
be overridden with `OPENROUTER_BASE_URL` when needed.
