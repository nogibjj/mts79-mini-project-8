"""
Extract a dataset from a URL like Kaggle or data.gov. 
JSON or CSV formats tend to work well.

Births dataset.
"""
import os
import requests

def extract(
    url="https://github.com/fivethirtyeight/data/raw/refs/heads/master/births/"
        "US_births_2000-2014_SSA.csv",
    file_path="data/births.csv"
):
    """Extract a URL to a file path."""
    # Create the directory if it doesn't exist
    os.makedirs(os.path.dirname(file_path), exist_ok=True)
    
    with requests.get(url) as r:
        with open(file_path, 'wb') as f:
            f.write(r.content)
    return file_path