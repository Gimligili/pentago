import random
from dataclasses import dataclass

from pentago_ai import pentago_engine
from pentago_ai.agent import NeuralAgent


@dataclass
class EvaluationResult:
    wins: int
    losses: int
    draws: int

    wins_as_white: int
    wins_as_black: int
    losses_as_white: int
    losses_as_black: int

    compared_actions: int
    matching_actions: int
    first_divergence_moves: list[int]

    @property
    def total_games(self) -> int:
        return self.wins + self.losses + self.draws

    @property
    def win_rate(self) -> float:
        if self.total_games == 0:
            return 0.0

        return self.wins / self.total_games

    @property
    def teacher_agreement(self) -> float:
        if self.compared_actions == 0:
            return 0.0

        return self.matching_actions / self.compared_actions

def evaluate_agent(
    neural_agent: NeuralAgent,
    num_games: int,
    seed: int = 42,
) -> EvaluationResult:
    random_src = random.Random(seed)

    wins = 0
    losses = 0
    draws = 0

    wins_as_white = 0
    wins_as_black = 0
    losses_as_white = 0
    losses_as_black = 0

    compared_actions = 0
    matching_actions = 0
    first_divergence_moves = []

    for game_index in range(num_games):
        neural_player = 1 if game_index % 2 == 0 else -1

        opening_moves = random_src.randint(2, 6)

        (
            result,
            game_compared_actions,
            game_matching_actions,
            first_divergence_move,
        ) = play_game(
            neural_agent=neural_agent,
            neural_player=neural_player,
            opening_moves=opening_moves,
            rng=random_src,
        )

        compared_actions += game_compared_actions
        matching_actions += game_matching_actions

        if first_divergence_move is not None:
            first_divergence_moves.append(first_divergence_move)

        if result == 2:
            draws += 1

        elif result == neural_player:
            wins += 1

            if neural_player == 1:
                wins_as_white += 1
            else:
                wins_as_black += 1

        else:
            losses += 1

            if neural_player == 1:
                losses_as_white += 1
            else:
                losses_as_black += 1

    return EvaluationResult(
        wins=wins,
        losses=losses,
        draws=draws,
        wins_as_white=wins_as_white,
        wins_as_black=wins_as_black,
        losses_as_white=losses_as_white,
        losses_as_black=losses_as_black,
        compared_actions=compared_actions,
        matching_actions=matching_actions,
        first_divergence_moves=first_divergence_moves,
    )

def play_game(
    neural_agent: NeuralAgent,
    neural_player: int,
    opening_moves: int = 0,
    rng: random.Random | None = None,
) -> tuple[int, int, int, int | None]:
    if rng is None:
        rng = random.Random()

    game = pentago_engine.PyGame()

    for _ in range(opening_moves):
        if game.game_status() != 0:
            return game.game_status(), 0, 0, None

        legal_actions = game.legal_actions()

        if not legal_actions:
            return game.game_status(), 0, 0, None

        game.step(rng.choice(legal_actions))

    compared_actions = 0
    matching_actions = 0
    first_divergence_move = None
    move_index = 0

    while game.game_status() == 0:
        move_index += 1

        teacher_action = game.teacher_action()
        neural_action = neural_agent.choose_action(game)

        if teacher_action is None:
            break

        compared_actions += 1

        if teacher_action == neural_action:
            matching_actions += 1
        elif first_divergence_move is None:
            first_divergence_move = move_index

        if game.current_player() == neural_player:
            action = neural_action
        else:
            action = teacher_action

        game.step(action)

    return (
        game.game_status(),
        compared_actions,
        matching_actions,
        first_divergence_move,
    )