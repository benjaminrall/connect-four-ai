"""
A high performance implementation of a perfect Connect Four solver, written in Rust.

This library provides functionality to compute the score and optimal move for any
given Connect Four position.
"""

class Position:
    """Represents a Connect Four position compactly as a bitboard.

    The standard, 6x7 Connect Four board can be represented unambiguously
    using 49 bits in the following bit order:

       6 13 20 27 34 41 48
       5 12 19 26 33 40 47
       4 11 18 25 32 39 46
       3 10 17 24 31 38 45
       2  9 16 23 30 37 44
       1  8 15 22 29 36 43
       0  7 14 21 28 35 42

    The extra row of bits at the top identifies full columns and prevents
    bits from overflowing into the next column. For computational
    efficiency, positions are stored using two 64-bit unsigned integers:
    one storing a mask of all occupied tiles, and the other storing a mask
    of the current player's tiles.
    """

    WIDTH: int
    HEIGHT: int

    position: int
    """A bitmask of the current player's tiles."""

    mask: int
    """A bitmask of all occupied tiles."""

    def __init__(self) -> None:
        """Creates a new position instance for the initial state of the game."""

    @staticmethod
    def from_moves(moves: str) -> Position:
        """
        Parses a position from a string of 1-indexed moves.

        The input string should contain a sequence of columns played, indexed from 1.
        """

    @staticmethod
    def from_board_string(board_string: str) -> Position:
        """
        Parses a position from a string representation of the Connect Four board.

        The input string should contain exactly 42 characters from the set `['.', 'o', 'x']`,
        representing the board row by row from the top-left to the bottom-right. All other
        characters are ignored. 'x' is treated as the current player, and 'o' as the opponent.
        This method assumes that a correctly formatted board string is a valid game position.
        Invalid game positions will lead to undefined behaviour.
        """

    def __str__(self) -> str:
        """Returns the position's attributes formatted as a string."""

    def get_key(self) -> int:
        """Returns the unique key for the current position."""

    def get_moves(self) -> int:
        """Returns the number of moves played to reach the current position."""

    def is_playable(self, col: int) -> bool:
        """Indicates whether a given column is playable.

        Parameters
        ----------
        col : int
            0-based index of a column.

        Returns
        -------
        bool
            True if the column is playable, false if the column is already full.
        """

    def is_winning_move(self, col: int) -> bool:
        """Indicates whether the current player wins by playing a given column.

        Parameters
        ----------
        col : int
            0-based index of a playable column.

        Returns
        -------
        bool
            True if the current player makes a 4-alignment by playing the column, false otherwise.
        """

    def can_win_next(self) -> bool:
        """Indicates whether the current player can win with their next move."""

    def play(self, col: int) -> None:
        """Plays a move in the given column.

        Parameters
        ----------
        col : int
            0-based index of a playable column.
        """

    def possible(self) -> int:
        """Returns a mask for the possible moves the current player can make."""

    def possible_non_losing_moves(self) -> int:
        """Returns a mask for the possible non-losing moves the current player can make."""

    def is_won_position(self) -> bool:
        """Indicates whether the current position has been won by either player."""

    @staticmethod
    def column_mask(col: int) -> int:
        """Returns a mask for the entirety of the given column.

        Parameters
        ----------
        col : int
            0-based index of a column.

        Returns
        -------
        int
            A bitmask with a one in all cells of the column.
        """

class Solver:
    """
    A strong solver for finding the exact score of Connect Four positions.

    This class implements a high-performance negamax search algorithm with several
    optimisations, including:
    - Alpha-beta pruning
    - Score-based move ordering to prioritise stronger moves
    - A transposition table to cache results of previously seen positions
    - A binary search on the score for faster convergence
    """

    explored_positions: int
    """A counter for the number of nodes explored in the last `solve` call."""

    def __init__(self) -> None:
        """Creates a new `Solver` instance, using the pre-packaged opening book."""

    def load_opening_book(self, path: str) -> bool:
        """
        Attempts to load an opening book from the given path.
        
        Returns whether the opening book was successfully loaded.
        """

    def reset(self) -> None:
        """Resets the solver's state."""

    def solve(self, position: Position) -> int:
        """Solves a position to find its exact score.

        This function uses a binary search over the possible score range, repeatedly calling
        the negamax search with a null window to test if the score is above a certain value.
        This allows for faster convergence to the true score.

        Assumes that the given position is valid and not won by either player.

        Parameters
        ----------
        position : Position
            The board position to solve.

        Returns
        -------
        int
            The exact score of the position, which reflects the outcome of the game assuming 
            that both players play perfectly. A position has:
            - A positive score if the current player will win. 1 if they win with their last move, 2 if
            they win with their second to last move, ...
            - A null score if the game will end in a draw
            - A negative score if the current player will lose. -1 if the opponent wins with their last
            move, -2 if the opponent wins with their second to last move, ...
        """

    def get_all_move_scores(self, position: Position) -> list[int | None]:
        """
        Calculates the scores for all possible next moves in the given position.
        
        Returns a fixed-size array where each index corresponds to a column, containing
        an integer score if a move in that column is possible and `None` if the column is full 
        and the move is impossible.
        
        This array can be used to directly calculate the optimal move to play in a position.
        """

class Difficulty:
    """An enum to represent the difficulty of an AI player."""

    EASY: Difficulty
    MEDIUM: Difficulty
    HARD: Difficulty
    IMPOSSIBLE: Difficulty

class AIPlayer:
    """
    An AI player that uses a solver to determine the best move to play in a Connect Four position.

    The player's skill level can be configured using the `Difficulty` enum, which adjusts the
    move selection strategy.
    """

    def __init__(self, difficulty: Difficulty = Difficulty.IMPOSSIBLE) -> None:
        """Creates a new AI player with a default solver and specified difficulty."""

    def load_opening_book(self, path: str) -> bool:
        """
        Attempts to load an opening book from the given path for the AI player's solver.
        
        Returns whether the opening book was successfully loaded.
        """

    def reset(self) -> None:
        """Resets the AI player's solver."""
    
    def solve(self, position: Position) -> int:
        """Solves a position to find its exact score using the AI player's solver."""

    def get_all_move_scores(self, position: Position) -> list[int | None]:
        """
        Calculates the scores for all possible next moves in the given position
        using the AI player's solver.
        """

    def get_move(self, position: Position) -> int | None:
        """Solves and selects the AI player's move for the given position."""

    def select_move(self, position: Position, scores: list[int | None]) -> int | None:
        """
        Selects a move from an array of scores using a Softmax distribution with a
        temperature defined by the AI player's difficulty. Temperature values <= 0 will
        result in greedy selection (always picking the best move).

        Returns the column index of the selected move or `None` if no moves are possible.
        """
        
