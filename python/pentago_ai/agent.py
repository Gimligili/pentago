import torch

from pentago_ai.encoding import encode_board
from pentago_ai.model import PentagoNet
from pentago_ai.policy import action_probabilities


class NeuralAgent:
    def __init__(
        self,
        model: PentagoNet,
        stochastic: bool = True,
    ) -> None:
        self.model = model
        self.stochastic = stochastic

    def choose_action(self, game) -> int:
        legal_actions = game.legal_actions()

        if not legal_actions:
            raise RuntimeError("No legal actions available")

        state = encode_board(game).unsqueeze(0)

        self.model.eval()

        with torch.no_grad():
            logits, _ = self.model(state)
            logits = logits.squeeze(0)

        probabilities = action_probabilities(
            logits,
            legal_actions,
        )

        if self.stochastic:
            action = torch.multinomial(
                probabilities,
                num_samples=1,
            )
            return int(action.item())

        return int(torch.argmax(probabilities).item())