# AIChat

[![CI](https://github.com/sigoden/aichat/actions/workflows/ci.yaml/badge.svg)](https://github.com/sigoden/aichat/actions/workflows/ci.yaml)
[![Crates](https://img.shields.io/crates/v/aichat.svg)](https://crates.io/crates/aichat)

Chat with ChatGPT-3.5 in the terminal.

![demo](https://user-images.githubusercontent.com/4012553/222600858-3fb60051-2bf2-4505-92ff-649356cdb1f6.gif)

## Install

### With cargo

```
cargo install --force aichat
```

### Binaries on macOS, Linux, Windows

Download from [Github Releases](https://github.com/sigoden/aichat/releases), unzip and add opscan to your $PATH.

## Features

- Compared to the browser, the terminal starts faster and needs less resources.
- Support directive and interactive modes
- Support markdown highlighting.
- Predefine prompts for role playing.
- History query/completion.
- Persist chat messages.
- Support proxy.
- Written in rust, single executable file, cross-platform.

## Config

On first launch, aichat will guide you through configuration.

```
> No config file, create a new one? Yes
> Openai API Key: sk-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
> Use proxy? Yes
> Set proxy: socks5://127.0.0.1:1080
> Save chat messages Yes
```

After setting, it will automatically create the configuration file. Of course, you can also manually set the configuration file. 

```yaml
api_key: "<YOUR SECRET API KEY>"        # Request via https://platform.openai.com/account/api-keys
temperature: 1.0                        # optional, see https://platform.openai.com/docs/api-reference/chat/create#chat/create-temperature
save: true                              # optional, If set to true, aichat will save chat messages to message.md
no_highlight: false                     # optional, Whether to disable highlight
proxy: "socks5://127.0.0.1:1080"        # optional, set proxy server. e.g. http://127.0.0.1:8080 or socks5://127.0.0.1:1080
```

The default config dir is as follows, You can override config dir with `$AICHAT_CONFIG_DIR`.

- Linux:   `/home/alice/.config/aichat`
- Windows: `C:\Users\Alice\AppData\Roaming\aichat`
- MacOS:   `/Users/Alice/Library/Application Support`

aichat may generate the following files in the config dir:

- `config.yaml`: the config file.
- `roles.yaml`: the roles definition file.
- `history.txt`: the repl history file.
- `messages.md`: the chat messages storage file.

### Roles

We can let ChatGPT play a certain role through `prompt` to make it better generate what we want. See [awesome-chatgpt-prompts](https://github.com/f/awesome-chatgpt-prompts) for details.

We can predefine a batch of roles in `roles.yaml`. For example, we define a javascript-console role as follows.

```yaml
- name: javascript-console
  prompt: > 
    I want you to act as a javascript console. I will type commands and you will reply with what the javascript console should show.
    I want you to only reply with the terminal output inside one unique code block, and nothing else.
    do not write explanations. do not type commands unless I instruct you to do so. 
    when i need to tell you something in english, i will do so by putting text inside curly brackets {like this}.
    My first command is:
```

Let ChaGPT answer questions in the role of a javascript-console.

```
aichat --role javascript-console console.log("Hello World")
```

In interactive mode, we do this:

```
〉.role javascript-console

〉console.log("Hello world")
```

## License

Copyright (c) 2023 aichat-developers.

aichat is made available under the terms of either the MIT License or the Apache License 2.0, at your option.

See the LICENSE-APACHE and LICENSE-MIT files for license details.