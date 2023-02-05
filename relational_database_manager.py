### for managing the relational database of the project
### this is a work in progress

import sqlite3


### relational database class
class Database:
    def __init__(self, database_name):
        self.database_name = database_name
        self.connection = sqlite3.connect(self.database_name)
        self.cursor = self.connection.cursor()

    def create_table(self, table_name, table_columns):
        self.table_name = table_name
        self.table_columns = table_columns
        self.cursor.execute(f"CREATE TABLE IF NOT EXISTS {self.table_name} ({self.table_columns})")
        self.connection.commit()
        return f"Table {self.table_name} created with columns {self.table_columns}"
    
    def create_view(self, view_name, table_name, table_columns):
        self.view_name = view_name
        self.table_name = table_name
        self.table_columns = table_columns
        self.cursor.execute(f"CREATE VIEW {self.view_name} AS SELECT {self.table_columns} FROM {self.table_name}")
        self.connection.commit()
        return f"View {self.view_name} created with columns {self.table_columns} from table {self.table_name}"

    def insert_data(self, table_name, table_columns, data):
        self.table_name = table_name
        self.table_columns = table_columns
        self.data = data
        self.cursor.execute(f"INSERT INTO {self.table_name} ({self.table_columns}) VALUES ({self.data})")
        self.connection.commit()
        return f"Data {self.data} inserted into table {self.table_name} with columns {self.table_columns}"

    def select_data(self, table_name, table_columns, data):
        self.table_name = table_name
        self.table_columns = table_columns
        self.data = data
        self.cursor.execute(f"SELECT {self.table_columns} FROM {self.table_name} WHERE {self.data}")
        self.connection.commit()
        return self.cursor.fetchall()

    def update_data(self, table_name, table_columns, data, new_data):
        self.table_name = table_name
        self.table_columns = table_columns
        self.data = data
        self.new_data = new_data
        self.cursor.execute(f"UPDATE {self.table_name} SET {self.table_columns} = {self.new_data} WHERE {self.data}")
        self.connection.commit()
        return f"Data {self.data} updated to {self.new_data} in table {self.table_name} with columns {self.table_columns}"

    def delete_data(self, table_name, table_columns, data):
        self.table_name = table_name
        self.table_columns = table_columns
        self.data = data
        self.cursor.execute(f"DELETE FROM {self.table_name} WHERE {self.table_columns} = {self.data}")
        self.connection.commit()
        return f"Data {self.data} deleted from table {self.table_name} with columns {self.table_columns}"
    
    def delete_table(self, table_name):
        self.table_name = table_name
        self.cursor.execute(f"DROP TABLE {self.table_name}")
        self.connection.commit()
        return f"Table {self.table_name} deleted"

    def delete_database(self):
        self.cursor.execute(f"DROP DATABASE {self.database_name}")
        self.connection.commit()
        return f"Database {self.database_name} deleted"
    
    def delete_view(self, view_name):
        self.view_name = view_name
        self.cursor.execute(f"DROP VIEW {self.view_name}")
        self.connection.commit()
        return f"View {self.view_name} deleted"
    
    def list_tables(self):
        self.cursor.execute("SELECT name FROM sqlite_master WHERE type='table'")
        self.connection.commit()
        return self.cursor.fetchall()

    def list_views(self):
        self.cursor.execute("SELECT name FROM sqlite_master WHERE type='view'")
        self.connection.commit()
        return self.cursor.fetchall()
    
    def list_columns(self, table_name):
        self.table_name = table_name
        self.cursor.execute(f"PRAGMA table_info({self.table_name})")
        self.connection.commit()
        return self.cursor.fetchall()
    
    def list_data(self, table_name):
        self.table_name = table_name
        self.cursor.execute(f"SELECT * FROM {self.table_name}")
        self.connection.commit()
        return self.cursor.fetchall()
    
    def examine_view(self, view_name):
        self.view_name = view_name
        self.cursor.execute(f"SELECT * FROM {self.view_name}")
        self.connection.commit()
        return self.cursor.fetchall()
        
    def search_data(self, table_name, table_columns, data):
        self.table_name = table_name
        self.table_columns = table_columns
        self.data = data
        self.cursor.execute(f"SELECT * FROM {self.table_name} WHERE {self.table_columns} = {self.data}")
        self.connection.commit()
        return self.cursor.fetchall()

    def close_connection(self):
        self.connection.close()
        return "Connection closed"

    def __str__(self):
        return f"Database {self.database_name} created"