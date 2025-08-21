import pygame
import argparse
from .game import ConnectFourGame
from connect_four_ai import Difficulty

def main():
    """Initialises Pygame, parses command line arguments, and runs the game."""
    # Parses the command line argument for difficulty
    parser = argparse.ArgumentParser(
        description="Play a game of Connect Four against an AI opponent."
    )
    parser.add_argument(
        "--difficulty", "-d",
        nargs="?",
        type=str,
        default='impossible',
        choices=['easy', 'medium', 'hard', 'impossible'],
        help="Sets the difficulty of the AI. Default: impossible"
    )
    parser.add_argument(
        "--player", "-p",
        nargs="?",
        type=int,
        default=1,
        choices=[1, 2],
        help="Sets the controlled player (1 = Red, 2 = Yellow). Default: 1"
    )
    args = parser.parse_args()

    # Attempts to create and run the game with the given arguments
    try:
        # Maps the string command-line argument to a true difficulty
        difficulty_map = {
            'easy': Difficulty.EASY,
            'medium': Difficulty.MEDIUM,
            'hard': Difficulty.HARD,
            'impossible': Difficulty.IMPOSSIBLE
        }
        if args.difficulty not in difficulty_map:
            raise Exception("invalid difficulty specified.")
        difficulty = difficulty_map[args.difficulty]

        if args.player not in [1, 2]:
            raise Exception("invalid player specified.")

        game = ConnectFourGame(difficulty, args.player - 1)
        game.run()
    except Exception as e:
        print(f"An unexpected error occurred: {e}")

if __name__ == "__main__":
    main()