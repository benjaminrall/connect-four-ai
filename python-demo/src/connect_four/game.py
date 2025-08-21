import pygame
import os
from connect_four_ai import Difficulty, AIPlayer, Position
from .board import Board
from concurrent.futures import ThreadPoolExecutor

BASE_DIR = os.path.dirname(os.path.abspath(__file__))

class ConnectFourGame:
    """Manages the main game loop and logic."""

    def __init__(self, difficulty: Difficulty, player: int, window_size=800):
        """Initialises a Connect Four game with the provided settings."""
        # Initialises the board and AI player
        self.position = Position()
        self.board = Board()
        self.player = player
        self.executor = ThreadPoolExecutor()
        self.ai_player = AIPlayer(difficulty)

        # Pygame display setup
        if not pygame.get_init():
            pygame.init()
        self.window_size = window_size
        self.win = pygame.display.set_mode((window_size, window_size))
        pygame.display.set_caption("Connect Four")
        try:
            icon_img = pygame.image.load(os.path.join(BASE_DIR, "assets", "icon.png"))
            pygame.display.set_icon(icon_img)
        except pygame.error as e:
            print(f"Could not load window icon: {e}")
        try:
            font_path = os.path.join(BASE_DIR, "assets", "nunito.ttf")
            self.font_big = pygame.font.Font(font_path, window_size // 12)
            self.font_small = pygame.font.Font(font_path, window_size // 24)
        except pygame.error as e:
            print(f"Could not load font.")
        self.overlay = pygame.Surface((self.window_size, self.window_size), pygame.SRCALPHA)
        self.overlay.fill((0, 0, 0, 160))
        self.clock = pygame.time.Clock()

        # Game state
        self.current_player = 0
        self.game_over = False
        self.end_state = None
        self.running = False
        self.play_ai()

    def reset(self):
        """Resets the game."""
        self.board.reset()
        self.position = Position()
        self.current_player = 0
        self.game_over = False
        self.end_state = None
        self.play_ai()

    def run(self):
        """Contains the main loop for the game."""
        self.running = True
        while self.running:
            # Handles Pygame events
            self._handle_events()

            # Draws the current state to the Pygame window
            self.board.draw(self.win)
            if self.game_over:
                self.draw_end_state(self.win)
            pygame.display.update()

            self.clock.tick(60)
        
        # Cleans up necessary resources
        self._cleanup()

    def _handle_events(self):
        """Processes all Pygame events for the game."""
        for event in pygame.event.get():
            # Handles quitting the Pygame window
            if event.type == pygame.QUIT:
                self.running = False
            # Handles any key presses
            elif event.type == pygame.KEYDOWN:
                self._handle_keypress(event.key)
    
    def _handle_keypress(self, key: int):
        """Handles all Pygame key press events."""
        # Quitting the game with 'Q'
        if key == pygame.K_q:
            self.running = False
        # Resetting the game with 'R'
        elif key == pygame.K_r:
            self.reset()
        # Playing columns with number keys
        elif key == pygame.K_1:
            self.play(0)
        elif key == pygame.K_2:
            self.play(1)
        elif key == pygame.K_3:
            self.play(2)
        elif key == pygame.K_4:
            self.play(3)
        elif key == pygame.K_5:
            self.play(4)
        elif key == pygame.K_6:
            self.play(5)
        elif key == pygame.K_7:
            self.play(6)

    def play(self, col: int):
        """Plays the given column if it's the player's turn."""
        if not self.game_over and self.player == self.current_player and self.board.is_playable(col):
            self.board.play(col, self.current_player)
            self.position.play(col)
            self.check_win(self.current_player)
            self.current_player = (self.current_player + 1) & 1
            self.play_ai()

    def play_ai(self):
        """Plays the AI's turn."""
        if not self.game_over and self.player != self.current_player:
            def callback(future):
                col = future.result()
                if self.board.is_playable(col):
                    self.board.play(col, self.current_player)
                    self.position.play(col)
                    self.check_win(self.current_player)
                    self.current_player = (self.current_player + 1) & 1

            future = self.executor.submit(self.ai_player.get_move, self.position)
            future.add_done_callback(callback)

    def check_win(self, player: int):
        """Checks for a win for the current player and sets the ending state accordingly."""
        if self.position.get_moves() == Board.COLS * Board.ROWS:
            self.end_state = 'draw'
        elif not self.position.is_won_position():
            return
        elif player == self.player:
            self.end_state = 'win'
        else:
            self.end_state = 'lose'
        self.game_over = True

    def draw_end_state(self, target: pygame.Surface, pos: tuple = (0, 0)):
        """Draws the current end state to the given target surface."""
        # Define text based on the end state
        if self.end_state == 'win':
            message = "You Win!"
        elif self.end_state == 'lose':
            message = "You Lose!"
        elif self.end_state == 'draw':
            message = "It's a Draw!"
        else:
            return

        self.win.blit(self.overlay, pos)

        # Render the main message
        text_surface = self.font_big.render(message, True, (255, 255, 255))
        text_rect = text_surface.get_rect(center=(target.get_width() // 2 + pos[0], target.get_height() // 2 - 30 + pos[1]))
        target.blit(text_surface, text_rect)

        # Render the "Press R to restart" message
        restart_text = self.font_small.render("Press 'R' to restart", True, (255, 255, 255))
        restart_rect = restart_text.get_rect(center=(target.get_width() // 2 + pos[0], target.get_height() // 2 + 30 + pos[1]))
        target.blit(restart_text, restart_rect)

    def _cleanup(self):
        """Ensures all resources are closed properly."""
        self.executor.shutdown()
        if pygame.get_init():
            pygame.quit()
        