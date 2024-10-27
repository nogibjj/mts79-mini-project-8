```sql
INSERT INTO USBirths VALUES (
            1999, 
            11, 
            17, 
            3, 
            77777);
```

```sql
SELECT * FROM USBirths LIMIT 10;
```

```sql
UPDATE USBirths SET 
        year=2000, 
        month=1, 
        date_of_month=1, 
        day_of_week=6, 
        births=9100 
        WHERE rowid=7;
```

```sql
DELETE FROM USBirths WHERE rowid=8;
```

