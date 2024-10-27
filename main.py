"""
ETL-Query script
"""

from mylib.extract import extract
from mylib.transform_load import load
from mylib.query import create_record, read_data, update_record, delete_record

# Extract
print("Extracting data...")
extract()

# Transform and load
print("Transforming data...")
load()

# Query
print("Querying data...")
create_record(1999, 11, 17, 3, 77777)

# Read the first 10 rows of the data
data = read_data()
print(data)

# Update a record
update_record(7, 2000, 1, 1, 6, 9100)

# Delete a record
delete_record(8)