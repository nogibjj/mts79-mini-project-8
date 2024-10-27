"""
Test suite for ETL-Query script.
"""

import subprocess


def test_extract():
    """Tests the extract() function."""
    result = subprocess.run(
        ["python", "main.py"],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0
    assert "Extracting data..." in result.stdout


def test_transform_load():
    """Tests the transform_load() function."""
    result = subprocess.run(
        ["python", "main.py"],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0
    assert "Transforming data..." in result.stdout


def test_create_record():
    """Tests the create_record() function."""
    result = subprocess.run(
        [
            "python",
            "main.py",
        ],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0
    assert "Querying data..." in result.stdout


def test_read_data():
    """Tests the read_data() function."""
    result = subprocess.run(
        ["python", "main.py"],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0


def test_update_record():
    """Tests the update_record() function."""
    result = subprocess.run(
        [
            "python",
            "main.py",
        ],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0


def test_delete_record():
    """Tests the delete_record() function."""
    result = subprocess.run(
        ["python", "main.py"],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0


if __name__ == "__main__":
    test_extract()
    test_transform_load()
    test_create_record()
    test_read_data()
    test_update_record()
    test_delete_record()