from datetime import datetime
from typing import Optional

from pydantic import BaseModel


class PlayerBase(BaseModel):
    name: str


class PlayerCreate(PlayerBase):
    default_elo: Optional[int] = 1000


class PlayerPut(BaseModel):
    name: str | None = None
    elo: int | None = None


class Player(PlayerBase):
    id: int
    elo: int

    class Config:
        orm_mode = True


class MatchBase(BaseModel):
    player1_id: int
    player2_id: int
    score1: int
    score2: int
    date: Optional[datetime] = None


class MatchCreate(MatchBase):
    pass


class Match(MatchBase):
    id: int

    class Config:
        orm_mode = True
