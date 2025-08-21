import pygame
from pygame import Surface

class Board:
    """
    Stores the state of the current board as a 2D array which
    can be drawn to a Pygame surface.
    
    Empty spaces are represented as 0, with 1 and 2 representing
    the first and second player respectively.
    """

    ROWS: int = 6
    COLS: int = 7
    COLOURS: dict = {
        'background': (23, 32, 46),
        'board': (55, 65, 81),
        'empty': (31, 41, 55),
        'empty-outline': (28, 38, 53),
        'player1': (220, 38, 38),
        'player1-outline': (153, 27, 27),
        'player2': (251, 192, 36),
        'player2-outline': (245, 158, 11),
    }

    def __init__(self, surface_size=800) -> None:
        """Creates a new, empty board."""        
        # Calculates values used for drawing the board
        self.surface_size = surface_size
        self.padding = surface_size // 40
        self.cell_size = (surface_size - 2 * self.padding) // 10
        self.board_width = Board.COLS * self.cell_size + self.padding * 2
        self.board_height = Board.ROWS * self.cell_size + self.padding * 2
        self.board_x = (surface_size - self.board_width) // 2
        self.board_y = (surface_size - self.board_height) // 2
        self.piece_radius = int(self.cell_size * 0.45)
        self.piece_interior = int(self.piece_radius * 0.85)

        # Creates the surface used for drawing the board
        self.surface = Surface((surface_size, surface_size))
        self.reset()

    def is_playable(self, col: int) -> bool:
        """Returns whether the given column is playable."""
        if col is None or col < 0 or col >= Board.COLS:
            return False
        return self.heights[col] < Board.ROWS

    def play(self, col: int, player: int):
        """
        Places a player's tile in the specified column.
        
        Assumes a 0-indexed column and player are given.
        """
        # Plays the piece
        if not self.is_playable(col):
            raise Exception("attempted to play a move in a full column.")
        row = Board.ROWS - 1 - self.heights[col]
        self.grid[row][col] = player + 1
        self.heights[col] += 1
        self.update()

    def reset(self):
        """Resets the board state."""
        self.grid = [[0 for _ in range(Board.COLS)] for _ in range(Board.ROWS)]
        self.heights = [0 for _ in range(Board.COLS)]
        self.update()

    def update(self):
        """Updates the board's surface to reflect its current state."""
        self.surface.fill(self.COLOURS['background'])
        pygame.draw.rect(
            self.surface, 
            self.COLOURS['board'], 
            (self.board_x, self.board_y, self.board_width, self.board_height),
            border_radius=self.cell_size // 2
        )
        for i, row in enumerate(self.grid):
            for j, n in enumerate(row):
                x = self.padding + self.board_x + self.cell_size * (j + 0.5)
                y = self.padding + self.board_y + self.cell_size * (i + 0.5)
                piece_type = ['empty', 'player1', 'player2'][n]
                pygame.draw.circle(self.surface, self.COLOURS[f'{piece_type}-outline'],
                                   (x, y), self.piece_radius)
                pygame.draw.circle(self.surface, self.COLOURS[f'{piece_type}'],
                                   (x, y), self.piece_interior)


    def draw(self, target: Surface, pos: tuple = (0, 0)):
        """Draws the board onto the given target surface."""
        target.blit(self.surface, pos)
