import pytest
from fastapi.testclient import TestClient
import pytest
from database import database
from main import app

client = TestClient(app)


@pytest.fixture(scope="module", autouse=True)
async def setup_and_teardown():
    # Setup: connect to the database
    await database.connect()
    yield
    # Teardown: disconnect from the database
    await database.disconnect()


def test_create_player():
    response = client.post("/players/", json={"name": "John Doe", "default_elo": 1200})
    assert response.status_code == 200
    data = response.json()
    assert data["name"] == "John Doe"
    assert data["elo"] == 1200
    assert data["default_elo"] == 1200


def test_read_player():
    # First create a player
    response = client.post("/players/", json={"name": "Jane Doe", "default_elo": 1300})
    player_id = response.json()["id"]

    # Then read the player
    response = client.get(f"/players/{player_id}")
    assert response.status_code == 200
    data = response.json()
    assert data["name"] == "Jane Doe"
    assert data["elo"] == 1300


def test_update_player():
    # First create a player
    response = client.post("/players/", json={"name": "John Smith", "default_elo": 1250})
    player_id = response.json()["id"]

    # Then update the player
    response = client.put(f"/players/{player_id}", json={"name": "John Smith", "default_elo": 1350})
    assert response.status_code == 200
    data = response.json()
    assert data["default_elo"] == 1350


def test_delete_player():
    # First create a player
    response = client.post("/players/", json={"name": "Alice", "default_elo": 1400})
    player_id = response.json()["id"]

    # Then delete the player
    response = client.delete(f"/players/{player_id}")
    assert response.status_code == 200
    assert response.json()["message"] == "Player deleted successfully"


def test_create_match_and_update_elo():
    # First create two players
    player1 = client.post("/players/", json={"name": "Player One", "default_elo": 1100})
    player2 = client.post("/players/", json={"name": "Player Two", "default_elo": 1150})
    player1_id = player1.json()["id"]
    player2_id = player2.json()["id"]

    # Then create a match
    response = client.post("/matches/", json={
        "player1_id": player1_id,
        "player2_id": player2_id,
        "score1": 3,
        "score2": 2,
        "date": "2024-06-18T12:00:00"
    })
    assert response.status_code == 200
    match = response.json()
    assert match["player1_id"] == player1_id
    assert match["player2_id"] == player2_id

    # Check updated ELOs
    player1_updated = client.get(f"/players/{player1_id}")
    player2_updated = client.get(f"/players/{player2_id}")
    assert player1_updated.status_code == 200
    assert player2_updated.status_code == 200

    player1_elo = player1_updated.json()["elo"]
    player2_elo = player2_updated.json()["elo"]

    assert player1_elo != 1100
    assert player2_elo != 1150


def test_read_match():
    # First create two players
    player1 = client.post("/players/", json={"name": "Player One", "default_elo": 1100})
    player2 = client.post("/players/", json={"name": "Player Two", "default_elo": 1150})
    player1_id = player1.json()["id"]
    player2_id = player2.json()["id"]

    # Then create a match
    match = client.post("/matches/", json={
        "player1_id": player1_id,
        "player2_id": player2_id,
        "score1": 3,
        "score2": 2,
        "date": "2024-06-18T12:00:00"
    })
    match_id = match.json()["id"]

    # Then read the match
    response = client.get(f"/matches/{match_id}")
    assert response.status_code == 200
    data = response.json()
    assert data["player1_id"] == player1_id
    assert data["player2_id"] == player2_id


def test_update_match():
    # First create two players
    player1 = client.post("/players/", json={"name": "Player One", "default_elo": 1100})
    player2 = client.post("/players/", json={"name": "Player Two", "default_elo": 1150})
    player1_id = player1.json()["id"]
    player2_id = player2.json()["id"]

    # Then create a match
    match = client.post("/matches/", json={
        "player1_id": player1_id,
        "player2_id": player2_id,
        "score1": 3,
        "score2": 2,
        "date": "2024-06-18T12:00:00"
    })
    match_id = match.json()["id"]

    # Then update the match
    response = client.put(f"/matches/{match_id}", json={
        "player1_id": player1_id,
        "player2_id": player2_id,
        "score1": 2,
        "score2": 3,
        "date": "2024-06-18T12:00:00"
    })
    assert response.status_code == 200
    updated_match = response.json()
    assert updated_match["score1"] == 2
    assert updated_match["score2"] == 3


def test_delete_match():
    # First create two players
    player1 = client.post("/players/", json={"name": "Player One", "default_elo": 1100})
    player2 = client.post("/players/", json={"name": "Player Two", "default_elo": 1150})
    player1_id = player1.json()["id"]
    player2_id = player2.json()["id"]

    # Then create a match
    match = client.post("/matches/", json={
        "player1_id": player1_id,
        "player2_id": player2_id,
        "score1": 3,
        "score2": 2,
        "date": "2024-06-18T12:00:00"
    })
    match_id = match.json()["id"]

    # Then delete the match
    response = client.delete(f"/matches/{match_id}")
    assert response.status_code == 200
    assert response.json()["message"] == "Match deleted successfully"


def test_get_matches_by_player():
    # First create two players
    player1 = client.post("/players/", json={"name": "Player One", "default_elo": 1100})
    player2 = client.post("/players/", json={"name": "Player Two", "default_elo": 1150})
    player1_id = player1.json()["id"]
    player2_id = player2.json()["id"]

    # Create multiple matches
    client.post("/matches/", json={
        "player1_id": player1_id,
        "player2_id": player2_id,
        "score1": 3,
        "score2": 2,
        "date": "2024-06-18T12:00:00"
    })
    client.post("/matches/", json={
        "player1_id": player2_id,
        "player2_id": player1_id,
        "score1": 1,
        "score2": 4,
        "date": "2024-06-19T12:00:00"
    })

    # Get matches by player
    response = client.get(f"/players/{player1_id}/matches")
    assert response.status_code == 200
    matches = response.json()
    assert len(matches) == 2
