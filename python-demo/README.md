# Connect Four AI Demo

A Python demo for the [`connect-four-ai`](https://pypi.org/project/connect-four-ai/) library.

![Connect Four GIF](https://github.com/user-attachments/assets/bb7dff1f-3a27-4f0a-b6ab-b46f19df6fd6)

This project uses Pygame to allow you to play against AI players of varying difficulties.
For full details about the library used to create this, please see the main
[GitHub Repository](https://github.com/benjaminrall/connect-four-ai).

## Installation and Usage

To play the demo, you first need to install it using `pip`:

```shell
pip install connect-four-ai-demo
```

Then, you can use the `connect-four` command to load the game. This command
comes with two command-line arguments:

| Option         | Short Flag | Description                                                                                 | Default Value |
|:---------------|:-----------|:--------------------------------------------------------------------------------------------|:--------------|
| `--difficulty` | `-d`       | The difficulty of the AI player. Must be one of:<br/>'easy', 'medium', 'hard', 'impossible' | impossible    |
| `--player`     | `-p`       | Which player to control (1 = red, 2 = yellow)                                               | 1             |

### Example Usage

This example loads the game against a medium difficulty opponent, playing as yellow:

```shell
connect-four -d medium -p 2
```

## License

This project is licensed under the **MIT License**. See the [`LICENSE`](https://github.com/benjaminrall/connect-four-ai/blob/main/LICENSE) file for details.
