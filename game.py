import random
import time
import typing


def get_input(
    display_text: str,
    check: typing.Callable[[str], bool],
    error_text: str = "Invalid input. Please try again.",
) -> str:
    """Gets input from the user"""

    valid = False

    while not valid:
        inp = input(display_text)
        if check(inp):
            return inp
        else:
            print(error_text)
            continue


def get_players() -> dict[str, int]:
    """Gets inputs for the names of players"""

    # players will be stored in a dict with key: `name` and value: `score`
    players = {}

    # a list of allowed players
    with open("allowed_names.txt") as f:
        allowed_players = f.read().splitlines()

    # value to check if there is another player to add to the game
    another_player = True

    # makes sure there are at least 2 players (no-one wants to be alone)
    # and if another player needs to be added
    while len(players) < 2 or another_player:

        # get input of player name
        name = get_input(
            f"Enter the name of player {len(players) + 1}: ",
            lambda x: x.lower() in allowed_players,
            "Invalid name. Please try again.",
        )

        # set player score to 0
        players[name] = 0

        # if there are less than 2 players another player must be added
        if len(players) >= 2:
            # ask players if they want to add another player
            # returns boolean: True if yes, False if no
            another_player = (
                get_input(
                    "Do you want to add another player? (y/n) ",
                    lambda x: x.lower() in ["y", "yes", "n", "no"],
                )
                in ["y", "yes"]
            )

    print()

    return players


def check_win(players: dict[str, int]) -> bool:
    """Checks if there is a winner to the game"""

    winners = {}

    # loop through player names and scores
    for name, score in players.items():
        # if score has reached win condition (>10 points)
        if score > 10:
            winners[name] = score

    # if there is no winner return False
    if len(winners) == 0:
        return False

    elif len(winners) == 1:
        # if there is only one winner return the winner
        print(
            f"Game over! {list(winners.keys())[0]} is the winner!"
        )  # gets the name of the winner
        return True

    else:
        # sort winners by score in descending order
        winners = dict(sorted(winners.items(), key=lambda x: x[1], reverse=True))

        # get a list of scores & names
        scores = list(winners.values())
        names = list(winners.keys())

        if scores[0] == scores[1]:
            # there is a draw
            print(f"Game over! It's a draw between {names[0]} and {names[1]}!")
            return True

        else:
            # there is a winner
            print(
                f"Game over! {names[0]} is the winner!"
            )  # gets the name of the winner (player with highest score)
            return True


def player_rolls(players: dict[str, int]) -> dict[str, int]:
    """Rolls the dice for each player"""

    # loop through players
    for name, score in players.items():
        # roll the dice
        roll = random.randint(1, 6)

        # calculate value to change score by
        # roll of 1 will deduct points, 2 will leave points unchanged
        # 3, 4, 5 or 6 will add points
        change = roll - 2

        # print the roll
        print(f"{name} rolled a {roll}, their score will change by {change}!")

        # change the score
        players[name] += change

    return players


def display_scores(players: dict[str, int]) -> None:
    """Displays the scores of all players"""

    # loop through players
    for name, score in players.items():
        print(f"{name}'s score is {score}")

    print()


def game() -> bool:
    """Runs the game"""

    # get players input
    # returns a dict with key: `name` and value: `score`
    players = get_players()

    win = False

    # loop until there is a winner
    while not win:

        # roll the dice
        players = player_rolls(players)

        # display scores
        display_scores(players)

        # check if there is a winner
        # returns boolean: True if there is a winner, False if not
        win = check_win(players)

        # wait 1 second before next round
        time.sleep(1)

    # returns boolean value: True to play again, False to quit
    return (
        get_input(
            "Do you want to play again? (y/n) ",
            lambda x: x.lower() in ["y", "yes", "n", "no"],
        )
        in ["y", "yes"]
    )


def main() -> None:
    """Starts the game & controls game repeats"""

    play = True

    # loop until player wants to quit
    while play:

        # game() returns a boolean based on whether players want to continue
        play = game()

        print()

    print("Thanks for playing!")


main()
