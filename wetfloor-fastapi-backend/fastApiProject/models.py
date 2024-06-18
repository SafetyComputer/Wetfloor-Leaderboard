from sqlalchemy import Table, Column, Integer, String, ForeignKey, DateTime, func
from database import metadata

players = Table(
    "players",
    metadata,
    Column("id", Integer, primary_key=True, index=True),
    Column("name", String(length=50), unique=True, index=True),
    Column("elo", Integer, default=1000),
    Column("default_elo", Integer, default=1000)
)

matches = Table(
    "matches",
    metadata,
    Column("id", Integer, primary_key=True, index=True),
    Column("player1_id", Integer, ForeignKey("players.id")),
    Column("player2_id", Integer, ForeignKey("players.id")),
    Column("score1", Integer),
    Column("score2", Integer),
    Column("date", DateTime, default=func.now())
)
