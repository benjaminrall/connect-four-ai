/* tslint:disable */
/* eslint-disable */
/**
 * Sets up a hook to log Rust panics to the browser's console when the
 * WASM module is first loaded.
 */
export function start(): void;
/**
 * An AI player that uses a solver to determine the best move to play in a Connect Four position.
 *
 * The player's skill level can be configured using the `Difficulty` enum, which adjusts the
 * move selection strategy.
 */
export class AIPlayer {
  free(): void;
  /**
   * Creates a new AI player with a default solver and specified difficulty.
   */
  constructor(difficulty: Difficulty);
  /**
   * Attempts to load an opening book from the given path for the AI player's solver.
   *
   * Returns whether the opening book was successfully loaded.
   */
  loadOpeningBook(path: string): boolean;
  /**
   * Resets the AI player's solver.
   */
  reset(): void;
  /**
   * Solves a position to find its exact score using the AI player's solver.
   */
  solve(position: Position): number;
  /**
   * Calculates the scores for all possible next moves in the given position using the
   * AI player's solver.
   */
  getAllMoveScores(position: Position): any[];
  /**
   * Solves and selects the AI player's move for the given position.
   */
  getMove(position: Position): number | undefined;
  /**
   * Selects a move from an array of scores using a Softmax distribution with a
   * temperature defined by the AI player's difficulty. Temperature values <= 0 will
   * result in greedy selection (always picking the best move).
   *
   * Returns the column index of the selected move or `None` if no moves are possible.
   */
  selectMove(position: Position, scores: any[]): number | undefined;
}
/**
 * An enum to represent the difficulty of an AI player.
 */
export class Difficulty {
  private constructor();
  free(): void;
  static readonly EASY: Difficulty;
  static readonly MEDIUM: Difficulty;
  static readonly HARD: Difficulty;
  static readonly IMPOSSIBLE: Difficulty;
}
/**
 * Represents a Connect Four position compactly as a bitboard.
 *
 * The standard, 6x7 Connect Four board can be represented unambiguously using 49 bits
 * in the following bit order:
 *
 * ```comment
 *   6 13 20 27 34 41 48
 *  ---------------------
 * | 5 12 19 26 33 40 47 |
 * | 4 11 18 25 32 39 46 |
 * | 3 10 17 24 31 38 45 |
 * | 2  9 16 23 30 37 44 |
 * | 1  8 15 22 29 36 43 |
 * | 0  7 14 21 28 35 42 |
 *  ---------------------
 * ```
 *
 * The extra row of bits at the top identifies full columns and prevents
 * bits from overflowing into the next column. For computational
 * efficiency, positions are stored using two 64-bit unsigned integers:
 * one storing a mask of all occupied tiles, and the other storing a mask
 * of the current player's tiles.
 */
export class Position {
  free(): void;
  /**
   * Creates a new position instance for the initial state of the game.
   */
  constructor();
  /**
   * Parses a position from a string of 1-indexed moves.
   *
   * The input string should contain a sequence of columns played, indexed from 1.
   */
  static fromMoves(moves: string): Position;
  /**
   * Parses a position from a string representation of the Connect Four board.
   *
   * The input string should contain exactly 42 characters from the set `['.', 'o', 'x']`,
   * representing the board row by row from the top-left to the bottom-right. All other
   * characters are ignored. 'x' is treated as the current player, and 'o' as the opponent.
   * This method assumes that a correctly formatted board string is a valid game position.
   * Invalid game positions will lead to undefined behaviour.
   */
  static fromBoardString(board_string: string): Position;
  /**
   * Returns the Position's attributes formatted as a string.
   */
  toString(): string;
  /**
   * Returns the unique key for the current position.
   *
   * This key is unique to each pair of horizontally symmetrical positions, as these
   * positions will always have the same solution.
   */
  getKey(): bigint;
  /**
   * Returns the number of moves played to reach the current position.
   */
  getMoves(): number;
  /**
   * Indicates whether a given column is playable.
   *
   * # Arguments
   *
   * * `col`: 0-based index of a column.
   *
   * # Returns
   *
   * True if the column is playable, false if the column is already full.
   */
  isPlayable(col: number): boolean;
  /**
   * Indicates whether the current player wins by playing a given column.
   *
   * # Arguments
   *
   * * `col`: 0-based index of a playable column.
   *
   * # Returns
   *
   * True if the current player makes a 4-alignment by playing the column, false otherwise.
   */
  isWinningMove(col: number): boolean;
  /**
   * Indicates whether the current player can win with their next move.
   */
  canWinNext(): boolean;
  /**
   * Plays a move in the given column.
   *
   * # Arguments
   *
   * * `col`: 0-based index of a playable column.
   */
  play(col: number): void;
  /**
   * Returns a mask for the possible moves the current player can make.
   */
  possible(): bigint;
  /**
   * Returns a mask for the possible non-losing moves the current player can make.
   */
  possibleNonLosingMoves(): bigint;
  /**
   * Indicates whether the current position has been won by either player.
   */
  isWonPosition(): boolean;
  /**
   * Returns a mask for the entirety of the given column.
   *
   * # Arguments
   *
   * * `col`: 0-based index of a column.
   *
   * # Returns
   *
   * A bitmask with a one in all cells of the column.
   */
  static columnMask(col: number): bigint;
  static readonly WIDTH: number;
  static readonly HEIGHT: number;
  /**
   * A bitmask of the current player's tiles.
   */
  readonly position: bigint;
  /**
   * A bitmask of all occupied tiles.
   */
  readonly mask: bigint;
}
/**
 * A strong solver for finding the exact score of Connect Four positions.
 *
 * This class implements a high-performance negamax search algorithm with several
 * optimisations, including:
 * - Alpha-beta pruning
 * - Score-based move ordering to prioritise stronger moves
 * - A transposition table to cache results of previously seen positions
 * - A binary search on the score for faster convergence
 */
export class Solver {
  free(): void;
  /**
   * Creates a new `Solver` instance, using the pre-packaged opening book.
   */
  constructor();
  /**
   * Attempts to load an opening book from the given path.
   *
   * Returns whether the opening book was successfully loaded.
   */
  loadOpeningBook(path: string): boolean;
  /**
   * Resets the solver's state.
   */
  reset(): void;
  /**
   * Solves a position to find its exact score.
   *
   * This function uses a binary search over the possible score range, repeatedly calling the
   * negamax search with a null window to test if the score is above a certain value. This
   * allows faster convergence to the true score.
   *
   * Assumes that the given position is valid and not won by either player.
   *
   * # Arguments
   *
   * * `position`: The board position to solve.
   *
   * # Returns
   * The exact score of the position, which reflects the outcome of the game assuming that both
   * players play perfectly. A position has:
   * - A positive score if the current player will win. 1 if they win with their last move, 2 if
   *   they win with their second to last move, ...
   * - A null score if the game will end in a draw
   * - A negative score if the current player will lose. -1 if the opponent wins with their last
   *   move, -2 if the opponent wins with their second to last move, ...
   */
  solve(position: Position): number;
  /**
   * Calculates the scores for all possible next moves in the given position.
   *
   * Returns a fixed-size array where each index corresponds to a column, containing
   * a score if a move in that column is possible and `None` if the column is full and
   * the move is impossible.
   *
   * This array can be used to directly calculate the optimal move to play in a position.
   */
  getAllMoveScores(position: Position): any[];
  /**
   * A counter for the number of nodes explored in the last `solve` call.
   */
  readonly explored_positions: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_difficulty_free: (a: number, b: number) => void;
  readonly difficulty_EASY: () => number;
  readonly difficulty_MEDIUM: () => number;
  readonly difficulty_HARD: () => number;
  readonly difficulty_IMPOSSIBLE: () => number;
  readonly __wbg_aiplayer_free: (a: number, b: number) => void;
  readonly aiplayer_new: (a: number) => number;
  readonly aiplayer_loadOpeningBook: (a: number, b: number, c: number) => number;
  readonly aiplayer_reset: (a: number) => void;
  readonly aiplayer_solve: (a: number, b: number) => number;
  readonly aiplayer_getAllMoveScores: (a: number, b: number) => [number, number];
  readonly aiplayer_getMove: (a: number, b: number) => number;
  readonly aiplayer_selectMove: (a: number, b: number, c: number, d: number) => number;
  readonly __wbg_position_free: (a: number, b: number) => void;
  readonly position_WIDTH: () => number;
  readonly position_HEIGHT: () => number;
  readonly position_new: () => [number, number, number];
  readonly position_fromMoves: (a: number, b: number) => [number, number, number];
  readonly position_fromBoardString: (a: number, b: number) => [number, number, number];
  readonly position_toString: (a: number) => [number, number];
  readonly position_position: (a: number) => bigint;
  readonly position_mask: (a: number) => bigint;
  readonly position_getKey: (a: number) => bigint;
  readonly position_getMoves: (a: number) => number;
  readonly position_isPlayable: (a: number, b: number) => number;
  readonly position_isWinningMove: (a: number, b: number) => number;
  readonly position_canWinNext: (a: number) => number;
  readonly position_play: (a: number, b: number) => void;
  readonly position_possible: (a: number) => bigint;
  readonly position_possibleNonLosingMoves: (a: number) => bigint;
  readonly position_isWonPosition: (a: number) => number;
  readonly position_columnMask: (a: number) => bigint;
  readonly __wbg_solver_free: (a: number, b: number) => void;
  readonly solver_new: () => number;
  readonly solver_explored_positions: (a: number) => number;
  readonly solver_loadOpeningBook: (a: number, b: number, c: number) => number;
  readonly solver_reset: (a: number) => void;
  readonly solver_solve: (a: number, b: number) => number;
  readonly solver_getAllMoveScores: (a: number, b: number) => [number, number];
  readonly start: () => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_3: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
