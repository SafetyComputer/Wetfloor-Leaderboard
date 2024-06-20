from datetime import datetime

from database import database
from elo import calculate_elo
from models import players, matches
from schemas import PlayerCreate, MatchCreate, PlayerPut


async def get_player(player_id: int):
    query = players.select().where(players.c.id == player_id)
    return await database.fetch_one(query)


async def get_player_by_name(player_name: str):
    query = players.select().where(players.c.name == player_name)
    return await database.fetch_one(query)


async def get_all_players():
    """Get all players with the order of their ELO rating."""
    query = players.select().order_by(players.c.elo.desc())
    return await database.fetch_all(query)


async def create_player(player: PlayerCreate):
    default_elo = player.default_elo or 1000
    query = players.insert().values(name=player.name, elo=default_elo, default_elo=default_elo)
    last_record_id = await database.execute(query)
    return {**player.dict(), "id": last_record_id, "elo": default_elo}


async def update_player(player_id: int, player: PlayerPut):
    update_data = player.dict(exclude_unset=True)
    query = players.update().where(players.c.id == player_id).values(**update_data)
    await database.execute(query)

    player = await get_player(player_id)
    return {**player, "id": player_id}


async def delete_player(player_id: int):
    query = players.delete().where(players.c.id == player_id)
    await database.execute(query)
    return {"message": "Player deleted successfully"}


async def get_match(match_id: int):
    query = matches.select().where(matches.c.id == match_id)
    return await database.fetch_one(query)


async def get_all_matches():
    query = matches.select().order_by(matches.c.date.desc())
    return await database.fetch_all(query)


async def create_match(match: MatchCreate):
    query = matches.insert().values(
        player1_id=match.player1_id,
        player2_id=match.player2_id,
        score1=match.score1,
        score2=match.score2,
        date=match.date or datetime.now()
    )
    last_record_id = await database.execute(query)

    # Check if the created match has the latest date
    latest_match_query = matches.select().order_by(matches.c.date.desc()).limit(1)
    latest_match = await database.fetch_one(latest_match_query)

    if latest_match['id'] == last_record_id:
        player1 = await get_player(match.player1_id)
        player2 = await get_player(match.player2_id)
        new_elo1, new_elo2 = calculate_elo(player1['elo'], player2['elo'], match.score1, match.score2)

        await update_player_elo(match.player1_id, new_elo1)
        await update_player_elo(match.player2_id, new_elo2)

    new_match = await get_match(last_record_id)

    return new_match


async def update_player_elo(player_id: int, new_elo: int):
    query = players.update().where(players.c.id == player_id).values(elo=new_elo)
    await database.execute(query)


async def update_match(match_id: int, match: MatchCreate):
    query = matches.update().where(matches.c.id == match_id).values(
        player1_id=match.player1_id,
        player2_id=match.player2_id,
        score1=match.score1,
        score2=match.score2,
        date=match.date
    )
    await database.execute(query)
    return {**match.dict(), "id": match_id}


async def delete_match(match_id: int):
    query = matches.delete().where(matches.c.id == match_id)
    await database.execute(query)
    return {"message": "Match deleted successfully"}


async def get_matches_by_player(player_id: int):
    query = matches.select().where(
        (matches.c.player1_id == player_id) | (matches.c.player2_id == player_id)
    )
    return await database.fetch_all(query)
