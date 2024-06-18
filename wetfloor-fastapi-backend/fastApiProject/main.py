from typing import List

from fastapi import FastAPI, HTTPException

from crud import get_player, create_player, update_player, delete_player, get_match, create_match, update_match, \
    delete_match, get_matches_by_player
from database import database
from schemas import PlayerCreate, Player, MatchCreate, Match

app = FastAPI()


@app.on_event("startup")
async def startup():
    await database.connect()


@app.on_event("shutdown")
async def shutdown():
    await database.disconnect()


@app.post("/players/", response_model=Player)
async def create_new_player(player: PlayerCreate):
    return await create_player(player)


@app.get("/players/{player_id}", response_model=Player)
async def read_player(player_id: int):
    player = await get_player(player_id)
    if player is None:
        raise HTTPException(status_code=404, detail="Player not found")
    return player


@app.put("/players/{player_id}", response_model=Player)
async def update_existing_player(player_id: int, player: PlayerCreate):
    existing_player = await get_player(player_id)
    if existing_player is None:
        raise HTTPException(status_code=404, detail="Player not found")
    return await update_player(player_id, player)


@app.delete("/players/{player_id}")
async def delete_existing_player(player_id: int):
    existing_player = await get_player(player_id)
    if existing_player is None:
        raise HTTPException(status_code=404, detail="Player not found")
    return await delete_player(player_id)


@app.post("/matches/", response_model=Match)
async def create_new_match(match: MatchCreate):
    return await create_match(match)


@app.get("/matches/{match_id}", response_model=Match)
async def read_match(match_id: int):
    match = await get_match(match_id)
    if match is None:
        raise HTTPException(status_code=404, detail="Match not found")
    return match


@app.put("/matches/{match_id}", response_model=Match)
async def update_existing_match(match_id: int, match: MatchCreate):
    existing_match = await get_match(match_id)
    if existing_match is None:
        raise HTTPException(status_code=404, detail="Match not found")
    return await update_match(match_id, match)


@app.delete("/matches/{match_id}")
async def delete_existing_match(match_id: int):
    existing_match = await get_match(match_id)
    if existing_match is None:
        raise HTTPException(status_code=404, detail="Match not found")
    return await delete_match(match_id)


@app.get("/players/{player_id}/matches", response_model=List[Match])
async def read_matches_by_player(player_id: int):
    player = await get_player(player_id)
    if player is None:
        raise HTTPException(status_code=404, detail="Player not found")
    return await get_matches_by_player(player_id)
