from pathlib import Path

from pentago_ai.agent import NeuralAgent
from pentago_ai.evaluation import evaluate_agent
from pentago_ai.model import load_model

MODEL_PATH = Path("models") / "pentago_policy.pt"
NUM_GAMES = 100


def main() -> None:
    print(f"Loading model from {MODEL_PATH}...")

    model = load_model(MODEL_PATH)

    agent = NeuralAgent(
        model,
        stochastic=False,
    )

    print(f"Playing {NUM_GAMES} games against the Rust teacher...")

    result = evaluate_agent(
        neural_agent=agent,
        num_games=NUM_GAMES,
    )

    print()
    print("Evaluation results")
    print(f"Wins:   {result.wins}")
    print(f"Losses: {result.losses}")
    print(f"Draws:  {result.draws}")
    print(f"Win rate: {result.win_rate * 100:.2f}%")


if __name__ == "__main__":
    main()