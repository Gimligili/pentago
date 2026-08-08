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

    @property
    def total_games(self) -> int:
        return self.wins + self.losses + self.draws

    @property
    def win_rate(self) -> float:
        if self.total_games == 0:
            return 0.0

        return self.wins / self.total_games

def evaluate_agent(
    neural_agent: NeuralAgent,
    num_games: int,
) -> EvaluationResult:
    wins = 0
    losses = 0
    draws = 0

    wins_as_white = 0
    wins_as_black = 0
    losses_as_white = 0
    losses_as_black = 0

    for game_index in range(num_games):
        neural_player = 1 if game_index % 2 == 0 else -1

        result = play_game(
            neural_agent=neural_agent,
            neural_player=neural_player,
        )

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
    )

def play_game(
    neural_agent: NeuralAgent,
    neural_player: int,
) -> int:
    game = pentago_engine.PyGame()

    while game.game_status() == 0:
        current_player = game.current_player()

        if current_player == neural_player:
            action = neural_agent.choose_action(game)
        else:
            action = game.teacher_action()

            if action is None:
                break

        game.step(action)

    return game.game_status()