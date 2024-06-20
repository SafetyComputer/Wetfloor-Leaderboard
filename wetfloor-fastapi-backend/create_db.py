from sqlalchemy import create_engine

from database import DATABASE_URL
from models import metadata


def create_db():
    engine = create_engine(DATABASE_URL)
    metadata.create_all(engine)
    print("Database and tables created.")


if __name__ == "__main__":
    create_db()
