from databases import Database
from sqlalchemy import create_engine, MetaData

DATABASE_URL = "mysql+mysqlconnector://root:Kevin42dsz@127.0.0.1:3306/wetfloor"

database = Database(DATABASE_URL)
metadata = MetaData()

engine = create_engine(DATABASE_URL)
metadata.create_all(engine)
