def calculate_elo(player1_elo, player2_elo, score1, score2, k=32):
    expected_score1 = 1 / (1 + 10 ** ((player2_elo - player1_elo) / 400))
    expected_score2 = 1 - expected_score1

    if score1 > score2:
        actual_score1 = 1
        actual_score2 = 0
    elif score1 < score2:
        actual_score1 = 0
        actual_score2 = 1
    else:
        actual_score1 = 0.5
        actual_score2 = 0.5

    new_player1_elo = player1_elo + k * (actual_score1 - expected_score1)
    new_player2_elo = player2_elo + k * (actual_score2 - expected_score2)

    return int(round(new_player1_elo)), int(round(new_player2_elo))
